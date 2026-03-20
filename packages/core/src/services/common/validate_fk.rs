use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::api::ApiError;

/// Validate that a FK reference points to an existing, non-deleted record.
pub async fn validate_fk_exists<E>(
  db: &impl ConnectionTrait,
  id: Uuid,
  id_col: E::Column,
  deleted_at_col: E::Column,
  field_name: &str,
) -> Result<(), ApiError>
where
  E: EntityTrait,
  E::Column: ColumnTrait,
{
  let exists = E::find()
    .filter(id_col.eq(id))
    .filter(deleted_at_col.is_null())
    .one(db)
    .await?;

  if exists.is_none() {
    return Err(ApiError::BadRequest(format!(
      "{field_name} '{id}' does not reference a valid record"
    )));
  }

  Ok(())
}

/// Validate an optional FK reference (skip if None).
pub async fn validate_optional_fk_exists<E>(
  db: &impl ConnectionTrait,
  id: Option<Uuid>,
  id_col: E::Column,
  deleted_at_col: E::Column,
  field_name: &str,
) -> Result<(), ApiError>
where
  E: EntityTrait,
  E::Column: ColumnTrait,
{
  if let Some(id) = id {
    validate_fk_exists::<E>(db, id, id_col, deleted_at_col, field_name).await?;
  }
  Ok(())
}
