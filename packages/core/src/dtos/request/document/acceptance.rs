use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::Decimal, ActiveValue::Set};
use uuid::Uuid;
use validator::Validate;
use voletu_core_macros::request_dto;

use crate::{
  entities::{acceptance_document, acceptance_item},
  enums::{ArrivalType, DocumentStatus},
};

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

impl From<&AcceptanceItemCompositeRequest> for acceptance_item::ActiveModelEx {
  fn from(item: &AcceptanceItemCompositeRequest) -> Self {
    Self {
      product_id: Set(item.product_id),
      storage_id: Set(item.storage_id),
      accepted_amount: Set(item.accepted_amount),
      ..Default::default()
    }
  }
}

impl From<&CreateAcceptanceCompositeRequest> for acceptance_document::ActiveModelEx {
  fn from(req: &CreateAcceptanceCompositeRequest) -> Self {
    Self {
      document_number: Set(req.acceptance.document_number.clone()),
      date_accepted: Set(req.acceptance.date_accepted),
      status: Set(DocumentStatus::Draft),
      version: Set(1),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      arrival_type: Set(req.acceptance.arrival_type),
      source_entity: Set(req.acceptance.source_entity.clone()),
      contractor_id: Set(req.acceptance.contractor_id),
      truck_waybill_id: Set(req.acceptance.truck_waybill_id),
      rail_waybill_id: Set(req.acceptance.rail_waybill_id),
      transit_dispatch_id: Set(req.acceptance.transit_dispatch_id),
      items: req
        .items
        .iter()
        .map(acceptance_item::ActiveModelEx::from)
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
  }
}

#[request_dto]
pub struct UpdateAcceptanceItemCompositeRequest {
  pub id: Option<Uuid>,
  pub product_id: Uuid,
  pub storage_id: Uuid,
  pub accepted_amount: Decimal,
}

#[request_dto]
pub struct UpdateAcceptanceCompositeRequest {
  #[validate(nested)]
  #[serde(flatten)]
  pub acceptance: UpdateAcceptanceRequest,
  #[validate(length(min = 1), nested)]
  pub items: Vec<UpdateAcceptanceItemCompositeRequest>,
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
