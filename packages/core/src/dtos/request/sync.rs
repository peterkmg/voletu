use chrono::{DateTime, Utc};
use uuid::Uuid;
use voletu_core_macros::request_dto;

use crate::{
  enums::{AuditTable, SyncDirection},
  services::sync::specs::{
    AuditLogQuerySpec,
    AwaitCycleQuerySpec,
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
  #[validate(range(min = 1, max = 60))]
  pub timeout: Option<u64>,
  pub since: Option<String>,
}

impl AwaitCycleQueryRequest {
  fn parse_since(&self) -> Option<DateTime<Utc>> {
    self.since.as_deref().and_then(|value| {
      DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
    })
  }
}

impl From<AwaitCycleQueryRequest> for AwaitCycleQuerySpec {
  fn from(query: AwaitCycleQueryRequest) -> Self {
    Self::new(query.timeout.unwrap_or(15), query.parse_since())
  }
}
