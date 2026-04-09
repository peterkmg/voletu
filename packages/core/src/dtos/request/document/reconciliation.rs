use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::request_dto;

use crate::enums::AdjustmentType;

#[request_dto]
pub struct CreateInventoryReconciliationRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub contractor_id: Uuid,
  pub warehouse_id: Uuid,
}

#[request_dto]
pub struct UpdateInventoryReconciliationRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date: Option<DateTime<Utc>>,
  pub contractor_id: Option<Uuid>,
  pub warehouse_id: Option<Uuid>,
}

#[request_dto]
pub struct CreateInventoryAdjustmentRequest {
  pub reconciliation_id: Uuid,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub adjustment_type: AdjustmentType,
  pub amount: Decimal,
  #[validate(length(min = 1))]
  pub reason: Option<String>,
}

#[request_dto]
pub struct UpdateInventoryAdjustmentRequest {
  pub storage_id: Option<Uuid>,
  pub product_id: Option<Uuid>,
  pub adjustment_type: Option<AdjustmentType>,
  pub amount: Option<Decimal>,
  #[validate(length(min = 1))]
  pub reason: Option<String>,
}
