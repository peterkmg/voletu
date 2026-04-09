use chrono::Utc;
use uuid::Uuid;
use voletu_core::{
  dtos::{AuditLogResponse, PushAuditLogRequest},
  entities::audit_log,
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
    "tableName": "dispatch_documents",
    "recordId": Uuid::max(),
    "action": "UPDATE",
    "oldValuesJson": null,
    "newValuesJson": null,
    "targetBaseIds": "",
    "userRoleWeight": 0,
    "userId": Uuid::nil(),
    "timestamp": Utc::now(),
    "originDbId": Uuid::max(),
  });

  let request: PushAuditLogRequest = serde_json::from_value(json).unwrap();
  assert_eq!(request.table_name, AuditTable::DispatchDocuments);
}

#[test]
fn audit_log_response_preserves_typed_table_name_from_model() {
  let row = audit_log::Model {
    id: Uuid::nil(),
    table_name: AuditTable::DispatchDocuments,
    record_id: Uuid::max(),
    action: AuditAction::Insert,
    old_values: None,
    new_values: None,
    target_base_ids: String::new(),
    user_role_weight: 7,
    user_id: Uuid::nil(),
    timestamp: Utc::now(),
    origin_db_id: Uuid::max(),
  };

  let response = AuditLogResponse::from(row);
  assert_eq!(response.table_name, AuditTable::DispatchDocuments);
}
