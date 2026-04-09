use sea_orm::{
  ColumnTrait,
  Condition,
  DatabaseConnection,
  EntityLoaderTrait,
  Order,
  QueryFilter,
  QueryOrder,
};
use uuid::Uuid;

use super::{
  helpers::{excluded_sync_tables, scope_condition_for},
  query::{
    AuditLogQuerySpec,
    OutboundAuditLogsQuerySpec,
    PullAuditLogsQuerySpec,
    SyncStatusQuerySpec,
  },
  SyncService,
};
use crate::{
  api::ApiError,
  dtos::{AuditLogResponse, PullAuditLogsResponse, PushAuditLogRequest, SyncStatusResponse},
  entities::{audit_log, database_instance},
};

fn bounded_limit(limit: Option<u64>, default: u64) -> u64 {
  limit.unwrap_or(default).min(1000)
}

async fn load_audit_log_slice(
  db: &DatabaseConnection,
  condition: Condition,
  limit: u64,
  offset: u64,
) -> Result<Vec<audit_log::ModelEx>, ApiError> {
  let per_page = limit.max(1);
  let page_index = offset / per_page;
  let intra_page_offset = (offset % per_page) as usize;

  let paginator = audit_log::Entity::load()
    .filter(condition)
    .order_by(audit_log::Column::Id, Order::Asc)
    .paginate(db, per_page);

  let mut rows = paginator.fetch_page(page_index).await?;

  if intra_page_offset > 0 {
    rows = rows.into_iter().skip(intra_page_offset).collect();

    if rows.len() < per_page as usize {
      let needed = per_page as usize - rows.len();
      let next_page = paginator.fetch_page(page_index + 1).await?;
      rows.extend(next_page.into_iter().take(needed));
    }
  }

  Ok(rows)
}

impl SyncService {
  async fn load_local_database_instance(&self) -> Result<database_instance::ModelEx, ApiError> {
    let local_node_id = self.cfg.node.db_id;
    database_instance::Entity::load()
      .filter_by_id(local_node_id)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| {
        ApiError::Internal(anyhow::anyhow!(
          "Database instance row '{}' is missing",
          local_node_id
        ))
      })
  }

  async fn highest_audit_log_id(&self) -> Result<Uuid, ApiError> {
    Ok(
      audit_log::Entity::load()
        .order_by_desc(audit_log::Column::Id)
        .one(self.db.as_ref())
        .await?
        .map(|row| row.id)
        .unwrap_or_else(Uuid::nil),
    )
  }

  async fn highest_matching_audit_log_id(&self, base_ids: &[Uuid]) -> Result<Uuid, ApiError> {
    Ok(
      audit_log::Entity::load()
        .filter(audit_log::Column::TableName.is_not_in(excluded_sync_tables()))
        .filter(scope_condition_for(base_ids))
        .order_by_desc(audit_log::Column::Id)
        .one(self.db.as_ref())
        .await?
        .map(|row| row.id)
        .unwrap_or_else(Uuid::nil),
    )
  }

  pub async fn audit_log_get(&self, id: Uuid) -> Result<crate::dtos::AuditLogResponse, ApiError> {
    let row: audit_log::ModelEx = audit_log::Entity::load()
      .filter_by_id(id)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Audit log '{}' not found", id)))?;
    Ok(row.into())
  }

  pub async fn audit_log_query(
    &self,
    query: AuditLogQuerySpec,
  ) -> Result<Vec<crate::dtos::AuditLogResponse>, ApiError> {
    let mut condition = Condition::all();
    let max_limit = bounded_limit(query.limit, 100);
    let offset = query.offset.unwrap_or(0);

    if let Some(table_name) = query.table_name {
      condition = condition.add(audit_log::Column::TableName.eq(table_name));
    }

    if let Some(record_id) = query.record_id {
      condition = condition.add(audit_log::Column::RecordId.eq(record_id));
    }

    if let Some(origin_db_id) = query.origin_db_id {
      condition = condition.add(audit_log::Column::OriginDbId.eq(origin_db_id));
    }

    let rows = load_audit_log_slice(self.db.as_ref(), condition, max_limit, offset).await?;

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
  pub async fn sync_status(
    &self,
    query: SyncStatusQuerySpec,
  ) -> Result<SyncStatusResponse, ApiError> {
    let instance = self.load_local_database_instance().await?;
    let highest_audit_log_id = self.highest_audit_log_id().await?;
    let highest_matching_id = self.highest_matching_audit_log_id(&query.base_ids).await?;

    Ok(SyncStatusResponse {
      node_id: instance.id,
      node_type: instance.node_type.to_string(),
      highest_audit_log_id,
      highest_matching_id,
    })
  }

  pub async fn outbound_logs(
    &self,
    query: OutboundAuditLogsQuerySpec,
  ) -> Result<Vec<PushAuditLogRequest>, ApiError> {
    let max_limit = bounded_limit(query.limit, 1000);
    let rows = load_audit_log_slice(
      self.db.as_ref(),
      Condition::all().add(audit_log::Column::Id.gt(query.after_audit_log_id)),
      max_limit,
      0,
    )
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
    query: PullAuditLogsQuerySpec,
  ) -> Result<PullAuditLogsResponse, ApiError> {
    let max_limit = bounded_limit(query.limit, 1000);
    let condition = Condition::all()
      .add(audit_log::Column::Id.gt(query.last_audit_log_id))
      .add(audit_log::Column::TableName.is_not_in(excluded_sync_tables()))
      .add(scope_condition_for(&query.base_ids));
    let logs = load_audit_log_slice(self.db.as_ref(), condition, max_limit, 0).await?;

    let highest_evaluated_id = logs.last().map(|l| l.id).unwrap_or(query.last_audit_log_id);

    Ok(PullAuditLogsResponse {
      highest_evaluated_id,
      logs: logs.into_iter().map(AuditLogResponse::from).collect(),
    })
  }
}
