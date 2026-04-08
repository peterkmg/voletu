use serde::Deserialize;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::{entities::sync_watermark, enums::SyncDirection};

/// Functional DTO describing node sync progress summary.
#[response_dto]
#[derive(Deserialize)]
pub struct SyncStatusResponse {
  pub node_id: Uuid,
  pub node_type: String,
  /// Highest audit_log id in the node's entire table, regardless of scope.
  /// Used for diagnostics and "is the peer alive" liveness checks.
  pub highest_audit_log_id: Uuid,
  /// Highest audit_log id that matches the caller's requested scope (global
  /// tables OR logs targeted for any of the provided base_ids). When the
  /// caller sends no baseIds this equals the max id of any global-table log.
  /// The worker's `has_updates` decision compares THIS field — not
  /// `highest_audit_log_id` — against its PULL cursor, so a peripheral does
  /// not hot-poll Central when the only new activity is on bases it does
  /// not serve.
  pub highest_matching_id: Uuid,
}

/// Response DTO for the `sync_watermark` entity.
#[response_dto]
#[derive(Deserialize)]
pub struct SyncWatermarkResponse {
  pub id: Uuid,
  pub target_node_id: Uuid,
  pub direction: SyncDirection,
  pub last_audit_log_id: Uuid,
  /// Canonical string of base UUIDs that were assigned to this node when the
  /// watermark was last written. Empty means catalog-only scope. See
  /// `docs/Sync.md` for semantics.
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

/// Functional DTO summarizing push synchronization results.
#[response_dto]
#[derive(Deserialize)]
pub struct PushAuditLogsResponse {
  pub accepted: u64,
  pub rejected: u64,
}

/// Functional DTO carrying pulled audit logs and cursor metadata.
#[response_dto]
#[derive(Deserialize)]
pub struct PullAuditLogsResponse {
  pub highest_evaluated_id: Uuid,
  pub logs: Vec<crate::dtos::AuditLogResponse>,
}
