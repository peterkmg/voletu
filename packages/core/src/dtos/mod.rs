pub mod request;
pub mod response;
pub mod validators;

/*
DTO macro contracts:

- #[request_dto]
  Adds derives: Debug, Deserialize, Serialize, Validate, ToSchema.
  Adds serde rename_all = "camelCase".
  For Decimal fields it applies string serde/schema representation.
  For request strings with length validation it also enforces non-blank validation.
  Parameters: none.

- #[response_dto]
  Adds derives: Debug, Serialize, ToSchema.
  Adds serde rename_all = "camelCase".
  Supports optional service field generation only when explicitly requested.
  Parameters:
  - no params: do not inject any service fields.
  - service_fields(common): inject created/updated/deleted timestamps and actors + origin_db_id.
  - service_fields(document): inject common fields + status + executed/reverted lifecycle fields.
  - service_fields(all): inject document fields + version.
  - service_fields(field_a, field_b, ...): inject only listed supported service fields.
  - service_fields(): explicit no-op (equivalent to no params).
*/

pub use request::{
  audit::{PushAuditLogRequest, PushAuditLogsRequest},
  catalog::{
    CreateBaseRequest,
    CreateCompanyRequest,
    CreatePortRequest,
    CreateProductGroupRequest,
    CreateProductRequest,
    CreateProductTypeRequest,
    CreateStorageRequest,
    CreateWarehouseRequest,
    UpdateBaseRequest,
    UpdateCompanyRequest,
    UpdatePortRequest,
    UpdateProductGroupRequest,
    UpdateProductRequest,
    UpdateProductTypeRequest,
    UpdateStorageRequest,
    UpdateWarehouseRequest,
  },
  document::{
    AcceptanceItemCompositeRequest,
    BlendingComponentCompositeRequest,
    BlendingResultCompositeRequest,
    CreateAcceptanceCompositeRequest,
    CreateAcceptanceItemRequest,
    CreateAcceptanceRequest,
    CreateBlendingComponentRequest,
    CreateBlendingCompositeRequest,
    CreateBlendingRequest,
    CreateBlendingResultRequest,
    CreateDispatchCompositeRequest,
    CreateDispatchItemRequest,
    CreateDispatchMeasurementRequest,
    CreateDispatchRequest,
    CreateInventoryAdjustmentRequest,
    CreateInventoryReconciliationRequest,
    CreateOwnershipTransferItemRequest,
    CreateOwnershipTransferRequest,
    CreatePhysicalTransferItemRequest,
    CreatePhysicalTransferRequest,
    CreateRailWagonManifestRequest,
    CreateRailWagonMeasurementRequest,
    CreateRailWagonWeightRequest,
    CreateRailWaybillRequest,
    CreateTruckWaybillItemRequest,
    CreateTruckWaybillRequest,
    CreateTruckWeightDocRequest,
    DispatchItemCompositeRequest,
    DispatchMeasurementCompositeRequest,
    OwnershipTransferItemCompositeRequest,
    PhysicalTransferItemCompositeRequest,
    RailWagonManifestCompositeRequest,
    RailWagonMeasurementCompositeRequest,
    RailWagonWeightCompositeRequest,
    RailWaybillCompositeRequest,
    TransportAcceptanceCompositeRequest,
    TruckWaybillCompositeRequest,
    TruckWaybillItemCompositeRequest,
    TruckWeightDocCompositeRequest,
    UpdateAcceptanceItemRequest,
    UpdateAcceptanceRequest,
    UpdateBlendingComponentRequest,
    UpdateBlendingRequest,
    UpdateBlendingResultRequest,
    UpdateDispatchItemRequest,
    UpdateDispatchMeasurementRequest,
    UpdateDispatchRequest,
    UpdateInventoryAdjustmentRequest,
    UpdateInventoryReconciliationRequest,
    UpdateOwnershipTransferItemRequest,
    UpdateOwnershipTransferRequest,
    UpdatePhysicalTransferItemRequest,
    UpdatePhysicalTransferRequest,
    UpdateRailWagonManifestRequest,
    UpdateRailWagonMeasurementRequest,
    UpdateRailWagonWeightRequest,
    UpdateRailWaybillRequest,
    UpdateTruckWaybillItemRequest,
    UpdateTruckWaybillRequest,
    UpdateTruckWeightDocRequest,
  },
  ledger::LedgerEntryLookupRequest,
  sync::{
    AwaitCycleQueryRequest,
    OutboundLogsQueryRequest,
    PullAuditLogsQueryRequest,
    SyncStatusQueryRequest,
    UpsertWatermarkRequest,
  },
  system::{
    AddBaseAssignmentRequest,
    ChangePasswordRequest,
    CompleteInitializationRequest,
    CreateUserRequest,
    LoginRequest,
    RefreshTokenRequest,
    UpdateUserRequest,
  },
};
pub use response::{
  audit::AuditLogResponse,
  catalog::{
    BaseResponse,
    CompanyResponse,
    PortResponse,
    ProductGroupResponse,
    ProductResponse,
    ProductTypeResponse,
    StorageResponse,
    WarehouseResponse,
  },
  document::{
    AcceptanceCompositeResponse,
    AcceptanceItemResponse,
    AcceptanceResponse,
    BlendingComponentResponse,
    BlendingCompositeResponse,
    BlendingResponse,
    BlendingResultResponse,
    DispatchCompositeResponse,
    DispatchItemResponse,
    DispatchMeasurementResponse,
    DispatchResponse,
    InventoryAdjustmentResponse,
    InventoryReconciliationResponse,
    OwnershipTransferItemResponse,
    OwnershipTransferResponse,
    PhysicalTransferItemResponse,
    PhysicalTransferResponse,
    RailWagonManifestResponse,
    RailWagonMeasurementResponse,
    RailWagonWeightResponse,
    RailWaybillCompositeResponse,
    RailWaybillResponse,
    TruckWaybillCompositeResponse,
    TruckWaybillItemResponse,
    TruckWaybillResponse,
    TruckWeightDocResponse,
  },
  ledger::LedgerEntryResponse,
  pipeline::{
    RailReceiptPipelineResponse,
    TruckDispatchPipelineResponse,
    TruckReceiptPipelineResponse,
  },
  sync::{
    AwaitCycleResponse,
    PullAuditLogsResponse,
    PushAuditLogsResponse,
    SyncStatusResponse,
    SyncWatermarkResponse,
  },
  system::{
    BaseAssignmentResponse,
    DatabaseInstanceResponse,
    HealthData,
    LocalResponse,
    LoginResponse,
    NodeStatusResponse,
    OperationMessageResponse,
    RefreshTokenResponse,
    RoleResponse,
    UserResponse,
  },
};

