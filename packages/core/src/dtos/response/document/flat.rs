use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::enums::{AdjustmentType, DispatchMethod, DispatchPurpose, DocumentStatus};

#[response_dto]
pub struct AcceptanceFlatRow {
  pub id: Uuid,
  pub document_id: Uuid,
  pub document_number: String,
  pub date_accepted: String,
  pub status: DocumentStatus,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub source_entity: Option<String>,
  pub item_id: Uuid,
  pub product_id_name: String,
  pub storage_id_name: String,
  pub contractor_id_name: String,
  pub accepted_amount: Decimal,
}

#[response_dto]
pub struct DispatchFlatRow {
  pub id: Uuid,
  pub document_id: Uuid,
  pub document_number: String,
  pub date: String,
  pub status: DocumentStatus,
  pub dispatch_method: DispatchMethod,
  pub dispatch_purpose: DispatchPurpose,
  pub contractor_id_name: String,
  pub item_id: Uuid,
  pub product_id_name: String,
  pub storage_id_name: String,
  pub dispatched_amount: Decimal,
}

#[response_dto]
pub struct PhysicalTransferFlatRow {
  pub id: Uuid,
  pub document_id: Uuid,
  pub document_number: String,
  pub date: String,
  pub status: DocumentStatus,
  pub contractor_id_name: String,
  pub item_id: Uuid,
  pub product_id_name: String,
  pub from_storage_id_name: String,
  pub to_storage_id_name: String,
  pub amount: Decimal,
}

#[response_dto]
pub struct OwnershipTransferFlatRow {
  pub id: Uuid,
  pub document_id: Uuid,
  pub date: String,
  pub status: DocumentStatus,
  pub item_id: Uuid,
  pub product_id_name: String,
  pub storage_id_name: String,
  pub from_contractor_id_name: String,
  pub to_contractor_id_name: String,
  pub amount: Decimal,
}

#[response_dto]
pub struct BlendingFlatRow {
  pub id: Uuid,
  pub document_id: Uuid,
  pub document_number: String,
  pub date: String,
  pub status: DocumentStatus,
  pub contractor_id_name: String,
  pub target_product_id_name: String,
  pub item_id: Uuid,
  pub item_type: String,
  pub product_id_name: String,
  pub storage_id_name: String,
  pub amount: Decimal,
}

#[response_dto]
pub struct ReconciliationFlatRow {
  pub id: Uuid,
  pub document_id: Uuid,
  pub document_number: String,
  pub date: String,
  pub status: DocumentStatus,
  pub contractor_id_name: String,
  pub warehouse_id_name: String,
  pub item_id: Uuid,
  pub product_id_name: String,
  pub storage_id_name: String,
  pub adjustment_type: AdjustmentType,
  pub amount: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub reason: Option<String>,
}
