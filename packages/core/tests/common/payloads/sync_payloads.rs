use serde_json::json;
use uuid::Uuid;

pub fn sync_push_invalid_action(
  record_id: Uuid,
  target_base_id: Uuid,
  user_id: Uuid,
  origin_db_id: Uuid,
) -> String {
  json!({
    "logs": [
      {
        "id": Uuid::now_v7(),
        "tableName": "companies",
        "recordId": record_id,
        "action": "BAD_ACTION",
        "oldValuesJson": null,
        "newValuesJson": "{}",
        "targetBaseIds": target_base_id.to_string(),
        "userRoleWeight": 1,
        "userId": user_id,
        "timestamp": "2026-01-01T00:00:00Z",
        "originDbId": origin_db_id,
      }
    ]
  })
  .to_string()
}

pub fn sync_push_insert_company(
  log_id: Uuid,
  record_id: Uuid,
  target_base_id: Uuid,
  user_id: Uuid,
  origin_db_id: Uuid,
  common_name: &str,
) -> String {
  let new_values_json = json!({
    "id": record_id,
    "common_name": common_name,
    "legal_name": null,
    "is_contractor": true,
    "is_exporter": false,
    "is_manufacturer": false,
    "is_sender": false,
    "created_at": "2026-01-01T00:00:00Z",
    "updated_at": "2026-01-01T00:00:00Z",
    "deleted_at": null,
    "created_by": user_id,
    "updated_by": user_id,
    "deleted_by": null,
    "origin_db_id": origin_db_id,
  })
  .to_string();

  json!({
    "logs": [
      {
        "id": log_id,
        "tableName": "companies",
        "recordId": record_id,
        "action": "INSERT",
        "oldValuesJson": null,
        "newValuesJson": new_values_json,
        "targetBaseIds": target_base_id.to_string(),
        "userRoleWeight": 40,
        "userId": user_id,
        "timestamp": "2026-01-01T00:00:00Z",
        "originDbId": origin_db_id,
      }
    ]
  })
  .to_string()
}

pub fn sync_watermark_upsert(
  target_node_id: Uuid,
  direction: &str,
  last_audit_log_id: Uuid,
) -> String {
  json!({
    "targetNodeId": target_node_id,
    "direction": direction,
    "lastAuditLogId": last_audit_log_id,
  })
  .to_string()
}

pub fn node_initialize_replace(new_username: &str, new_password: &str, fullname: &str) -> String {
  json!({
    "action": "REPLACE",
    "newUsername": new_username,
    "newPassword": new_password,
    "fullname": fullname,
  })
  .to_string()
}

pub fn node_initialize_replace_with_node_type(
  new_username: &str,
  new_password: &str,
  fullname: &str,
  node_type: &str,
) -> String {
  json!({
    "action": "REPLACE",
    "newUsername": new_username,
    "newPassword": new_password,
    "fullname": fullname,
    "nodeType": node_type,
  })
  .to_string()
}
