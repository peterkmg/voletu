use serde::Deserialize;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::{dtos::AuditLogResponse, entities::sync_watermark, enums::SyncDirection};

#[response_dto]
#[derive(Deserialize)]
pub struct SyncStatusResponse {
  pub node_id: Uuid,
  pub node_type: String,
  pub highest_audit_log_id: Uuid,
  pub highest_matching_id: Uuid,
}

#[response_dto]
#[derive(Deserialize)]
pub struct SyncWatermarkResponse {
  pub id: Uuid,
  pub target_node_id: Uuid,
  pub direction: SyncDirection,
  pub last_audit_log_id: Uuid,
  pub base_discriminant: String,
  pub synced_at: String,
}

impl From<sync_watermark::Model> for SyncWatermarkResponse {
  fn from(row: sync_watermark::Model) -> Self {
    Self {
      id: row.id,
      target_node_id: row.target_node_id,
      direction: row.direction,
      last_audit_log_id: row.last_audit_log_id,
      base_discriminant: row.base_discriminant,
      synced_at: row.synced_at.to_rfc3339(),
    }
  }
}

impl From<sync_watermark::ModelEx> for SyncWatermarkResponse {
  fn from(row: sync_watermark::ModelEx) -> Self {
    Self {
      id: row.id,
      target_node_id: row.target_node_id,
      direction: row.direction,
      last_audit_log_id: row.last_audit_log_id,
      base_discriminant: row.base_discriminant,
      synced_at: row.synced_at.to_rfc3339(),
    }
  }
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

#[response_dto]
pub struct AwaitCycleResponse {
  pub worker_state: String,
  pub last_sync_at: Option<String>,
  pub completed: bool,
}
