use sea_orm::{
  ColumnTrait,
  Condition,
  DatabaseConnection,
  EntityLoaderTrait,
  EntityTrait,
  Order,
  QueryFilter,
  QueryOrder,
  QuerySelect,
};
use uuid::Uuid;

use super::{
  helpers::{excluded_sync_tables, scope_condition_for},
  specs::{
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
) -> Result<Vec<audit_log::Model>, ApiError> {
  audit_log::Entity::find()
    .filter(condition)
    .order_by(audit_log::Column::Id, Order::Asc)
    .offset(offset)
    .limit(limit.max(1))
    .all(db)
    .await
    .map_err(ApiError::from)
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

  pub async fn audit_log_get(&self, id: Uuid) -> Result<AuditLogResponse, ApiError> {
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
  ) -> Result<Vec<AuditLogResponse>, ApiError> {
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
