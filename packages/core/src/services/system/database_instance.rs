use sea_orm::{
  ActiveModelTrait,
  ActiveValue::{NotSet, Set},
  ColumnTrait,
  EntityTrait,
  QueryFilter,
};
use uuid::Uuid;

use super::SystemService;
use crate::{
  api::ApiError,
  dtos::DatabaseInstanceResponse,
  entities::database_instance,
  enums::NodeType,
};

impl SystemService {
  pub async fn database_instance_list(&self) -> Result<Vec<DatabaseInstanceResponse>, ApiError> {
    let rows = database_instance::Entity::find()
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
    let instance = database_instance::Entity::find()
      .filter(database_instance::Column::Id.eq(id))
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
    let row = database_instance::Entity::find()
      .filter(database_instance::Column::Id.eq(id))
      .filter(database_instance::Column::DeletedAt.is_null())
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Database instance '{}' not found", id)))?;

    let mut model: database_instance::ActiveModel = row.into();
    model.common_name = Set(common_name);
    model.node_type = Set(node_type);
    model.base_id = Set(base_id);
    model.updated_by = NotSet;

    let saved = model
      .update(self.db.as_ref())
      .await
      .map_err(ApiError::Database)?;
    Ok((&saved).into())
  }
}
