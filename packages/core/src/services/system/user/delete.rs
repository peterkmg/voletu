use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};
use uuid::Uuid;

use super::super::SystemService;
use crate::{
  api::ApiError,
  context::audit::current_actor_id,
  entities::user,
  services::common::{map_hard_delete_db_error, set_soft_deleted_fields},
};

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

    let existing = user::Entity::find()
      .filter(user::Column::Id.eq(id))
      .filter(user::Column::OriginDbId.eq(local_db_id))
      .filter(if undo {
        user::Column::DeletedAt.is_not_null()
      } else {
        user::Column::DeletedAt.is_null()
      })
      .one(self.db.as_ref())
      .await?;

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

    let actor_id = current_actor_id()
      .ok_or_else(|| ApiError::Unauthorized("Missing authenticated actor context".to_string()))?;
    let mut model: user::ActiveModel = existing.into();
    set_soft_deleted_fields(&mut model.deleted_at, &mut model.deleted_by, undo, actor_id);
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
    let existing = user::Entity::find()
      .filter(user::Column::Id.eq(id))
      .filter(user::Column::OriginDbId.eq(local_db_id))
      .one(&txn)
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
