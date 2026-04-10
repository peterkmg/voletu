use uuid::Uuid;
use voletu_core::{
  dtos::{AuditLogQueryRequest, PullAuditLogsQueryRequest, SyncStatusQueryRequest},
  enums::AuditTable,
  services::sync::specs::{AuditLogQuerySpec, PullAuditLogsQuerySpec, SyncStatusQuerySpec},
};

#[test]
fn sync_query_requests_parse_comma_separated_base_ids_via_public_specs() {
  let first = Uuid::nil();
  let second = Uuid::from_u128(1);

  let pull_spec: PullAuditLogsQuerySpec = PullAuditLogsQueryRequest {
    last_audit_log_id: Uuid::from_u128(2),
    base_ids: Some(format!("{first}, {second}, not-a-uuid, ")),
    limit: Some(10),
  }
  .into();
  assert_eq!(pull_spec.base_ids, vec![first, second]);

  let status_spec: SyncStatusQuerySpec = SyncStatusQueryRequest {
    base_ids: Some(format!(" , {second}")),
  }
  .into();
  assert_eq!(status_spec.base_ids, vec![second]);
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
