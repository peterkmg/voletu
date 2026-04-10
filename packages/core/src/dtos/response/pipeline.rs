use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::{
  entities::{
    acceptance_document,
    acceptance_item,
    dispatch_document,
    dispatch_item,
    ownership_transfer,
    ownership_transfer_item,
    physical_storage_transfer,
    physical_transfer_item,
  },
  enums::{AdjustmentType, DispatchMethod, DispatchPurpose, DocumentStatus, PipelineStatus},
};

const EMPTY_LABEL: &str = "\u{2014}";

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

pub struct AcceptanceFlatRowRef<'a> {
  pub document: &'a acceptance_document::ModelEx,
  pub item: Option<&'a acceptance_item::ModelEx>,
}

impl From<AcceptanceFlatRowRef<'_>> for AcceptanceFlatRow {
  fn from(value: AcceptanceFlatRowRef<'_>) -> Self {
    let contractor_id_name = value
      .document
      .contractor
      .as_ref()
      .map(|contractor| contractor.common_name.clone())
      .unwrap_or_else(|| EMPTY_LABEL.to_string());

    match value.item {
      Some(item) => Self {
        id: value.document.id,
        document_id: value.document.id,
        document_number: value.document.document_number.clone(),
        date_accepted: value.document.date_accepted.to_string(),
        status: value.document.status,
        source_entity: value.document.source_entity.clone(),
        item_id: item.id,
        product_id_name: item
          .product
          .as_ref()
          .map(|product| product.common_name.clone())
          .unwrap_or_default(),
        storage_id_name: item
          .storage
          .as_ref()
          .map(|storage| storage.common_name.clone())
          .unwrap_or_default(),
        contractor_id_name,
        accepted_amount: item.accepted_amount,
      },
      None => Self {
        id: value.document.id,
        document_id: value.document.id,
        document_number: value.document.document_number.clone(),
        date_accepted: value.document.date_accepted.to_string(),
        status: value.document.status,
        source_entity: value.document.source_entity.clone(),
        item_id: value.document.id,
        product_id_name: EMPTY_LABEL.to_string(),
        storage_id_name: EMPTY_LABEL.to_string(),
        contractor_id_name,
        accepted_amount: Default::default(),
      },
    }
  }
}

pub struct DispatchFlatRowRef<'a> {
  pub document: &'a dispatch_document::ModelEx,
  pub item: Option<&'a dispatch_item::ModelEx>,
}

impl From<DispatchFlatRowRef<'_>> for DispatchFlatRow {
  fn from(value: DispatchFlatRowRef<'_>) -> Self {
    let contractor_id_name = value
      .document
      .contractor
      .as_ref()
      .map(|contractor| contractor.common_name.clone())
      .unwrap_or_else(|| EMPTY_LABEL.to_string());

    match value.item {
      Some(item) => Self {
        id: value.document.id,
        document_id: value.document.id,
        document_number: value.document.document_number.clone(),
        date: value.document.date.to_string(),
        status: value.document.status,
        dispatch_method: value.document.dispatch_method,
        dispatch_purpose: value.document.dispatch_purpose,
        contractor_id_name,
        item_id: item.id,
        product_id_name: item
          .product
          .as_ref()
          .map(|product| product.common_name.clone())
          .unwrap_or_default(),
        storage_id_name: item
          .storage
          .as_ref()
          .map(|storage| storage.common_name.clone())
          .unwrap_or_default(),
        dispatched_amount: item.dispatched_amount,
      },
      None => Self {
        id: value.document.id,
        document_id: value.document.id,
        document_number: value.document.document_number.clone(),
        date: value.document.date.to_string(),
        status: value.document.status,
        dispatch_method: value.document.dispatch_method,
        dispatch_purpose: value.document.dispatch_purpose,
        contractor_id_name,
        item_id: value.document.id,
        product_id_name: EMPTY_LABEL.to_string(),
        storage_id_name: EMPTY_LABEL.to_string(),
        dispatched_amount: Default::default(),
      },
    }
  }
}

pub struct PhysicalTransferFlatRowRef<'a> {
  pub document: &'a physical_storage_transfer::ModelEx,
  pub item: Option<&'a physical_transfer_item::ModelEx>,
  pub contractor_id_name: &'a str,
  pub to_storage_id_name: &'a str,
}

impl From<PhysicalTransferFlatRowRef<'_>> for PhysicalTransferFlatRow {
  fn from(value: PhysicalTransferFlatRowRef<'_>) -> Self {
    match value.item {
      Some(item) => Self {
        id: value.document.id,
        document_id: value.document.id,
        document_number: value.document.document_number.clone(),
        date: value.document.date.to_string(),
        status: value.document.status,
        contractor_id_name: value.contractor_id_name.to_string(),
        item_id: item.id,
        product_id_name: item
          .product
          .as_ref()
          .map(|product| product.common_name.clone())
          .unwrap_or_default(),
        from_storage_id_name: item
          .from_storage
          .as_ref()
          .map(|storage| storage.common_name.clone())
          .unwrap_or_default(),
        to_storage_id_name: value.to_storage_id_name.to_string(),
        amount: item.amount,
      },
      None => Self {
        id: value.document.id,
        document_id: value.document.id,
        document_number: value.document.document_number.clone(),
        date: value.document.date.to_string(),
        status: value.document.status,
        contractor_id_name: value.contractor_id_name.to_string(),
        item_id: value.document.id,
        product_id_name: EMPTY_LABEL.to_string(),
        from_storage_id_name: EMPTY_LABEL.to_string(),
        to_storage_id_name: EMPTY_LABEL.to_string(),
        amount: Default::default(),
      },
    }
  }
}

pub struct OwnershipTransferFlatRowRef<'a> {
  pub document: &'a ownership_transfer::ModelEx,
  pub item: Option<&'a ownership_transfer_item::ModelEx>,
  pub from_contractor_id_name: &'a str,
  pub to_contractor_id_name: &'a str,
}

impl From<OwnershipTransferFlatRowRef<'_>> for OwnershipTransferFlatRow {
  fn from(value: OwnershipTransferFlatRowRef<'_>) -> Self {
    match value.item {
      Some(item) => Self {
        id: value.document.id,
        document_id: value.document.id,
        date: value.document.date.to_string(),
        status: value.document.status,
        item_id: item.id,
        product_id_name: item
          .product
          .as_ref()
          .map(|product| product.common_name.clone())
          .unwrap_or_default(),
        storage_id_name: item
          .storage
          .as_ref()
          .map(|storage| storage.common_name.clone())
          .unwrap_or_default(),
        from_contractor_id_name: value.from_contractor_id_name.to_string(),
        to_contractor_id_name: value.to_contractor_id_name.to_string(),
        amount: item.amount,
      },
      None => Self {
        id: value.document.id,
        document_id: value.document.id,
        date: value.document.date.to_string(),
        status: value.document.status,
        item_id: value.document.id,
        product_id_name: EMPTY_LABEL.to_string(),
        storage_id_name: EMPTY_LABEL.to_string(),
        from_contractor_id_name: EMPTY_LABEL.to_string(),
        to_contractor_id_name: EMPTY_LABEL.to_string(),
        amount: Default::default(),
      },
    }
  }
}
