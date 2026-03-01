use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::request_dto;

use crate::enums::AdjustmentType;

#[request_dto]
#[validate(schema(function = "crate::dtos::validators::validate_physical_transfer_request"))]
pub struct CreatePhysicalTransferRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub start_cargo_ops: DateTime<Utc>,
  pub end_cargo_ops: DateTime<Utc>,
  pub contractor_id: Uuid,
  pub product_id: Uuid,
  pub from_storage_id: Uuid,
  pub to_storage_id: Uuid,
  pub amount_transferred: Decimal,
}

#[request_dto]
pub struct CreateOwnershipTransferRequest {
  pub date: DateTime<Utc>,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub from_contractor_id: Uuid,
  pub to_contractor_id: Uuid,
  pub amount_transferred: Decimal,
}

#[request_dto]
pub struct CreateBlendingRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub contractor_id: Uuid,
  pub target_product_id: Uuid,
}

#[request_dto]
pub struct CreateBlendingComponentRequest {
  pub blending_doc_id: Uuid,
  #[serde(flatten)]
  pub component: BlendingComponentCompositeRequest,
}

#[request_dto]
pub struct CreateBlendingResultRequest {
  pub blending_doc_id: Uuid,
  #[serde(flatten)]
  pub result: BlendingResultCompositeRequest,
}

#[request_dto]
pub struct BlendingComponentCompositeRequest {
  pub storage_id: Uuid,
  pub source_product_id: Uuid,
  pub amount_used: Decimal,
}

#[request_dto]
pub struct BlendingResultCompositeRequest {
  pub storage_id: Uuid,
  pub produced_amount: Decimal,
}

#[request_dto]
pub struct CreateInventoryReconciliationRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub warehouse_id: Uuid,
}

#[request_dto]
pub struct CreateInventoryAdjustmentRequest {
  pub reconciliation_id: Uuid,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub contractor_id: Uuid,
  pub adjustment_type: AdjustmentType,
  pub amount: Decimal,
  #[validate(length(min = 1))]
  pub reason: Option<String>,
}

#[voletu_core_macros::request_dto]
pub struct CreateBlendingCompositeRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub contractor_id: Uuid,
  pub target_product_id: Uuid,
  #[validate(length(min = 1))]
  pub components: Vec<BlendingComponentCompositeRequest>,
  #[validate(length(min = 1))]
  pub results: Vec<BlendingResultCompositeRequest>,
}
