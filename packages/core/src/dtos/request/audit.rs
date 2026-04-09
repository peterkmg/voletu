use chrono::{DateTime, Utc};
use uuid::Uuid;
use voletu_core_macros::request_dto;

use crate::{
  dtos::AuditLogResponse,
  enums::{AuditAction, AuditTable},
};

#[request_dto]
pub struct PushAuditLogRequest {
  pub id: Uuid,
  pub table_name: AuditTable,
  pub record_id: Uuid,
  pub action: AuditAction,
  pub old_values_json: Option<String>,
  pub new_values_json: Option<String>,
  #[validate(length(min = 1))]
  pub target_base_ids: String,
  #[validate(range(min = 0))]
  pub user_role_weight: i32,
  pub user_id: Uuid,
  pub timestamp: DateTime<Utc>,
  pub origin_db_id: Uuid,
}

#[request_dto]
pub struct PushAuditLogsRequest {
  #[validate(length(min = 1))]
  pub logs: Vec<PushAuditLogRequest>,
}

impl From<AuditLogResponse> for PushAuditLogRequest {
  fn from(req: AuditLogResponse) -> Self {
    Self {
      id: req.id,
      table_name: req.table_name,
      record_id: req.record_id,
      action: req.action,
      old_values_json: req.old_values_json,
      new_values_json: req.new_values_json,
      target_base_ids: req.target_base_ids,
      user_role_weight: req.user_role_weight,
      user_id: req.user_id,
      timestamp: req.timestamp,
      origin_db_id: req.origin_db_id,
    }
  }
}