#[cfg(test)]
mod tests {
  use uuid::Uuid;

  #[test]
  fn shared_transport_dtos_are_exported_from_the_dto_surface() {
    let _ = crate::dtos::AddBaseAssignmentRequest {
      base_id: Uuid::nil(),
    };
    let _ = crate::dtos::LedgerEntryLookupRequest {
      storage_id: Uuid::nil(),
      product_id: Uuid::nil(),
      contractor_id: Uuid::nil(),
    };
    let _ = crate::dtos::UpsertWatermarkRequest {
      target_node_id: Uuid::nil(),
      direction: crate::enums::SyncDirection::Push,
      last_audit_log_id: Uuid::nil(),
      base_discriminant: None,
    };
    let _ = crate::dtos::OutboundLogsQueryRequest {
      after_audit_log_id: Uuid::nil(),
      limit: Some(50),
    };
    let _ = crate::dtos::PullAuditLogsQueryRequest {
      last_audit_log_id: Uuid::nil(),
      base_ids: Some(format!("{},{}", Uuid::nil(), Uuid::nil())),
      limit: Some(25),
    };
    let _ = crate::dtos::SyncStatusQueryRequest {
      base_ids: Some(Uuid::nil().to_string()),
    };
    let _ = crate::dtos::AwaitCycleQueryRequest {
      timeout: Some(15),
      since: Some("2026-01-01T00:00:00Z".into()),
    };
    let _ = crate::dtos::AwaitCycleResponse {
      worker_state: "OnlineIdle".into(),
      last_sync_at: Some("2026-01-01T00:00:00Z".into()),
      completed: true,
    };
    let _ = crate::dtos::OperationMessageResponse {
      message: "ok".into(),
    };
    let _ = crate::dtos::HealthData {
      status: "ok".into(),
      is_initialized: true,
      node_type: "CENTRAL".into(),
      node_name: "Node".into(),
    };
    let _ = crate::dtos::NodeStatusResponse {
      is_initialized: true,
      node_type: "PERIPHERAL".into(),
      node_name: "Node".into(),
      worker_state: "OnlineIdle".into(),
      last_sync_at: None,
    };
  }
}
