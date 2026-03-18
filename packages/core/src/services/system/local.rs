use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use super::SystemService;
use crate::{api::ApiError, dtos::LocalResponse, entities::local};

impl SystemService {
  pub async fn local_list(&self) -> Result<Vec<LocalResponse>, ApiError> {
    let rows = local::Entity::find()
      .all(self.db.as_ref())
      .await
      .map_err(ApiError::Database)?;

    Ok(rows.iter().map(LocalResponse::from).collect())
  }

  pub async fn local_get(&self, id: i32) -> Result<LocalResponse, ApiError> {
    let row = local::Entity::find_by_id(id)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Local row '{}' not found", id)))?;

    Ok((&row).into())
  }

  pub async fn local_query(
    &self,
    is_initialized: Option<bool>,
  ) -> Result<Vec<LocalResponse>, ApiError> {
    let mut condition = Condition::all();

    if let Some(is_initialized) = is_initialized {
      condition = condition.add(local::Column::IsInitialized.eq(is_initialized));
    }

    let rows = local::Entity::find()
      .filter(condition)
      .all(self.db.as_ref())
      .await
      .map_err(ApiError::Database)?;

    Ok(rows.iter().map(LocalResponse::from).collect())
  }
}
