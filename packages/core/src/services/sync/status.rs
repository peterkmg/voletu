use sea_orm::{ColumnTrait, Condition, EntityTrait, Order, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

use super::{helpers::targeted_base_condition, SyncService};
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

  pub async fn sync_status(&self) -> Result<SyncStatusResponse, ApiError> {
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

    let latest_log = audit_log::Entity::find()
      .order_by_desc(audit_log::Column::Id)
      .one(self.db.as_ref())
      .await?;
    let highest_audit_log_id = match latest_log {
      Some(row) => row.id,
      None => Uuid::nil(),
    };

    Ok(SyncStatusResponse {
      node_id: instance.id,
      node_type: instance.node_type.to_string(),
      highest_audit_log_id,
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
  /// `base_ids` is provided by the requesting peripheral — Central filters accordingly.
  /// Empty `base_ids` = catalog-only sync (global tables only).
  pub async fn pull_logs(
    &self,
    last_audit_log_id: Uuid,
    base_ids: &[Uuid],
    limit: Option<u64>,
  ) -> Result<PullAuditLogsResponse, ApiError> {
    let max_limit = limit.unwrap_or(1000).min(1000);
    let excluded_tables = vec![
      "roles".to_string(),
      "local".to_string(),
      "node_base_assignments".to_string(),
    ];
    let global_tables = vec![
      "companies".to_string(),
      "products".to_string(),
      "product_groups".to_string(),
      "product_types".to_string(),
      "bases".to_string(),
      "warehouses".to_string(),
      "storages".to_string(),
      "ports".to_string(),
      "users".to_string(),
      "database_instances".to_string(),
    ];

    let scope_condition = if base_ids.is_empty() {
      // No base IDs provided → catalog-only sync (global tables)
      Condition::any().add(audit_log::Column::TableName.is_in(global_tables))
    } else {
      // Peripheral requesting specific bases → global + targeted for each base
      let mut cond = Condition::any().add(audit_log::Column::TableName.is_in(global_tables));
      for base_id in base_ids {
        cond = cond.add(targeted_base_condition(&base_id.to_string()));
      }
      cond
    };

    let logs = audit_log::Entity::find()
      .filter(audit_log::Column::Id.gt(last_audit_log_id))
      .filter(audit_log::Column::TableName.is_not_in(excluded_tables))
      .filter(scope_condition)
      .order_by(audit_log::Column::Id, Order::Asc)
      .limit(max_limit)
      .all(self.db.as_ref())
      .await?;

    let highest_evaluated_id = if let Some(last) = logs.last() {
      last.id
    } else {
      let latest_log = audit_log::Entity::find()
        .order_by_desc(audit_log::Column::Id)
        .one(self.db.as_ref())
        .await?;
      match latest_log {
        Some(log) => log.id,
        None => last_audit_log_id,
      }
    };

    Ok(PullAuditLogsResponse {
      highest_evaluated_id,
      logs: logs.into_iter().map(AuditLogResponse::from).collect(),
    })
  }
}
