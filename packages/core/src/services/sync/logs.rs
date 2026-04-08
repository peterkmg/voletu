use sea_orm::{
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  EntityTrait,
  QueryFilter,
  QueryOrder,
  TransactionTrait,
};
use uuid::Uuid;

use super::{
  helpers::{compute_base_discriminant, normalize_target_base_ids, parse_json_field},
  restore::apply_audit_log_restore,
  SyncService,
};
use crate::{
  api::ApiError,
  db::ops::exists_by_id,
  dtos::{PushAuditLogRequest, PushAuditLogsResponse},
  entities::{audit_log, node_base_assignment},
  enums::{AuditTable, SyncDirection},
};

impl SyncService {
  pub async fn push_logs(
    &self,
    logs: &[PushAuditLogRequest],
  ) -> Result<PushAuditLogsResponse, ApiError> {
    tracing::debug!(count = logs.len(), "push_logs: processing incoming batch");
    let txn = self.db.begin().await?;
    let mut accepted = 0_u64;
    let mut rejected = 0_u64;

    for incoming in logs {
      let already_exists = exists_by_id::<audit_log::Entity>(&txn, incoming.id).await?;
      if already_exists {
        continue;
      }

      let latest_for_record = audit_log::Entity::find()
        .filter(audit_log::Column::RecordId.eq(incoming.record_id))
        .order_by_desc(audit_log::Column::Timestamp)
        .one(&txn)
        .await?;
      if let Some(existing) = latest_for_record {
        let incoming_ts = incoming.timestamp;
        if existing.user_role_weight > incoming.user_role_weight
          && existing.timestamp >= incoming_ts
        {
          rejected += 1;
          continue;
        }
      }

      let old_values = parse_json_field(incoming.old_values_json.as_deref(), "oldValuesJson")?;
      let new_values = parse_json_field(incoming.new_values_json.as_deref(), "newValuesJson")?;

      if AuditTable::resolve(&incoming.table_name).is_none() {
        return Err(ApiError::BadRequest(format!(
          "Unsupported audit table '{}' for sync log ingestion",
          incoming.table_name
        )));
      }
      apply_audit_log_restore(
        &txn,
        &incoming.table_name,
        incoming.action,
        old_values.as_ref(),
        new_values.as_ref(),
      )
      .await?;

      audit_log::ActiveModel {
        id: Set(incoming.id),
        table_name: Set(incoming.table_name.clone()),
        record_id: Set(incoming.record_id),
        action: Set(incoming.action),
        old_values: Set(old_values),
        new_values: Set(new_values),
        target_base_ids: Set(normalize_target_base_ids(&incoming.target_base_ids)),
        user_role_weight: Set(incoming.user_role_weight),
        user_id: Set(incoming.user_id),
        timestamp: Set(incoming.timestamp),
        origin_db_id: Set(incoming.origin_db_id),
      }
      .insert(&txn)
      .await?;
      accepted += 1;
    }

    txn.commit().await?;
    Ok(PushAuditLogsResponse { accepted, rejected })
  }

