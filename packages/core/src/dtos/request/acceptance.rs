use chrono::{DateTime, Utc};
use uuid::Uuid;
use voletu_core_macros::request_dto;

use crate::dtos::enums::ArrivalType;

#[request_dto]
pub struct CreateAcceptanceRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date_accepted: DateTime<Utc>,
  pub arrival_type: ArrivalType,
  pub source_entity: Option<String>,
  pub truck_waybill_id: Option<Uuid>,
  pub rail_waybill_id: Option<Uuid>,
  pub transit_dispatch_id: Option<Uuid>,
}

#[request_dto]
pub struct CreateAcceptanceItemRequest {
  pub acceptance_doc_id: Uuid,
  pub product_id: Uuid,
  pub contractor_id: Uuid,
  pub accepted_amount: f64,
}

#[request_dto]
pub struct CreateAcceptanceAllocationRequest {
  pub acceptance_item_id: Uuid,
  #[serde(flatten)]
  pub allocation: AcceptanceAllocationCompositeRequest,
}

#[request_dto]
pub struct AcceptanceAllocationCompositeRequest {
  pub storage_id: Uuid,
  pub allocated_amount: f64,
}

#[request_dto]
pub struct AcceptanceItemCompositeRequest {
  pub product_id: Uuid,
  pub contractor_id: Uuid,
  pub accepted_amount: f64,
  #[validate(length(min = 1))]
  pub allocations: Vec<AcceptanceAllocationCompositeRequest>,
}

#[request_dto]
pub struct CreateAcceptanceCompositeRequest {
  #[serde(flatten)]
  pub acceptance: CreateAcceptanceRequest,
  #[validate(length(min = 1))]
  pub items: Vec<AcceptanceItemCompositeRequest>,
}
