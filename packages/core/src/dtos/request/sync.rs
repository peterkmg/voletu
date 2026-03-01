use chrono::{DateTime, Utc};
use uuid::Uuid;
use voletu_core_macros::request_dto;

use crate::enums::AuditAction;

#[request_dto]
pub struct PushAuditLogRequest {
  pub id: Uuid,
  #[validate(length(min = 1))]
  pub table_name: String,
  pub record_id: Uuid,
  pub action: AuditAction,
  pub old_values_json: Option<String>,
  pub new_values_json: Option<String>,
  #[validate(length(min = 1))]
  pub target_base_ids: String,
  #[validate(range(min = 0))]
  pub user_role_weight: i32,
  pub user_id: Option<Uuid>,
  pub timestamp: DateTime<Utc>,
  pub origin_db_id: Uuid,
}

#[request_dto]
pub struct PushAuditLogsRequest {
  #[validate(length(min = 1))]
  pub logs: Vec<PushAuditLogRequest>,
}
