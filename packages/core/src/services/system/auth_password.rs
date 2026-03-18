use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use super::SystemService;
use crate::{
  api::ApiError,
  context::audit::current_actor_id,
  dtos::ChangePasswordRequest,
  entities::user,
  utils::password::{hash_password, verify_password},
};

impl SystemService {
  pub async fn change_password(&self, dto: &ChangePasswordRequest) -> Result<(), ApiError> {
    let local_db_id = self.local_db_id().await?;
    let user = user::Entity::find()
      .filter(user::Column::Username.eq(&dto.username))
      .filter(user::Column::OriginDbId.eq(local_db_id))
      .filter(user::Column::DeletedAt.is_null())
      .one(self.db.as_ref())
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
    let mut user_model: user::ActiveModel = user.into();
    user_model.password_hash = Set(
      hash_password(&dto.new_password)
        .await
        .map_err(ApiError::Internal)?,
    );
    user_model.updated_by = Set(actor_id);
    user_model.update(self.db.as_ref()).await?;

    self.revoke_user_refresh_tokens(user_id).await?;

    Ok(())
  }
}
