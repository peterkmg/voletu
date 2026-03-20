use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PrimaryKeyTrait, TransactionTrait};

use crate::{api::ApiError, services::audit::AuditService};

pub fn map_hard_delete_db_error(err: DbErr, entity_name: &str) -> ApiError {
  let message = err.to_string().to_lowercase();
  let has_dependency_violation = message.contains("foreign key")
    || message.contains("constraint failed")
    || message.contains("violates foreign key constraint")
    || message.contains("violates constraint");

  if has_dependency_violation {
    ApiError::Conflict(format!(
      "Cannot hard delete {} because dependent records exist",
      entity_name
    ))
  } else {
    ApiError::Database(err)
  }
}

pub async fn hard_delete_with_audit<E>(
  db: &DatabaseConnection,
  audit: &AuditService,
  id: uuid::Uuid,
  entity_name: &str,
  not_found_message: String,
) -> Result<(), ApiError>
where
  E: EntityTrait,
  E::PrimaryKey: PrimaryKeyTrait<ValueType = uuid::Uuid>,
  E::Model: sea_orm::ModelTrait + serde::Serialize,
{
  let txn = db.begin().await?;
  let existing = E::find_by_id(id)
    .one(&txn)
    .await?
    .ok_or(ApiError::NotFound(not_found_message))?;

  E::delete_by_id(id)
    .exec(&txn)
    .await
    .map_err(|err| map_hard_delete_db_error(err, entity_name))?;

  audit.register_delete(&txn, id, &existing).await?;
  txn.commit().await?;
  Ok(())
}
