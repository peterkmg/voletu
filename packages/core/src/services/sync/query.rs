use uuid::Uuid;

use crate::enums::AuditTable;

#[derive(Debug, Clone, Default)]
pub struct AuditLogQuerySpec {
  pub table_name: Option<AuditTable>,
  pub record_id: Option<Uuid>,
  pub origin_db_id: Option<Uuid>,
  pub limit: Option<u64>,
  pub offset: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct OutboundAuditLogsQuerySpec {
  pub after_audit_log_id: Uuid,
  pub limit: Option<u64>,
}

impl OutboundAuditLogsQuerySpec {
  pub fn new(after_audit_log_id: Uuid, limit: Option<u64>) -> Self {
    Self {
      after_audit_log_id,
      limit,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PullAuditLogsQuerySpec {
  pub last_audit_log_id: Uuid,
  pub base_ids: Vec<Uuid>,
  pub limit: Option<u64>,
}

impl PullAuditLogsQuerySpec {
  pub fn new(last_audit_log_id: Uuid, base_ids: Vec<Uuid>, limit: Option<u64>) -> Self {
    Self {
      last_audit_log_id,
      base_ids,
      limit,
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct SyncStatusQuerySpec {
  pub base_ids: Vec<Uuid>,
}

impl SyncStatusQuerySpec {
  pub fn new(base_ids: Vec<Uuid>) -> Self {
    Self { base_ids }
  }
}
