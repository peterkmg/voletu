use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::enums::{FlowEntityType, FlowOperation, FlowType, PipelineStatus};

/// A single row in the truck-receipt flow view.
///
/// Joins a truck waybill (the "basis") with its linked acceptance document
/// (the "action"), if one exists, and computes a `pipeline_status`.
#[response_dto]
pub struct TruckReceiptFlowRow {
  // -- Basis (truck waybill) -----------------
  pub basis_id: Uuid,
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
  // -- Action (acceptance document) ----------
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub action_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub action_document_number: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub actual_quantity: Option<Decimal>,
  // -- Computed ------------------------------
  pub pipeline_status: PipelineStatus,
}

/// A single row in the rail-receipt flow view.
///
/// Joins a rail waybill (the "basis") with its linked acceptance document
/// (the "action"), if one exists, and computes a `pipeline_status`.
#[response_dto]
pub struct RailReceiptFlowRow {
  // -- Basis (rail waybill) ------------------
  pub basis_id: Uuid,
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
  // -- Action (acceptance document) ----------
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub action_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub action_document_number: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub actual_quantity: Option<Decimal>,
  // -- Computed ------------------------------
  pub pipeline_status: PipelineStatus,
}

/// A unified row for the cargo flow aggregate view.
/// Contains common fields projected from all document types.
///
/// NOTE: This endpoint fetches all matching documents into memory, sorts by
/// date, and slices for pagination. This is a known limitation due to the
/// UNION ALL nature of the query across heterogeneous entity types. For large
/// datasets, consider creating a database view or using raw SQL with
/// server-side pagination.
#[response_dto]
#[derive(Clone)]
pub struct CargoFlowRow {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub flow_type: FlowType,
  pub operation: FlowOperation,
  pub contractor_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub contractor_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub quantity: Option<Decimal>,
  pub status: PipelineStatus,
  pub entity_type: FlowEntityType,
}

/// A single row in the truck-dispatch flow view.
///
/// Shows dispatch documents with `dispatch_method = Truck` and a computed
/// `pipeline_status` based on the document's own status.
#[response_dto]
pub struct TruckDispatchFlowRow {
  pub dispatch_id: Uuid,
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
  // -- Computed ------------------------------
  pub pipeline_status: PipelineStatus,
}
