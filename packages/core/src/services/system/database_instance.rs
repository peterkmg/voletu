use sea_orm::{
  ActiveModelTrait,
  ActiveValue::{NotSet, Set},
  ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter,
};
use uuid::Uuid;

use super::SystemService;
use crate::{
  api::ApiError, dtos::DatabaseInstanceResponse, entities::database_instance, enums::NodeType,
};

impl SystemService {
  pub async fn database_instance_list(&self) -> Result<Vec<DatabaseInstanceResponse>, ApiError> {
    let rows: Vec<database_instance::ModelEx> = database_instance::Entity::load()
      .filter(database_instance::Column::DeletedAt.is_null())
      .all(self.db.as_ref())
      .await
      .map_err(ApiError::Database)?;

    Ok(rows.iter().map(DatabaseInstanceResponse::from).collect())
  }

  pub async fn database_instance_get(
    &self,
    id: Uuid,
  ) -> Result<DatabaseInstanceResponse, ApiError> {
    let instance = database_instance::Entity::load()
      .filter_by_id(id)
      .filter(database_instance::Column::DeletedAt.is_null())
      .one(self.db.as_ref())
      .await?;

    let row = instance
      .ok_or_else(|| ApiError::NotFound(format!("Database instance '{}' not found", id)))?;
    Ok((&row).into())
  }

  pub async fn database_instance_update(
    &self,
    id: Uuid,
    common_name: String,
    node_type: NodeType,
    base_id: Option<Uuid>,
  ) -> Result<DatabaseInstanceResponse, ApiError> {
    let row = database_instance::Entity::load()
      .filter_by_id(id)
      .filter(database_instance::Column::DeletedAt.is_null())
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Database instance '{}' not found", id)))?;

    let saved = database_instance::ActiveModel {
      id: Set(row.id),
      common_name: Set(common_name),
      node_type: Set(node_type),
      base_id: Set(base_id),
      updated_by: NotSet,
      ..Default::default()
    }
    .update(self.db.as_ref())
    .await
    .map_err(ApiError::Database)?;
    Ok((&saved).into())
  }
}

pub async fn load_active_database_instance(
  conn: &impl ConnectionTrait,
  id: Uuid,
) -> Result<database_instance::ModelEx, ApiError> {
  database_instance::Entity::load()
    .filter_by_id(id)
    .filter(database_instance::Column::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Database instance '{}' not found", id)))
}
