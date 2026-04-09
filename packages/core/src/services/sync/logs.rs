use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use sea_orm::{
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  ConnectionTrait,
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
  dtos::{PushAuditLogRequest, PushAuditLogsResponse},
  entities::{audit_log, node_base_assignment},
  enums::SyncDirection,
};

#[derive(Debug, Clone, Copy)]
struct LatestAuditLogHead {
  id: Uuid,
  timestamp: DateTime<Utc>,
  user_role_weight: i32,
}

impl LatestAuditLogHead {
  fn from_row(row: &audit_log::ModelEx) -> Self {
    Self {
      id: row.id,
      timestamp: row.timestamp,
      user_role_weight: row.user_role_weight,
    }
  }

  fn from_incoming(incoming: &PushAuditLogRequest) -> Self {
    Self {
      id: incoming.id,
      timestamp: incoming.timestamp,
      user_role_weight: incoming.user_role_weight,
    }
  }

  fn rejects(self, incoming: &PushAuditLogRequest) -> bool {
    self.user_role_weight > incoming.user_role_weight && self.timestamp >= incoming.timestamp
  }

  fn should_replace(self, incoming: &PushAuditLogRequest) -> bool {
    incoming.timestamp > self.timestamp
      || (incoming.timestamp == self.timestamp
        && (incoming.user_role_weight > self.user_role_weight
          || (incoming.user_role_weight == self.user_role_weight && incoming.id > self.id)))
  }
}

#[derive(Debug, Default)]
struct IncomingAuditBatchState {
  existing_ids: HashSet<Uuid>,
  latest_by_record: HashMap<Uuid, LatestAuditLogHead>,
}

impl IncomingAuditBatchState {
  fn contains_id(&self, id: Uuid) -> bool {
    self.existing_ids.contains(&id)
  }

  fn rejects(&self, incoming: &PushAuditLogRequest) -> bool {
    self
      .latest_by_record
      .get(&incoming.record_id)
      .is_some_and(|head| head.rejects(incoming))
  }

  fn observe_insert(&mut self, incoming: &PushAuditLogRequest) {
    self.existing_ids.insert(incoming.id);
    match self.latest_by_record.get_mut(&incoming.record_id) {
      Some(head) if head.should_replace(incoming) => {
        *head = LatestAuditLogHead::from_incoming(incoming);
      }
      Some(_) => {}
      None => {
        self.latest_by_record.insert(
          incoming.record_id,
          LatestAuditLogHead::from_incoming(incoming),
        );
      }
    }
  }
}

impl SyncService {
  fn unique_log_ids(logs: &[PushAuditLogRequest]) -> Vec<Uuid> {
    let mut ids: Vec<Uuid> = logs.iter().map(|log| log.id).collect();
    ids.sort_unstable();
    ids.dedup();
    ids
  }

  fn unique_record_ids(logs: &[PushAuditLogRequest]) -> Vec<Uuid> {
    let mut ids: Vec<Uuid> = logs.iter().map(|log| log.record_id).collect();
    ids.sort_unstable();
    ids.dedup();
    ids
  }

  async fn load_existing_audit_ids<C: ConnectionTrait>(
    conn: &C,
    logs: &[PushAuditLogRequest],
  ) -> Result<HashSet<Uuid>, ApiError> {
    let log_ids = Self::unique_log_ids(logs);
    if log_ids.is_empty() {
      return Ok(HashSet::new());
    }

    let rows = audit_log::Entity::load()
      .filter(audit_log::Column::Id.is_in(log_ids))
      .all(conn)
      .await?;

    Ok(rows.into_iter().map(|row| row.id).collect())
  }

  async fn load_latest_audit_heads<C: ConnectionTrait>(
    conn: &C,
    logs: &[PushAuditLogRequest],
  ) -> Result<HashMap<Uuid, LatestAuditLogHead>, ApiError> {
    let record_ids = Self::unique_record_ids(logs);
    if record_ids.is_empty() {
      return Ok(HashMap::new());
    }

    let rows = audit_log::Entity::load()
      .filter(audit_log::Column::RecordId.is_in(record_ids.clone()))
      .order_by_asc(audit_log::Column::RecordId)
      .order_by_desc(audit_log::Column::Timestamp)
      .order_by_desc(audit_log::Column::UserRoleWeight)
      .order_by_desc(audit_log::Column::Id)
      .all(conn)
      .await?;

    let mut latest_by_record = HashMap::with_capacity(record_ids.len());
    for row in rows {
      latest_by_record
        .entry(row.record_id)
        .or_insert_with(|| LatestAuditLogHead::from_row(&row));
    }

    Ok(latest_by_record)
  }

  async fn load_incoming_batch_state<C: ConnectionTrait>(
    conn: &C,
    logs: &[PushAuditLogRequest],
  ) -> Result<IncomingAuditBatchState, ApiError> {
    Ok(IncomingAuditBatchState {
      existing_ids: Self::load_existing_audit_ids(conn, logs).await?,
      latest_by_record: Self::load_latest_audit_heads(conn, logs).await?,
    })
  }

  async fn apply_incoming_logs_in_txn<C: ConnectionTrait>(
    conn: &C,
    logs: &[PushAuditLogRequest],
  ) -> Result<PushAuditLogsResponse, ApiError> {
    let mut state = Self::load_incoming_batch_state(conn, logs).await?;
    let mut accepted = 0_u64;
    let mut rejected = 0_u64;

    for incoming in logs {
      if state.contains_id(incoming.id) {
        continue;
      }

      if state.rejects(incoming) {
        rejected += 1;
        continue;
      }

      let old_values = parse_json_field(incoming.old_values_json.as_deref(), "oldValuesJson")?;
      let new_values = parse_json_field(incoming.new_values_json.as_deref(), "newValuesJson")?;

      apply_audit_log_restore(
        conn,
        incoming.table_name,
        incoming.action,
        old_values.as_ref(),
        new_values.as_ref(),
      )
      .await?;

      audit_log::ActiveModel {
        id: Set(incoming.id),
        table_name: Set(incoming.table_name),
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
      .insert(conn)
      .await?;

      accepted += 1;
      state.observe_insert(incoming);
    }

    Ok(PushAuditLogsResponse { accepted, rejected })
  }

  pub async fn push_logs(
    &self,
    logs: &[PushAuditLogRequest],
  ) -> Result<PushAuditLogsResponse, ApiError> {
    tracing::debug!(count = logs.len(), "push_logs: processing incoming batch");
    let txn = self.db.begin().await?;
    let result = Self::apply_incoming_logs_in_txn(&txn, logs).await?;
    txn.commit().await?;
    Ok(result)
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
    let assignment_rows = node_base_assignment::Entity::load()
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

    let result = Self::apply_incoming_logs_in_txn(&txn, logs).await?;

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
    Ok(result)
  }
}
