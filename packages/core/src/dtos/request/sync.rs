use chrono::{DateTime, Utc};
use uuid::Uuid;
use voletu_core_macros::request_dto;

#[request_dto]
pub struct PushAuditLogRequest {
  pub id: Uuid,
  pub table_name: String,
  pub record_id: Uuid,
  pub action: String,
  pub old_values_json: Option<String>,
  pub new_values_json: Option<String>,
  pub target_base_ids: String,
  pub user_role_weight: i32,
  pub user_id: Option<Uuid>,
  pub timestamp: DateTime<Utc>,
  pub origin_db_id: Uuid,
}

#[request_dto]
pub struct PushAuditLogsRequest {
  pub logs: Vec<PushAuditLogRequest>,
}
