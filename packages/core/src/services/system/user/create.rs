use std::str::FromStr;

use anyhow::anyhow;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityLoaderTrait, QueryFilter};

use super::super::SystemService;
use crate::{
  api::ApiError,
  context::audit::current_actor_id,
  dtos::{CreateUserRequest, UserResponse},
  entities::{role, user},
  enums::RoleType,
  utils::password::hash_password,
};

impl SystemService {
  pub async fn user_create(&self, dto: &CreateUserRequest) -> Result<UserResponse, ApiError> {
    tracing::info!(username = %dto.username, "Creating user");
    let local_db_id = self.user_local_db_id().await?;

    let existing: Option<user::ModelEx> = user::Entity::load()
      .filter(user::Column::Username.eq(&dto.username))
      .filter(user::Column::OriginDbId.eq(local_db_id))
      .filter(user::Column::DeletedAt.is_null())
      .one(self.db.as_ref())
      .await?;

    if existing.is_some() {
      tracing::warn!(username = %dto.username, "Username already taken");
      return Err(ApiError::Conflict(format!(
        "Username '{}' is already taken",
        dto.username
      )));
    }

    let req_role = RoleType::from_str(&dto.role_name).map_err(|_| {
      tracing::warn!(role_name = %dto.role_name, "Invalid role name provided");
      ApiError::BadRequest(format!("Invalid role name '{}'", dto.role_name))
    })?;

    let role: role::ModelEx = role::Entity::load()
      .filter_by_common_name(req_role)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| {
        tracing::warn!(role_name = %dto.role_name, "Role not found");
        ApiError::NotFound(format!("Role '{}' not found", dto.role_name))
      })?;

    let password_hash = hash_password(&dto.password)
      .await
      .map_err(ApiError::Internal)?;
    let actor_id = current_actor_id()
      .ok_or_else(|| ApiError::Unauthorized("Missing authenticated actor context".to_string()))?;

    let active_model = user::ActiveModel {
      username: Set(dto.username.clone()),
      fullname: Set(dto.fullname.clone()),
      password_hash: Set(password_hash),
      role_id: Set(role.id),
      created_by: Set(actor_id),
      updated_by: Set(actor_id),
      ..Default::default()
    };

    let inserted = active_model.insert(self.db.as_ref()).await?;

    tracing::info!(id = %inserted.id, username = %inserted.username, "User created");

    let user_ex = user::Entity::load()
      .filter_by_id(inserted.id)
      .with(role::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::Internal(anyhow!("User not found after creation")))?;

    UserResponse::try_from(&user_ex).map_err(ApiError::Internal)
  }
}
