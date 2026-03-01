use sea_orm::{ColumnTrait, Condition};
use serde_json::Value;

use crate::{
  api::ApiError,
  dtos::{AuditLogResponse, PushAuditLogRequest, SyncWatermarkResponse},
  entities::{audit_log, sync_watermark},
};

pub fn map_audit_row(row: audit_log::Model) -> AuditLogResponse {
  AuditLogResponse {
    id: row.id,
    table_name: row.table_name,
    record_id: row.record_id,
    action: row.action,
    old_values_json: row.old_values.map(|value| value.to_string()),
    new_values_json: row.new_values.map(|value| value.to_string()),
    target_base_ids: row.target_base_ids,
    user_role_weight: row.user_role_weight,
    user_id: row.user_id,
    timestamp: row.timestamp,
    origin_db_id: row.origin_db_id,
  }
}

pub fn map_push_request(row: audit_log::Model) -> PushAuditLogRequest {
  PushAuditLogRequest {
    id: row.id,
    table_name: row.table_name,
    record_id: row.record_id,
    action: row.action,
    old_values_json: row.old_values.map(|value| value.to_string()),
    new_values_json: row.new_values.map(|value| value.to_string()),
    target_base_ids: row.target_base_ids,
    user_role_weight: row.user_role_weight,
    user_id: row.user_id,
    timestamp: row.timestamp,
    origin_db_id: row.origin_db_id,
  }
}

pub fn map_audit_to_push(log: AuditLogResponse) -> PushAuditLogRequest {
  PushAuditLogRequest {
    id: log.id,
    table_name: log.table_name,
    record_id: log.record_id,
    action: log.action,
    old_values_json: log.old_values_json,
    new_values_json: log.new_values_json,
    target_base_ids: log.target_base_ids,
    user_role_weight: log.user_role_weight,
    user_id: log.user_id,
    timestamp: log.timestamp,
    origin_db_id: log.origin_db_id,
  }
}

pub fn map_sync_watermark(row: sync_watermark::Model) -> SyncWatermarkResponse {
  SyncWatermarkResponse {
    id: row.id,
    target_node_id: row.target_node_id,
    direction: row.direction,
    last_audit_log_id: row.last_audit_log_id,
    synced_at: row.synced_at.to_rfc3339(),
  }
}

pub fn normalize_target_base_ids(value: &str) -> String {
  let mut base_ids = value
    .split(',')
    .map(str::trim)
    .filter(|part| !part.is_empty())
    .collect::<Vec<_>>();
  base_ids.sort_unstable();
  base_ids.dedup();

  if base_ids.is_empty() {
    return String::new();
  }

  format!(",{},", base_ids.join(","))
}

pub fn targeted_base_condition(base_id: &str) -> Condition {
  let wrapped = format!("%,{},%", base_id);
  let starts_with = format!("{},%", base_id);
  let ends_with = format!("%,{}", base_id);

  Condition::any()
    .add(audit_log::Column::TargetBaseIds.eq(base_id))
    .add(audit_log::Column::TargetBaseIds.like(starts_with))
    .add(audit_log::Column::TargetBaseIds.like(ends_with))
    .add(audit_log::Column::TargetBaseIds.like(wrapped))
}

pub fn parse_json_field(raw: Option<&str>, field: &str) -> Result<Option<Value>, ApiError> {
  raw
    .map(|json| {
      serde_json::from_str(json)
        .map_err(|_| ApiError::Validation(format!("Invalid JSON payload for {}", field)))
    })
    .transpose()
}
