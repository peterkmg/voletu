use sea_orm::{ColumnTrait, Condition, QueryFilter, QueryOrder};
use uuid::Uuid;

use super::super::SystemService;
use crate::{
  api::ApiError,
  dtos::UserResponse,
  entities::{role, user},
};

impl SystemService {
  pub async fn user_list(&self) -> Result<Vec<UserResponse>, ApiError> {
    tracing::debug!("Listing all users");
    let local_db_id = self.user_local_db_id().await?;

    let users: Vec<user::ModelEx> = user::Entity::load()
      .filter(user::Column::OriginDbId.eq(local_db_id))
      .filter(user::Column::DeletedAt.is_null())
      .with(role::Entity)
      .all(self.db.as_ref())
      .await?;

    tracing::debug!("Found {} user(s)", users.len());

    users
      .iter()
      .map(UserResponse::try_from)
      .collect::<anyhow::Result<Vec<_>>>()
      .map_err(ApiError::Internal)
  }

  pub async fn user_get(&self, id: Uuid) -> Result<UserResponse, ApiError> {
    let local_db_id = self.user_local_db_id().await?;

    let user = user::Entity::load()
      .filter(user::Column::Id.eq(id))
      .filter(user::Column::OriginDbId.eq(local_db_id))
      .filter(user::Column::DeletedAt.is_null())
      .with(role::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("User '{}' not found", id)))?;

    UserResponse::try_from(&user).map_err(ApiError::Internal)
  }

  pub async fn user_query(
    &self,
    username: Option<&str>,
    role_id: Option<Uuid>,
  ) -> Result<Vec<UserResponse>, ApiError> {
    let local_db_id = self.user_local_db_id().await?;
    let mut condition = Condition::all();
    condition = condition
      .add(user::Column::OriginDbId.eq(local_db_id))
      .add(user::Column::DeletedAt.is_null());

    if let Some(username) = username {
      condition = condition.add(user::Column::Username.contains(username));
    }

    if let Some(role_id) = role_id {
      condition = condition.add(user::Column::RoleId.eq(role_id));
    }

    let users: Vec<user::ModelEx> = user::Entity::load()
      .filter(condition)
      .with(role::Entity)
      .order_by_asc(user::Column::Username)
      .all(self.db.as_ref())
      .await?;

    users
      .iter()
      .map(UserResponse::try_from)
      .collect::<anyhow::Result<Vec<_>>>()
      .map_err(ApiError::Internal)
  }
}
