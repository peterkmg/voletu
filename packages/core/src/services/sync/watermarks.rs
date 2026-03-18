use sea_orm::{
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  Condition,
  EntityTrait,
  QueryFilter,
};
use uuid::Uuid;

use super::SyncService;
use crate::{
  api::ApiError,
  db::ops::list_all,
  dtos::SyncWatermarkResponse,
  entities::sync_watermark,
  enums::SyncDirection,
};

impl SyncService {
  pub async fn list_sync_watermarks(&self) -> Result<Vec<SyncWatermarkResponse>, ApiError> {
    let rows = list_all::<sync_watermark::Entity>(self.db.as_ref()).await?;
    Ok(rows.into_iter().map(SyncWatermarkResponse::from).collect())
  }

  pub async fn sync_watermark_get(&self, id: Uuid) -> Result<SyncWatermarkResponse, ApiError> {
    let row = sync_watermark::Entity::find_by_id(id)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Sync watermark '{}' not found", id)))?;
    Ok(row.into())
  }

  pub async fn sync_watermark_query(
    &self,
    target_node_id: Option<Uuid>,
    direction: Option<SyncDirection>,
  ) -> Result<Vec<SyncWatermarkResponse>, ApiError> {
    let mut condition = Condition::all();

    if let Some(target_node_id) = target_node_id {
      condition = condition.add(sync_watermark::Column::TargetNodeId.eq(target_node_id));
    }

    if let Some(direction) = direction {
      condition = condition.add(sync_watermark::Column::Direction.eq(direction));
    }

    let rows = sync_watermark::Entity::find()
      .filter(condition)
      .all(self.db.as_ref())
      .await?;

    Ok(rows.into_iter().map(SyncWatermarkResponse::from).collect())
  }

  pub async fn upsert_watermark(
    &self,
    target_node_id: Uuid,
    direction: SyncDirection,
    last_audit_log_id: Uuid,
  ) -> Result<SyncWatermarkResponse, ApiError> {
    let existing = sync_watermark::Entity::find()
      .filter(sync_watermark::Column::TargetNodeId.eq(target_node_id))
      .filter(sync_watermark::Column::Direction.eq(direction))
      .one(self.db.as_ref())
      .await?;

    let row = if let Some(existing) = existing {
      let mut am: sync_watermark::ActiveModel = existing.into();
      am.last_audit_log_id = Set(last_audit_log_id);
      am.update(self.db.as_ref()).await?
    } else {
      sync_watermark::ActiveModel {
        target_node_id: Set(target_node_id),
        direction: Set(direction),
        last_audit_log_id: Set(last_audit_log_id),
        ..Default::default()
      }
      .insert(self.db.as_ref())
      .await?
    };

    Ok(row.into())
  }
}
