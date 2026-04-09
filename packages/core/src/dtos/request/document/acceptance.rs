use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use validator::Validate;
use voletu_core_macros::request_dto;

use crate::enums::ArrivalType;

#[request_dto]
pub struct CreateAcceptanceRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date_accepted: DateTime<Utc>,
  pub arrival_type: ArrivalType,
  #[validate(length(min = 1))]
  pub source_entity: Option<String>,
  pub contractor_id: Uuid,
  pub truck_waybill_id: Option<Uuid>,
  pub rail_waybill_id: Option<Uuid>,
  pub transit_dispatch_id: Option<Uuid>,
}

#[request_dto]
pub struct UpdateAcceptanceRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date_accepted: Option<DateTime<Utc>>,
  pub arrival_type: Option<ArrivalType>,
  #[validate(length(min = 1))]
  pub source_entity: Option<String>,
  pub contractor_id: Option<Uuid>,
  pub truck_waybill_id: Option<Uuid>,
  pub rail_waybill_id: Option<Uuid>,
  pub transit_dispatch_id: Option<Uuid>,
}

#[request_dto]
pub struct CreateAcceptanceItemRequest {
  pub acceptance_doc_id: Uuid,
  #[validate(nested)]
  #[serde(flatten)]
  pub item: AcceptanceItemCompositeRequest,
}

impl CreateAcceptanceItemRequest {
  pub fn from_composite(acceptance_doc_id: Uuid, item: &AcceptanceItemCompositeRequest) -> Self {
    Self {
      acceptance_doc_id,
      item: item.clone(),
    }
  }
}

#[request_dto]
pub struct UpdateAcceptanceItemRequest {
  pub product_id: Option<Uuid>,
  pub storage_id: Option<Uuid>,
  pub accepted_amount: Option<Decimal>,
}

#[request_dto]
pub struct AcceptanceItemCompositeRequest {
  pub product_id: Uuid,
  pub storage_id: Uuid,
  pub accepted_amount: Decimal,
}

#[request_dto]
pub struct CreateAcceptanceCompositeRequest {
  #[validate(nested)]
  #[serde(flatten)]
  pub acceptance: CreateAcceptanceRequest,
  #[validate(length(min = 1), nested)]
  pub items: Vec<AcceptanceItemCompositeRequest>,
}

#[request_dto]
pub struct TransportAcceptanceCompositeRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date_accepted: DateTime<Utc>,
  #[validate(length(min = 1))]
  pub source_entity: Option<String>,
  #[validate(length(min = 1), nested)]
  pub items: Vec<AcceptanceItemCompositeRequest>,
}
