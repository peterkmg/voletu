use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::enums::AdjustmentType;

#[response_dto]
pub struct PhysicalTransferResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub start_cargo_ops: String,
  pub end_cargo_ops: String,
  pub contractor_id: Uuid,
  pub product_id: Uuid,
  pub from_storage_id: Uuid,
  pub to_storage_id: Uuid,
  pub amount_transferred: Decimal,
}

#[response_dto]
pub struct OwnershipTransferResponse {
  pub id: Uuid,
  pub date: String,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub from_contractor_id: Uuid,
  pub to_contractor_id: Uuid,
  pub amount_transferred: Decimal,
}

#[response_dto]
pub struct BlendingResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub contractor_id: Uuid,
  pub target_product_id: Uuid,
}

#[response_dto]
pub struct BlendingComponentResponse {
  pub id: Uuid,
  pub blending_doc_id: Uuid,
  pub storage_id: Uuid,
  pub source_product_id: Uuid,
  pub amount_used: Decimal,
}

#[response_dto]
pub struct BlendingResultResponse {
  pub id: Uuid,
  pub blending_doc_id: Uuid,
  pub storage_id: Uuid,
  pub produced_amount: Decimal,
}

#[response_dto]
pub struct InventoryReconciliationResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub warehouse_id: Uuid,
}

#[response_dto]
pub struct InventoryAdjustmentResponse {
  pub id: Uuid,
  pub reconciliation_id: Uuid,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub contractor_id: Uuid,
  pub adjustment_type: AdjustmentType,
  pub amount: Decimal,
  pub reason: Option<String>,
}

#[response_dto]
pub struct BlendingCompositeResponse {
  pub document: BlendingResponse,
  pub components: Vec<BlendingComponentResponse>,
  pub results: Vec<BlendingResultResponse>,
  pub executed: bool,
}
