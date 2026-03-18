use std::str::FromStr;

use anyhow::anyhow;
use sea_orm::{
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  EntityLoaderTrait,
  EntityTrait,
  QueryFilter,
};
use uuid::Uuid;

use super::super::SystemService;
use crate::{
  api::ApiError,
  dtos::{UpdateUserRequest, UserResponse},
  entities::{role, user},
  enums::RoleType,
  utils::password::hash_password,
};

impl SystemService {
  pub async fn user_update(
    &self,
    id: Uuid,
    dto: &UpdateUserRequest,
  ) -> Result<UserResponse, ApiError> {
    let local_db_id = self.user_local_db_id().await?;

    let existing = user::Entity::find()
      .filter(user::Column::Id.eq(id))
      .filter(user::Column::OriginDbId.eq(local_db_id))
      .filter(user::Column::DeletedAt.is_null())
      .one(self.db.as_ref())
      .await?;

    let Some(existing) = existing else {
      return Err(ApiError::NotFound(format!("User '{}' not found", id)));
    };

    let mut model: user::ActiveModel = existing.clone().into();

    if let Some(username) = &dto.username {
      if username != &existing.username {
        let duplicate = user::Entity::find()
          .filter(user::Column::Username.eq(username))
          .filter(user::Column::OriginDbId.eq(local_db_id))
          .filter(user::Column::DeletedAt.is_null())
          .one(self.db.as_ref())
          .await?;

        if let Some(duplicate) = duplicate {
          if duplicate.id != id {
            return Err(ApiError::Conflict(format!(
              "Username '{}' is already taken",
              username
            )));
          }
        }
      }

      model.username = Set(username.clone());
    }

    if let Some(fullname) = &dto.fullname {
      model.fullname = Set(Some(fullname.clone()));
    }

    if let Some(role_name) = &dto.role_name {
      let req_role = RoleType::from_str(role_name)
        .map_err(|_| ApiError::BadRequest(format!("Invalid role name '{}'", role_name)))?;

      let role: role::ModelEx = role::Entity::load()
        .filter_by_common_name(req_role)
        .one(self.db.as_ref())
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Role '{}' not found", role_name)))?;

      model.role_id = Set(role.id);
    }

    if let Some(password) = &dto.password {
      let password_hash = hash_password(password).await.map_err(ApiError::Internal)?;
      model.password_hash = Set(password_hash);
    }
    let updated = model.update(self.db.as_ref()).await?;

    let user_ex = user::Entity::load()
      .filter_by_id(updated.id)
      .with(role::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::Internal(anyhow!("User not found after update")))?;

    UserResponse::try_from(&user_ex).map_err(ApiError::Internal)
  }
}
