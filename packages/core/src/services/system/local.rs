use sea_orm::{
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  Condition,
  ConnectionTrait,
  EntityLoaderTrait,
  QueryFilter,
};

use super::SystemService;
use crate::{api::ApiError, dtos::LocalResponse, entities::local};

impl SystemService {
  pub async fn local_list(&self) -> Result<Vec<LocalResponse>, ApiError> {
    let rows: Vec<local::ModelEx> = local::Entity::load()
      .all(self.db.as_ref())
      .await
      .map_err(ApiError::Database)?;

    Ok(rows.iter().map(LocalResponse::from).collect())
  }

  pub async fn local_get(&self, id: i32) -> Result<LocalResponse, ApiError> {
    let row: local::ModelEx = local::Entity::load()
      .filter_by_id(id)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Local row '{}' not found", id)))?;

    Ok((&row).into())
  }

  pub async fn update_central_api_url(&self, url: &str) -> Result<LocalResponse, ApiError> {
    let current = load_local_bootstrap(self.db.as_ref()).await?;
    let mut model: local::ActiveModel = current.into();
    model.central_api_url = Set(Some(url.to_string()));
    let updated = model.update(self.db.as_ref()).await?;
    Ok(LocalResponse::from(&updated))
  }

  pub async fn local_query(
    &self,
    is_initialized: Option<bool>,
  ) -> Result<Vec<LocalResponse>, ApiError> {
    let mut condition = Condition::all();

    if let Some(is_initialized) = is_initialized {
      condition = condition.add(local::Column::IsInitialized.eq(is_initialized));
    }

    let rows: Vec<local::ModelEx> = local::Entity::load()
      .filter(condition)
      .all(self.db.as_ref())
      .await
      .map_err(ApiError::Database)?;

    Ok(rows.iter().map(LocalResponse::from).collect())
  }
}

pub async fn load_local_bootstrap(conn: &impl ConnectionTrait) -> Result<local::Model, ApiError> {
  let row = local::Entity::load()
    .filter_by_id(1)
    .one(conn)
    .await
    .map_err(ApiError::Database)?;

  row
    .map(|row| local::Model {
      id: row.id,
      is_initialized: row.is_initialized,
      local_db_id: row.local_db_id,
      jwt_secret: row.jwt_secret,
      central_api_url: row.central_api_url,
    })
    .ok_or_else(|| ApiError::NotFound("Local bootstrap row is missing".to_string()))
}