  /// Apply a batch of pulled audit logs from a remote peer and advance the
  /// PULL watermark in a single database transaction.
  ///
  /// This is the peripheral's entry point for pull responses. Unlike
  /// `push_logs` (which is called on Central when a peripheral uploads its
  /// outbound batch), `apply_pulled_logs`:
  ///
  ///   1. Opens one transaction on the local DB.
  ///   2. Re-reads `node_base_assignment` and recomputes the current
  ///      `base_discriminant`.
  ///   3. Aborts with `ApiError::Conflict` if the caller's
  ///      `expected_discriminant` does not match the current one — nothing is
  ///      persisted, nothing is advanced. The worker treats this as a
  ///      transient drift and retries on the next tick.
  ///   4. Applies each log via `apply_audit_log_restore` + audit_log row
  ///      insert, subject to the same conflict-resolution rule as `push_logs`
  ///      (reject lower-role updates when a newer higher-role log exists
  ///      locally).
  ///   5. Upserts the PULL watermark with the new `last_audit_log_id` and the
  ///      current discriminant via `upsert_watermark_in_txn`.
  ///   6. Commits.
  ///
  /// Either everything commits or nothing does. There is no code path where
  /// the PULL watermark advances past rows that were not applied.
  pub async fn apply_pulled_logs(
    &self,
    logs: &[PushAuditLogRequest],
    target_node_id: Uuid,
    new_last_audit_log_id: Uuid,
    expected_discriminant: String,
  ) -> Result<PushAuditLogsResponse, ApiError> {
    let local_node_id = self.cfg.node.db_id;
    let txn = self.db.begin().await?;

    // Re-read base assignments inside the transaction and recompute the
    // current discriminant. If it has drifted since the pull was issued,
    // abort the whole transaction — no logs applied, no watermark advanced.
    let assignment_rows = node_base_assignment::Entity::find()
      .filter(node_base_assignment::Column::NodeId.eq(local_node_id))
      .all(&txn)
      .await?;
    let current_base_ids: Vec<Uuid> = assignment_rows.into_iter().map(|r| r.base_id).collect();
    let current_discriminant = compute_base_discriminant(&current_base_ids);

    if current_discriminant != expected_discriminant {
      txn.rollback().await?;
      return Err(ApiError::Conflict(format!(
        "base discriminant drifted during pull (expected='{}', current='{}') — retrying next tick",
        expected_discriminant, current_discriminant
      )));
    }

    let mut accepted = 0_u64;
    let mut rejected = 0_u64;

    for incoming in logs {
      let already_exists = exists_by_id::<audit_log::Entity>(&txn, incoming.id).await?;
      if already_exists {
        continue;
      }

      // Conflict resolution (same rule as push_logs): skip this incoming log
      // if a newer local log with a higher role weight exists for the same
      // record.
      let latest_for_record = audit_log::Entity::find()
        .filter(audit_log::Column::RecordId.eq(incoming.record_id))
        .order_by_desc(audit_log::Column::Timestamp)
        .one(&txn)
        .await?;
      if let Some(existing) = latest_for_record {
        if existing.user_role_weight > incoming.user_role_weight
          && existing.timestamp >= incoming.timestamp
        {
          rejected += 1;
          continue;
        }
      }

      let old_values = parse_json_field(incoming.old_values_json.as_deref(), "oldValuesJson")?;
      let new_values = parse_json_field(incoming.new_values_json.as_deref(), "newValuesJson")?;

      if AuditTable::resolve(&incoming.table_name).is_none() {
        return Err(ApiError::BadRequest(format!(
          "Unsupported audit table '{}' for pulled log ingestion",
          incoming.table_name
        )));
      }
      apply_audit_log_restore(
        &txn,
        &incoming.table_name,
        incoming.action,
        old_values.as_ref(),
        new_values.as_ref(),
      )
      .await?;

      audit_log::ActiveModel {
        id: Set(incoming.id),
        table_name: Set(incoming.table_name.clone()),
        record_id: Set(incoming.record_id),
        action: Set(incoming.action),
        old_values: Set(old_values),
        new_values: Set(new_values),
        target_base_ids: Set(normalize_target_base_ids(&incoming.target_base_ids)),
        user_role_weight: Set(incoming.user_role_weight),
        user_id: Set(incoming.user_id),
        timestamp: Set(incoming.timestamp),
        origin_db_id: Set(incoming.origin_db_id),
      }
      .insert(&txn)
      .await?;
      accepted += 1;
    }

    // Advance the PULL watermark atomically in the same transaction.
    Self::upsert_watermark_in_txn(
      &txn,
      target_node_id,
      SyncDirection::Pull,
      new_last_audit_log_id,
      current_discriminant,
    )
    .await?;

    txn.commit().await?;
    Ok(PushAuditLogsResponse { accepted, rejected })
  }
}
