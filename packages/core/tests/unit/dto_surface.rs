use uuid::Uuid;
use voletu_core::{
  dtos::{
    AcceptanceDocumentQueryParams,
    AuditLogQueryRequest,
    AwaitCycleQueryRequest,
    AwaitCycleResponse,
    HealthData,
    LedgerBalanceLookupRequest,
    NodeStatusResponse,
    OperationMessageResponse,
    OutboundLogsQueryRequest,
    PaginationParams,
    PullAuditLogsQueryRequest,
    SeedResult,
    SyncStatusQueryRequest,
    SyncStatusResponse,
    UpsertWatermarkRequest,
  },
  enums::{AuditTable, DocumentStatus, SyncDirection},
};

#[test]
fn transport_dtos_are_accessible_at_crate_surface() {
  let _ = voletu_core::dtos::AddBaseAssignmentRequest {
    base_id: Uuid::nil(),
  };
  let _ = LedgerBalanceLookupRequest {
    storage_id: Uuid::nil(),
    product_id: Uuid::nil(),
    contractor_id: Uuid::nil(),
  };
  let _ = UpsertWatermarkRequest {
    target_node_id: Uuid::nil(),
    direction: SyncDirection::Push,
    last_audit_log_id: Uuid::nil(),
    base_discriminant: None,
  };
  let _ = OutboundLogsQueryRequest {
    after_audit_log_id: Uuid::nil(),
    limit: Some(50),
  };
  let _ = PullAuditLogsQueryRequest {
    last_audit_log_id: Uuid::nil(),
    base_ids: Some(format!("{},{}", Uuid::nil(), Uuid::nil())),
    limit: Some(25),
  };
  let _ = SyncStatusQueryRequest {
    base_ids: Some(Uuid::nil().to_string()),
  };
  let _ = AuditLogQueryRequest {
    table_name: Some(AuditTable::Companies),
    record_id: Some(Uuid::nil()),
    origin_db_id: Some(Uuid::nil()),
    limit: Some(10),
    offset: Some(5),
  };
  let _ = PaginationParams {
    page: Some(1),
    per_page: Some(25),
  };
  let _ = AcceptanceDocumentQueryParams {
    document_number: Some("ACC-1".into()),
    status: Some(DocumentStatus::Draft),
    truck_waybill_id: None,
    rail_waybill_id: None,
    transit_dispatch_id: None,
    pagination: PaginationParams {
      page: Some(1),
      per_page: Some(25),
    },
  };
  let _ = AwaitCycleQueryRequest {
    timeout: Some(15),
    since: Some("2026-01-01T00:00:00Z".into()),
  };
  let _ = AwaitCycleResponse {
    worker_state: "OnlineIdle".into(),
    last_sync_at: Some("2026-01-01T00:00:00Z".into()),
    completed: true,
  };
  let _ = SyncStatusResponse {
    node_id: Uuid::nil(),
    node_type: "CENTRAL".into(),
    highest_audit_log_id: Uuid::nil(),
    highest_matching_id: Uuid::nil(),
  };
  let _ = OperationMessageResponse {
    message: "ok".into(),
  };
  let _ = HealthData {
    status: "ok".into(),
    is_initialized: true,
    node_type: "CENTRAL".into(),
    node_name: "Node".into(),
  };
  let _ = NodeStatusResponse {
    is_initialized: true,
    node_type: "PERIPHERAL".into(),
    node_name: "Node".into(),
    worker_state: "OnlineIdle".into(),
    last_sync_at: None,
    central_api_url: None,
    assigned_base_ids: vec![Uuid::now_v7()],
  };
  let _ = SeedResult {
    product_types: 1,
    product_groups: 2,
    products: 3,
    companies: 4,
    ports: 5,
    bases: 6,
    warehouses: 7,
    storages: 8,
    users: 9,
    truck_waybills: 10,
    rail_waybills: 11,
    acceptance_docs: 12,
    dispatch_docs: 13,
    blending_docs: 14,
    ownership_transfers: 15,
    physical_transfers: 16,
    reconciliations: 17,
    ledger_entries: 18,
  };
}
