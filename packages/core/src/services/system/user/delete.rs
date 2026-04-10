use sea_orm::{
  prelude::ChronoUtc,
  ActiveModelTrait,
  ActiveValue::Set,
  DbErr,
  EntityTrait,
  TransactionTrait,
};
use uuid::Uuid;

use super::{super::SystemService, helpers::load_local_user_by_id};
use crate::{api::ApiError, context::audit::current_actor_id, entities::user};

fn map_hard_delete_db_error(err: DbErr, entity_name: &str) -> ApiError {
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

impl SystemService {
  pub async fn user_soft_delete(&self, id: Uuid) -> Result<(), ApiError> {
    tracing::info!(id = %id, "Soft deleting user");
    let user_id = self.load_local_user_for_soft_delete(id).await?;
    let actor_id = current_actor_id()
      .ok_or_else(|| ApiError::Unauthorized("Missing authenticated actor context".to_string()))?;

    self
      .write_soft_delete_state(user_id, Some(ChronoUtc::now()), Some(actor_id))
      .await?;
    tracing::info!(id = %id, "User soft deleted");
    Ok(())
  }

  pub async fn user_soft_delete_undo(&self, id: Uuid) -> Result<(), ApiError> {
    tracing::info!(id = %id, "Restoring soft deleted user");
    let user_id = self.load_local_user_for_restore(id).await?;

    self.write_soft_delete_state(user_id, None, None).await?;
    tracing::info!(id = %id, "User soft delete restored");
    Ok(())
  }

  async fn load_local_user_for_soft_delete(&self, id: Uuid) -> Result<Uuid, ApiError> {
    let local_db_id = self.user_local_db_id().await?;
    let existing = load_local_user_by_id(self.db.as_ref(), local_db_id, id).await?;
    let Some(existing) = existing else {
      tracing::warn!(id = %id, "User not found for deletion");
      return Err(ApiError::NotFound(format!("User '{}' not found", id)));
    };

    if existing.deleted_at.is_some() {
      tracing::warn!(id = %id, "User not found for deletion");
      return Err(ApiError::NotFound(format!("User '{}' not found", id)));
    }

    Ok(existing.id)
  }

  async fn load_local_user_for_restore(&self, id: Uuid) -> Result<Uuid, ApiError> {
    let local_db_id = self.user_local_db_id().await?;
    let existing = load_local_user_by_id(self.db.as_ref(), local_db_id, id).await?;
    let Some(existing) = existing else {
      tracing::warn!(id = %id, "Soft deleted user not found for restore");
      return Err(ApiError::NotFound(format!(
        "Deleted user '{}' not found",
        id
      )));
    };

    if existing.deleted_at.is_none() {
      tracing::warn!(id = %id, "Soft deleted user not found for restore");
      return Err(ApiError::NotFound(format!(
        "Deleted user '{}' not found",
        id
      )));
    }

    Ok(existing.id)
  }

  async fn write_soft_delete_state(
    &self,
    id: Uuid,
    deleted_at: Option<chrono::DateTime<ChronoUtc>>,
    deleted_by: Option<Uuid>,
  ) -> Result<(), ApiError> {
    let mut model = user::ActiveModel {
      id: Set(id),
      ..Default::default()
    };
    model.deleted_at = Set(deleted_at);
    model.deleted_by = Set(deleted_by);
    model.update(self.db.as_ref()).await?;
    Ok(())
  }

  pub async fn user_hard_delete(&self, id: Uuid) -> Result<(), ApiError> {
    tracing::info!(id = %id, "Hard deleting user");
    let local_db_id = self.user_local_db_id().await?;

    let txn = self.db.begin().await?;
    let existing = load_local_user_by_id(&txn, local_db_id, id)
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("User '{}' not found", id)))?;

    user::Entity::delete_by_id(id)
      .exec(&txn)
      .await
      .map_err(|err| map_hard_delete_db_error(err, "user"))?;

    self.audit.register_delete(&txn, id, &existing).await?;
    txn.commit().await?;
    tracing::info!(id = %id, "User hard deleted");
    Ok(())
  }
}
