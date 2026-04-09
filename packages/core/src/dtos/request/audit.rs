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

#[cfg(test)]
mod tests {
  use chrono::Utc;
  use uuid::Uuid;

  use super::PushAuditLogRequest;
  use crate::{
    dtos::AuditLogResponse,
    enums::{AuditAction, AuditTable},
  };

  #[test]
  fn push_audit_log_request_preserves_typed_table_name_from_response() {
    let response = AuditLogResponse {
      id: Uuid::nil(),
      table_name: AuditTable::DispatchDocuments,
      record_id: Uuid::max(),
      action: AuditAction::Update,
      old_values_json: None,
      new_values_json: None,
      target_base_ids: String::new(),
      user_role_weight: 2,
      user_id: Uuid::nil(),
      timestamp: Utc::now(),
      origin_db_id: Uuid::max(),
    };

    let request = PushAuditLogRequest::from(response);
    assert_eq!(request.table_name, AuditTable::DispatchDocuments);
  }

  #[test]
  fn push_audit_log_request_deserializes_table_name_from_snake_case() {
    let json = serde_json::json!({
      "id": Uuid::nil(),
      "table_name": "dispatch_documents",
      "record_id": Uuid::max(),
      "action": "UPDATE",
      "old_values_json": null,
      "new_values_json": null,
      "target_base_ids": "",
      "user_role_weight": 0,
      "user_id": Uuid::nil(),
      "timestamp": Utc::now(),
      "origin_db_id": Uuid::max(),
    });

    let request: PushAuditLogRequest = serde_json::from_value(json).unwrap();
    assert_eq!(request.table_name, AuditTable::DispatchDocuments);
  }
}
