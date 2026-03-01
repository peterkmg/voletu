use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::request_dto;

use crate::enums::{BunkerType, DispatchMethod, DispatchPurpose};

#[request_dto]
#[validate(schema(function = "crate::dtos::validators::validate_dispatch_request"))]
pub struct CreateDispatchRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub dispatch_purpose: DispatchPurpose,
  pub dispatch_method: DispatchMethod,
  pub contractor_id: Uuid,
  pub destination_base_id: Option<Uuid>,
  #[validate(length(min = 1))]
  pub receiver_entity: Option<String>,
  pub start_cargo_ops: Option<DateTime<Utc>>,
  pub end_cargo_ops: Option<DateTime<Utc>>,
  pub bunker_type: Option<BunkerType>,
  pub exporter_id: Option<Uuid>,
  pub port_id: Option<Uuid>,
}

#[request_dto]
pub struct CreateDispatchItemRequest {
  pub dispatch_doc_id: Uuid,
  #[serde(flatten)]
  pub item: DispatchItemCompositeRequest,
}

#[request_dto]
pub struct DispatchMeasurementCompositeRequest {
  pub storage_id: Uuid,
  pub before_height: Option<Decimal>,
  pub before_volume: Option<Decimal>,
  pub before_density: Option<Decimal>,
  pub before_mass: Decimal,
  pub after_height: Option<Decimal>,
  pub after_volume: Option<Decimal>,
  pub after_density: Option<Decimal>,
  pub after_mass: Decimal,
}

#[request_dto]
pub struct CreateDispatchMeasurementRequest {
  pub dispatch_doc_id: Uuid,
  #[serde(flatten)]
  pub measurement: DispatchMeasurementCompositeRequest,
}

#[request_dto]
pub struct DispatchItemCompositeRequest {
  pub product_id: Uuid,
  pub storage_id: Uuid,
  pub dispatched_amount: Decimal,
}

#[request_dto]
pub struct CreateDispatchCompositeRequest {
  #[serde(flatten)]
  pub dispatch: CreateDispatchRequest,
  #[validate(length(min = 1))]
  pub items: Vec<DispatchItemCompositeRequest>,
  #[validate(length(min = 1))]
  pub storage_measurements: Option<Vec<DispatchMeasurementCompositeRequest>>,
}
