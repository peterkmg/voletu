use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use super::SystemService;
use crate::{api::ApiError, dtos::RoleResponse, entities::role};

impl SystemService {
  pub async fn role_list(&self) -> Result<Vec<RoleResponse>, ApiError> {
    let rows = role::Entity::find()
      .order_by_asc(role::Column::CommonName)
      .all(self.db.as_ref())
      .await?;

    Ok(rows.iter().map(RoleResponse::from).collect())
  }

  pub async fn role_get(&self, id: Uuid) -> Result<RoleResponse, ApiError> {
    let row = role::Entity::find_by_id(id)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Role '{}' not found", id)))?;

    Ok((&row).into())
  }

  pub async fn role_query(
    &self,
    common_name: Option<crate::enums::RoleType>,
  ) -> Result<Vec<RoleResponse>, ApiError> {
    let mut query = role::Entity::find();

    if let Some(common_name) = common_name {
      query = query.filter(role::Column::CommonName.eq(common_name));
    }

    let rows = query
      .order_by_asc(role::Column::CommonName)
      .all(self.db.as_ref())
      .await
      .map_err(ApiError::Database)?;

    Ok(rows.iter().map(RoleResponse::from).collect())
  }
}
