use sea_orm::{
  ActiveModelBehavior,
  ActiveModelTrait,
  ColumnTrait,
  ConnectionTrait,
  EntityTrait,
  IntoActiveModel,
  ModelTrait,
  QueryFilter,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{api::ApiError, services::audit::AuditService};

pub async fn list_active<E, C>(
  conn: &C,
  deleted_at_col: E::Column,
) -> Result<Vec<E::Model>, ApiError>
where
  E: EntityTrait,
  C: ConnectionTrait,
  E::Column: ColumnTrait,
{
  let rows = E::find().filter(deleted_at_col.is_null()).all(conn).await?;
  Ok(rows)
}

pub async fn get_active_by_id<E, C>(
  conn: &C,
  id: Uuid,
  id_col: E::Column,
  deleted_at_col: E::Column,
  not_found_message: String,
) -> Result<E::Model, ApiError>
where
  E: EntityTrait,
  C: ConnectionTrait,
  E::Column: ColumnTrait,
{
  let row = E::find()
    .filter(id_col.eq(id))
    .filter(deleted_at_col.is_null())
    .one(conn)
    .await?
    .ok_or(ApiError::NotFound(not_found_message))?;

  Ok(row)
}

pub async fn get_soft_delete_target_by_id<E, C>(
  conn: &C,
  id: Uuid,
  id_col: E::Column,
  deleted_at_col: E::Column,
  undo: bool,
  not_found_message: String,
) -> Result<E::Model, ApiError>
where
  E: EntityTrait,
  C: ConnectionTrait,
  E::Column: ColumnTrait,
{
  let deleted_filter = if undo {
    deleted_at_col.is_not_null()
  } else {
    deleted_at_col.is_null()
  };

  let row = E::find()
    .filter(id_col.eq(id))
    .filter(deleted_filter)
    .one(conn)
    .await?
    .ok_or(ApiError::NotFound(not_found_message))?;

  Ok(row)
}

pub async fn update_with_audit<AM, F>(
  txn: &impl ConnectionTrait,
  audit: &AuditService,
  active_model: AM,
  existing: &<AM::Entity as EntityTrait>::Model,
  id_of: F,
) -> Result<<AM::Entity as EntityTrait>::Model, ApiError>
where
  AM: ActiveModelTrait + ActiveModelBehavior + Send,
  F: Fn(&<AM::Entity as EntityTrait>::Model) -> Uuid,
  <AM::Entity as EntityTrait>::Model: ModelTrait + Serialize + IntoActiveModel<AM>,
{
  let saved = active_model.update(txn).await?;
  audit
    .register_update(txn, id_of(&saved), existing, &saved)
    .await?;
  Ok(saved)
}
