use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::enums::{
  AdjustmentType,
  DispatchMethod,
  DispatchPurpose,
  DocumentStatus,
  PipelineStatus,
};

#[response_dto]
pub struct TruckReceiptPipelineResponse {
  pub id: Uuid,
  pub basis_document_number: String,
  pub basis_date: String,
  pub contractor_id: Uuid,
  pub contractor_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub expected_quantity: Option<Decimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub action_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub action_document_number: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub actual_quantity: Option<Decimal>,
  pub pipeline_status: PipelineStatus,
}

#[response_dto]
pub struct RailReceiptPipelineResponse {
  pub id: Uuid,
  pub basis_document_number: String,
  pub basis_date: String,
  pub contractor_id: Uuid,
  pub contractor_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub expected_quantity: Option<Decimal>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub action_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub action_document_number: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub actual_quantity: Option<Decimal>,
  pub pipeline_status: PipelineStatus,
}

#[response_dto]
pub struct TruckDispatchPipelineResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub contractor_id: Uuid,
  pub contractor_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub dispatched_quantity: Option<Decimal>,
  pub pipeline_status: PipelineStatus,
}

// ---------------------------------------------------------------------------
// Flat (document + items) query responses
// ---------------------------------------------------------------------------

/// One row per acceptance item, with document fields repeated for grouping.
#[response_dto]
pub struct AcceptanceFlatRow {
  /// Row ID — equals document_id (for entity provider compatibility).
  pub id: Uuid,
  /// Document ID — used as groupKey on the frontend.
  pub document_id: Uuid,
  pub document_number: String,
  pub date_accepted: String,
  pub status: DocumentStatus,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub source_entity: Option<String>,
  // Item-level fields
  pub item_id: Uuid,
  pub product_id_name: String,
  pub storage_id_name: String,
  pub contractor_id_name: String,
  pub accepted_amount: Decimal,
}

/// One row per dispatch item, with document fields repeated for grouping.
#[response_dto]
pub struct DispatchFlatRow {
  /// Row ID — equals document_id (for entity provider compatibility).
  pub id: Uuid,
  /// Document ID — used as groupKey on the frontend.
  pub document_id: Uuid,
  pub document_number: String,
  pub date: String,
  pub status: DocumentStatus,
  pub dispatch_method: DispatchMethod,
  pub dispatch_purpose: DispatchPurpose,
  pub contractor_id_name: String,
  // Item-level fields
  pub item_id: Uuid,
  pub product_id_name: String,
  pub storage_id_name: String,
  pub dispatched_amount: Decimal,
}

/// One row per physical transfer item, with document fields repeated for grouping.
#[response_dto]
pub struct PhysicalTransferFlatRow {
  /// Row ID — equals document_id (for entity provider compatibility).
  pub id: Uuid,
  /// Document ID — used as groupKey on the frontend.
  pub document_id: Uuid,
  pub document_number: String,
  pub date: String,
  pub status: DocumentStatus,
  pub contractor_id_name: String,
  // Item-level fields
  pub item_id: Uuid,
  pub product_id_name: String,
  pub from_storage_id_name: String,
  pub to_storage_id_name: String,
  pub amount: Decimal,
}

/// One row per ownership transfer item, with document fields repeated for grouping.
#[response_dto]
pub struct OwnershipTransferFlatRow {
  pub id: Uuid,
  pub document_id: Uuid,
  pub date: String,
  pub status: DocumentStatus,
  // Item fields — ownership has from/to contractor at item level, not doc level
  pub item_id: Uuid,
  pub product_id_name: String,
  pub storage_id_name: String,
  pub from_contractor_id_name: String,
  pub to_contractor_id_name: String,
  pub amount: Decimal,
}

/// One row per blending component/result, with document fields repeated for grouping.
#[response_dto]
pub struct BlendingFlatRow {
  pub id: Uuid,
  pub document_id: Uuid,
  pub document_number: String,
  pub date: String,
  pub status: DocumentStatus,
  pub contractor_id_name: String,
  pub target_product_id_name: String,
  // Item fields
  pub item_id: Uuid,
  pub item_type: String, // "component" or "result"
  pub product_id_name: String,
  pub storage_id_name: String,
  pub amount: Decimal,
}

/// One row per reconciliation adjustment, with document fields repeated for grouping.
#[response_dto]
pub struct ReconciliationFlatRow {
  pub id: Uuid,
  pub document_id: Uuid,
  pub document_number: String,
  pub date: String,
  pub status: DocumentStatus,
  pub contractor_id_name: String,
  pub warehouse_id_name: String,
  // Item fields
  pub item_id: Uuid,
  pub product_id_name: String,
  pub storage_id_name: String,
  pub adjustment_type: AdjustmentType,
  pub amount: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub reason: Option<String>,
}

// ---------------------------------------------------------------------------
// Cargo flow aggregate (union of all document types)
// ---------------------------------------------------------------------------

/// Normalized cargo flow row — unions all document types with items.
#[response_dto]
pub struct CargoFlowFlatRow {
  pub id: Uuid,
  pub document_id: Uuid,
  pub document_number: String,
  pub date: String,
  #[serde(rename = "type")]
  pub flow_type: String, // "Incoming" | "Outgoing" | "Internal"
  pub operation: String, // "Truck Receipt" | "Blending" | etc.
  pub contractor_name: String,
  pub status: String,     // Keep as string for mixed status types
  pub flow_route: String, // Frontend navigation path
  // Item-level fields
  pub product_name: String,
  pub storage_name: String, // "Tank A" or "Tank A → Tank B" for transfers
  pub quantity: String,     // Formatted decimal
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub item_type: Option<String>, // "component"/"result" for blending, "surplus"/"loss" for reconciliation
}
