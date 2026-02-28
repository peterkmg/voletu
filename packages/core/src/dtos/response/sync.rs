use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::dtos::enums::SyncDirection;

#[response_dto]
#[derive(Deserialize)]
pub struct SyncStatusResponse {
  pub node_id: Uuid,
  pub node_type: String,
  pub highest_audit_log_id: Uuid,
}

#[response_dto]
#[derive(Deserialize)]
pub struct SyncWatermarkResponse {
  pub id: Uuid,
  pub target_node_id: Uuid,
  pub direction: SyncDirection,
  pub last_audit_log_id: Uuid,
  pub synced_at: String,
}

#[response_dto]
#[derive(Deserialize)]
pub struct AuditLogResponse {
  pub id: Uuid,
  pub table_name: String,
  pub record_id: Uuid,
  pub action: String,
  pub target_base_ids: String,
  pub user_role_weight: i32,
  pub user_id: Option<Uuid>,
  pub timestamp: DateTime<Utc>,
  pub origin_db_id: Uuid,
}

#[response_dto]
#[derive(Deserialize)]
pub struct PushAuditLogsResponse {
  pub accepted: u64,
  pub rejected: u64,
}

#[response_dto]
#[derive(Deserialize)]
pub struct PullAuditLogsResponse {
  pub highest_evaluated_id: Uuid,
  pub logs: Vec<AuditLogResponse>,
}
