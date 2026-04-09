use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::{
  entities::audit_log,
  enums::{AuditAction, AuditTable},
};

/// Response DTO for the `audit_log` entity.
#[response_dto(service_fields(origin_db_id))]
#[derive(Deserialize)]
pub struct AuditLogResponse {
  pub id: Uuid,
  pub table_name: AuditTable,
  pub record_id: Uuid,
  pub action: AuditAction,
  pub old_values_json: Option<String>,
  pub new_values_json: Option<String>,
  pub target_base_ids: String,
  pub user_role_weight: i32,
  pub user_id: Uuid,
  pub timestamp: DateTime<Utc>,
}

impl From<audit_log::Model> for AuditLogResponse {
  fn from(row: audit_log::Model) -> Self {
    Self {
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
}

impl From<audit_log::ModelEx> for AuditLogResponse {
  fn from(row: audit_log::ModelEx) -> Self {
    Self {
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
}

#[cfg(test)]
mod tests {
  use chrono::Utc;
  use uuid::Uuid;

  use super::AuditLogResponse;
  use crate::{
    entities::audit_log,
    enums::{AuditAction, AuditTable},
  };

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
}
