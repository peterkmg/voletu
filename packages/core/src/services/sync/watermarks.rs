use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, ConnectionTrait, EntityLoaderTrait,
  QueryFilter,
};
use uuid::Uuid;

use super::SyncService;
use crate::{
  api::ApiError, dtos::SyncWatermarkResponse, entities::sync_watermark, enums::SyncDirection,
};

impl SyncService {
  async fn load_watermark_row<C: ConnectionTrait>(
    conn: &C,
    target_node_id: Uuid,
    direction: SyncDirection,
  ) -> Result<Option<sync_watermark::ModelEx>, ApiError> {
    sync_watermark::Entity::load()
      .filter(sync_watermark::Column::TargetNodeId.eq(target_node_id))
      .filter(sync_watermark::Column::Direction.eq(direction))
      .one(conn)
      .await
      .map_err(Into::into)
  }

  pub async fn list_sync_watermarks(&self) -> Result<Vec<SyncWatermarkResponse>, ApiError> {
    let rows: Vec<sync_watermark::ModelEx> =
      sync_watermark::Entity::load().all(self.db.as_ref()).await?;
    Ok(rows.into_iter().map(SyncWatermarkResponse::from).collect())
  }

  pub async fn sync_watermark_get(&self, id: Uuid) -> Result<SyncWatermarkResponse, ApiError> {
    let row: sync_watermark::ModelEx = sync_watermark::Entity::load()
      .filter_by_id(id)
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

    let rows: Vec<sync_watermark::ModelEx> = sync_watermark::Entity::load()
      .filter(condition)
      .all(self.db.as_ref())
      .await?;

    Ok(rows.into_iter().map(SyncWatermarkResponse::from).collect())
  }

  /// Load the `(last_audit_log_id, base_discriminant)` pair for a specific
  /// target node and direction. Returns `(Uuid::nil(), String::new())` when no
  /// row exists yet — this is the "fresh peripheral, never synced" state.
  pub async fn load_pull_watermark(
    &self,
    target_node_id: Uuid,
    direction: SyncDirection,
  ) -> Result<(Uuid, String), ApiError> {
    let row = Self::load_watermark_row(self.db.as_ref(), target_node_id, direction).await?;
    Ok(match row {
      Some(r) => (r.last_audit_log_id, r.base_discriminant),
      None => (Uuid::nil(), String::new()),
    })
  }

  /// Upsert a watermark row outside of any caller-managed transaction. The
  /// peripheral's pull path must NOT use this method directly — it should go
  /// through `apply_pulled_logs`, which atomically applies logs and advances
  /// the watermark with a discriminant re-check. This is the entry point for
  /// the push path, which has no scope/discriminant concept and may pass an
  /// empty string for `base_discriminant`.
  pub async fn upsert_watermark(
    &self,
    target_node_id: Uuid,
    direction: SyncDirection,
    last_audit_log_id: Uuid,
    base_discriminant: String,
  ) -> Result<SyncWatermarkResponse, ApiError> {
    let row = Self::upsert_watermark_in_txn(
      self.db.as_ref(),
      target_node_id,
      direction,
      last_audit_log_id,
      base_discriminant,
    )
    .await?;
    Ok(row.into())
  }

  /// Upsert a watermark row on an existing connection or transaction. Used by
  /// `apply_pulled_logs` to advance the PULL watermark in the same transaction
  /// as the audit-log restore, guaranteeing the two commit together or not at
  /// all.
  pub async fn upsert_watermark_in_txn<C: ConnectionTrait>(
    conn: &C,
    target_node_id: Uuid,
    direction: SyncDirection,
    last_audit_log_id: Uuid,
    base_discriminant: String,
  ) -> Result<sync_watermark::Model, ApiError> {
    let existing = Self::load_watermark_row(conn, target_node_id, direction).await?;

    let row = if let Some(existing) = existing {
      sync_watermark::ActiveModel {
        id: Set(existing.id),
        last_audit_log_id: Set(last_audit_log_id),
        base_discriminant: Set(base_discriminant),
        ..Default::default()
      }
      .update(conn)
      .await?
    } else {
      sync_watermark::ActiveModel {
        target_node_id: Set(target_node_id),
        direction: Set(direction),
        last_audit_log_id: Set(last_audit_log_id),
        base_discriminant: Set(base_discriminant),
        ..Default::default()
      }
      .insert(conn)
      .await?
    };

    Ok(row)
  }
}
