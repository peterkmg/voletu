use sea_orm::{ColumnTrait, Condition, EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

use super::{
  helpers::{excluded_sync_tables, scope_condition_for},
  SyncService,
};
use crate::{
  api::ApiError,
  db::ops::list_all,
  dtos::{AuditLogResponse, PullAuditLogsResponse, PushAuditLogRequest, SyncStatusResponse},
  entities::{audit_log, database_instance},
};

impl SyncService {
  pub async fn list_audit_logs(&self) -> Result<Vec<crate::dtos::AuditLogResponse>, ApiError> {
    let rows = list_all::<audit_log::Entity>(self.db.as_ref()).await?;
    Ok(rows.into_iter().map(AuditLogResponse::from).collect())
  }

  pub async fn audit_log_get(&self, id: Uuid) -> Result<crate::dtos::AuditLogResponse, ApiError> {
    let row = audit_log::Entity::find_by_id(id)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Audit log '{}' not found", id)))?;
    Ok(row.into())
  }

  pub async fn audit_log_query(
    &self,
    table_name: Option<&str>,
    record_id: Option<Uuid>,
    origin_db_id: Option<Uuid>,
    limit: Option<u64>,
    offset: Option<u64>,
  ) -> Result<Vec<crate::dtos::AuditLogResponse>, ApiError> {
    let mut condition = Condition::all();

    if let Some(table_name) = table_name {
      condition = condition.add(audit_log::Column::TableName.eq(table_name));
    }

    if let Some(record_id) = record_id {
      condition = condition.add(audit_log::Column::RecordId.eq(record_id));
    }

    if let Some(origin_db_id) = origin_db_id {
      condition = condition.add(audit_log::Column::OriginDbId.eq(origin_db_id));
    }

    let rows = audit_log::Entity::find()
      .filter(condition)
      .order_by(audit_log::Column::Id, Order::Asc)
      .limit(limit.unwrap_or(100).min(1000))
      .offset(offset.unwrap_or(0))
      .all(self.db.as_ref())
      .await?;

    Ok(rows.into_iter().map(AuditLogResponse::from).collect())
  }

  /// Return the node's identity and sync high-water marks.
  ///
  /// `base_ids` is the caller's requested scope. When non-empty, Central
  /// filters the `highest_matching_id` computation to logs matching that
  /// scope (global tables OR targeted for any of the provided bases). When
  /// empty, the scope is catalog-only (global tables only). This is what
  /// lets the worker's `has_updates` check avoid hot-polling when Central
  /// has activity on bases the caller does not serve.
  pub async fn sync_status(&self, base_ids: &[Uuid]) -> Result<SyncStatusResponse, ApiError> {
    let local_node_id = self.cfg.node.db_id;
    let instance_row = database_instance::Entity::find_by_id(local_node_id)
      .one(self.db.as_ref())
      .await?;
    let instance = match instance_row {
      Some(instance) => instance,
      None => {
        return Err(ApiError::Internal(anyhow::anyhow!(
          "Database instance row '{}' is missing",
          local_node_id
        )));
      }
    };

    // Highest overall (unfiltered) — diagnostic / liveness signal.
    let latest_log = audit_log::Entity::find()
      .order_by_desc(audit_log::Column::Id)
      .one(self.db.as_ref())
      .await?;
    let highest_audit_log_id = match latest_log {
      Some(row) => row.id,
      None => Uuid::nil(),
    };

    // Highest in-scope — the authoritative "is there anything for you"
    // signal. Shares the `scope_condition_for` helper with `pull_logs` to
    // guarantee the two endpoints never drift.
    let latest_matching = audit_log::Entity::find()
      .filter(audit_log::Column::TableName.is_not_in(excluded_sync_tables()))
      .filter(scope_condition_for(base_ids))
      .order_by_desc(audit_log::Column::Id)
      .one(self.db.as_ref())
      .await?;
    let highest_matching_id = match latest_matching {
      Some(row) => row.id,
      None => Uuid::nil(),
    };

    Ok(SyncStatusResponse {
      node_id: instance.id,
      node_type: instance.node_type.to_string(),
      highest_audit_log_id,
      highest_matching_id,
    })
  }

  pub async fn outbound_logs(
    &self,
    after_audit_log_id: Uuid,
    limit: Option<u64>,
  ) -> Result<Vec<PushAuditLogRequest>, ApiError> {
    let max_limit = limit.unwrap_or(1000).min(1000);
    let rows = audit_log::Entity::find()
      .filter(audit_log::Column::Id.gt(after_audit_log_id))
      .order_by(audit_log::Column::Id, Order::Asc)
      .limit(max_limit)
      .all(self.db.as_ref())
      .await?;

    Ok(
      rows
        .into_iter()
        .map(AuditLogResponse::from)
        .map(PushAuditLogRequest::from)
        .collect(),
    )
  }

  /// Pull audit logs for a requesting node.
  ///
  /// Returns logs matching the requester's scope with `id > last_audit_log_id`,
  /// ordered by id ascending, up to `limit` rows (default 1000, capped at 1000).
  /// `base_ids` is provided by the requesting peripheral — Central filters
  /// accordingly. An empty `base_ids` slice means catalog-only scope
  /// (global tables only).
  ///
  /// The `highest_evaluated_id` in the response is the id of the last log in
  /// the batch, or `last_audit_log_id` if no logs matched. It is NOT the max
  /// audit_log id in Central's table — that optimization used to cause a
  /// leapfrog bug where the peripheral's PULL watermark advanced past logs it
  /// had never actually processed under its scope. Correct watermark-advancement
  /// discipline now lives on the peripheral side in `apply_pulled_logs`, which
  /// is guarded by `base_discriminant`. Central just reports what it actually
  /// returned.
  pub async fn pull_logs(
    &self,
    last_audit_log_id: Uuid,
    base_ids: &[Uuid],
    limit: Option<u64>,
  ) -> Result<PullAuditLogsResponse, ApiError> {
    let max_limit = limit.unwrap_or(1000).min(1000);

    let logs = audit_log::Entity::find()
      .filter(audit_log::Column::Id.gt(last_audit_log_id))
      .filter(audit_log::Column::TableName.is_not_in(excluded_sync_tables()))
      .filter(scope_condition_for(base_ids))
      .order_by(audit_log::Column::Id, Order::Asc)
      .limit(max_limit)
      .all(self.db.as_ref())
      .await?;

    let highest_evaluated_id = logs.last().map(|l| l.id).unwrap_or(last_audit_log_id);

    Ok(PullAuditLogsResponse {
      highest_evaluated_id,
      logs: logs.into_iter().map(AuditLogResponse::from).collect(),
    })
  }
}
