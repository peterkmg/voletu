use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::dtos::enums::ArrivalType;

#[response_dto]
pub struct AcceptanceResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date_accepted: String,
  pub arrival_type: ArrivalType,
  pub source_entity: Option<String>,
  pub truck_waybill_id: Option<Uuid>,
  pub rail_waybill_id: Option<Uuid>,
  pub transit_dispatch_id: Option<Uuid>,
}

#[response_dto]
pub struct AcceptanceItemResponse {
  pub id: Uuid,
  pub acceptance_doc_id: Uuid,
  pub product_id: Uuid,
  pub contractor_id: Uuid,
  pub accepted_amount: Decimal,
}

#[response_dto]
pub struct AcceptanceAllocationResponse {
  pub id: Uuid,
  pub acceptance_item_id: Uuid,
  pub storage_id: Uuid,
  pub allocated_amount: Decimal,
}

#[response_dto]
pub struct AcceptanceItemCompositeResponse {
  pub item: AcceptanceItemResponse,
  pub allocations: Vec<AcceptanceAllocationResponse>,
}

#[response_dto]
pub struct AcceptanceCompositeResponse {
  pub document: AcceptanceResponse,
  pub items: Vec<AcceptanceItemCompositeResponse>,
  pub executed: bool,
}
