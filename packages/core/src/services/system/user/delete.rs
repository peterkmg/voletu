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
    self.user_set_soft_deleted_state(id, false).await
  }

  pub async fn user_soft_delete_undo(&self, id: Uuid) -> Result<(), ApiError> {
    self.user_set_soft_deleted_state(id, true).await
  }

  async fn user_set_soft_deleted_state(&self, id: Uuid, undo: bool) -> Result<(), ApiError> {
    if undo {
      tracing::info!(id = %id, "Restoring soft deleted user");
    } else {
      tracing::info!(id = %id, "Soft deleting user");
    }

    let local_db_id = self.user_local_db_id().await?;

    let existing = load_local_user_by_id(self.db.as_ref(), local_db_id, id).await?;

    let Some(existing) = existing else {
      if undo {
        tracing::warn!(id = %id, "Soft deleted user not found for restore");
      } else {
        tracing::warn!(id = %id, "User not found for deletion");
      }

      return Err(ApiError::NotFound(if undo {
        format!("Deleted user '{}' not found", id)
      } else {
        format!("User '{}' not found", id)
      }));
    };

    let has_matching_deleted_state = if undo {
      existing.deleted_at.is_some()
    } else {
      existing.deleted_at.is_none()
    };

    if !has_matching_deleted_state {
      if undo {
        tracing::warn!(id = %id, "Soft deleted user not found for restore");
        return Err(ApiError::NotFound(format!(
          "Deleted user '{}' not found",
          id
        )));
      }

      tracing::warn!(id = %id, "User not found for deletion");
      return Err(ApiError::NotFound(format!("User '{}' not found", id)));
    }

    let actor_id = current_actor_id()
      .ok_or_else(|| ApiError::Unauthorized("Missing authenticated actor context".to_string()))?;
    let mut model = user::ActiveModel {
      id: Set(existing.id),
      ..Default::default()
    };
    let now = ChronoUtc::now();
    model.deleted_at = Set(if undo { None } else { Some(now) });
    model.deleted_by = Set(if undo { None } else { Some(actor_id) });
    model.update(self.db.as_ref()).await?;

    if undo {
      tracing::info!(id = %id, "User soft delete restored");
    } else {
      tracing::info!(id = %id, "User soft deleted");
    }

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
