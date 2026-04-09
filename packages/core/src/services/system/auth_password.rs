use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use super::SystemService;
use crate::{
  api::ApiError,
  context::audit::current_actor_id,
  dtos::ChangePasswordRequest,
  entities::user,
  services::system::user::helpers::load_local_active_user_by_username,
  utils::password::{hash_password, verify_password},
};

impl SystemService {
  pub async fn change_password(&self, dto: &ChangePasswordRequest) -> Result<(), ApiError> {
    let local_db_id = self.local_db_id().await?;
    let user = load_local_active_user_by_username(self.db.as_ref(), local_db_id, &dto.username)
      .await?
      .ok_or(ApiError::Unauthorized("Invalid credentials".to_string()))?;

    let is_valid = verify_password(&dto.current_password, &user.password_hash)
      .await
      .map_err(|_| ApiError::Unauthorized("Invalid credentials".to_string()))?;

    if !is_valid {
      return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
    }

    let actor_id = current_actor_id()
      .ok_or_else(|| ApiError::Unauthorized("Missing authenticated actor context".to_string()))?;

    let user_id = user.id;
    user::ActiveModel {
      id: Set(user.id),
      password_hash: Set(
        hash_password(&dto.new_password)
          .await
          .map_err(ApiError::Internal)?,
      ),
      updated_by: Set(actor_id),
      ..Default::default()
    }
    .update(self.db.as_ref())
    .await?;

    self.revoke_user_refresh_tokens(user_id).await?;

    Ok(())
  }
}
