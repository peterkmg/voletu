use uuid::Uuid;
use voletu_core_macros::request_dto;

use crate::{
  enums::{AuditTable, SyncDirection},
  services::sync::query::{
    AuditLogQuerySpec,
    OutboundAuditLogsQuerySpec,
    PullAuditLogsQuerySpec,
    SyncStatusQuerySpec,
  },
};

#[request_dto]
pub struct UpsertWatermarkRequest {
  pub target_node_id: Uuid,
  pub direction: SyncDirection,
  pub last_audit_log_id: Uuid,
  /// Canonical base discriminant to store alongside the cursor. Optional;
  /// defaults to empty string ("catalog-only scope") when omitted. This
  /// endpoint is a manual override - the normal pull path goes through
  /// `apply_pulled_logs`, which sets the discriminant atomically from the
  /// peripheral's actual assignments.
  #[serde(default)]
  pub base_discriminant: Option<String>,
}

fn parse_base_ids(base_ids: Option<&str>) -> Vec<Uuid> {
  base_ids
    .unwrap_or("")
    .split(',')
    .filter_map(|s| {
      let trimmed = s.trim();
      if trimmed.is_empty() {
        None
      } else {
        Uuid::try_parse(trimmed).ok()
      }
    })
    .collect()
}

#[request_dto]
pub struct AuditLogQueryRequest {
  pub table_name: Option<AuditTable>,
  pub record_id: Option<Uuid>,
  pub origin_db_id: Option<Uuid>,
  pub limit: Option<u64>,
  pub offset: Option<u64>,
}

impl From<AuditLogQueryRequest> for AuditLogQuerySpec {
  fn from(query: AuditLogQueryRequest) -> Self {
    Self {
      table_name: query.table_name,
      record_id: query.record_id,
      origin_db_id: query.origin_db_id,
      limit: query.limit,
      offset: query.offset,
    }
  }
}

#[request_dto]
pub struct OutboundLogsQueryRequest {
  pub after_audit_log_id: Uuid,
  pub limit: Option<u64>,
}

impl From<OutboundLogsQueryRequest> for OutboundAuditLogsQuerySpec {
  fn from(query: OutboundLogsQueryRequest) -> Self {
    Self::new(query.after_audit_log_id, query.limit)
  }
}

#[request_dto]
pub struct PullAuditLogsQueryRequest {
  pub last_audit_log_id: Uuid,
  /// Comma-separated base UUIDs the requesting node handles. Empty means
  /// catalog-only sync.
  #[serde(default)]
  pub base_ids: Option<String>,
  pub limit: Option<u64>,
}

impl PullAuditLogsQueryRequest {
  pub(crate) fn parse_base_ids(&self) -> Vec<Uuid> {
    parse_base_ids(self.base_ids.as_deref())
  }
}

impl From<PullAuditLogsQueryRequest> for PullAuditLogsQuerySpec {
  fn from(query: PullAuditLogsQueryRequest) -> Self {
    Self::new(query.last_audit_log_id, query.parse_base_ids(), query.limit)
  }
}

#[request_dto]
pub struct SyncStatusQueryRequest {
  /// Comma-separated base UUIDs the caller handles. Absent or empty means
  /// catalog-only scope.
  #[serde(default)]
  pub base_ids: Option<String>,
}

impl SyncStatusQueryRequest {
  pub(crate) fn parse_base_ids(&self) -> Vec<Uuid> {
    parse_base_ids(self.base_ids.as_deref())
  }
}

impl From<SyncStatusQueryRequest> for SyncStatusQuerySpec {
  fn from(query: SyncStatusQueryRequest) -> Self {
    Self::new(query.parse_base_ids())
  }
}

#[request_dto]
pub struct AwaitCycleQueryRequest {
  /// Maximum time to wait in seconds (default: 15, max: 60).
  #[validate(range(min = 1, max = 60))]
  pub timeout: Option<u64>,
  /// If provided, return immediately if last_sync_at is already after this
  /// timestamp. Format: RFC 3339 (e.g. "2026-01-01T00:00:00Z").
  pub since: Option<String>,
}

#[cfg(test)]
mod tests {
  use uuid::Uuid;

  use super::{AuditLogQueryRequest, PullAuditLogsQueryRequest, SyncStatusQueryRequest};
  use crate::{enums::AuditTable, services::sync::query::AuditLogQuerySpec};

  #[test]
  fn sync_query_requests_parse_comma_separated_base_ids() {
    let first = Uuid::nil();
    let second = Uuid::from_u128(1);

    let pull = PullAuditLogsQueryRequest {
      last_audit_log_id: Uuid::from_u128(2),
      base_ids: Some(format!("{first}, {second}, not-a-uuid, ")),
      limit: Some(10),
    };
    assert_eq!(pull.parse_base_ids(), vec![first, second]);

    let status = SyncStatusQueryRequest {
      base_ids: Some(format!(" , {second}")),
    };
    assert_eq!(status.parse_base_ids(), vec![second]);
  }

  #[test]
  fn audit_log_query_request_preserves_typed_table_filters() {
    let record_id = Uuid::now_v7();
    let origin_db_id = Uuid::now_v7();

    let spec: AuditLogQuerySpec = AuditLogQueryRequest {
      table_name: Some(AuditTable::DispatchDocuments),
      record_id: Some(record_id),
      origin_db_id: Some(origin_db_id),
      limit: Some(20),
      offset: Some(5),
    }
    .into();

    assert_eq!(spec.table_name, Some(AuditTable::DispatchDocuments));
    assert_eq!(spec.record_id, Some(record_id));
    assert_eq!(spec.origin_db_id, Some(origin_db_id));
    assert_eq!(spec.limit, Some(20));
    assert_eq!(spec.offset, Some(5));
  }
}
