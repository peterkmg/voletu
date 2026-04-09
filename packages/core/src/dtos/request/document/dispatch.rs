use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use validator::Validate;
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
pub struct UpdateDispatchRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date: Option<DateTime<Utc>>,
  pub dispatch_purpose: Option<DispatchPurpose>,
  pub dispatch_method: Option<DispatchMethod>,
  pub contractor_id: Option<Uuid>,
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
  #[validate(nested)]
  #[serde(flatten)]
  pub item: DispatchItemCompositeRequest,
}

impl CreateDispatchItemRequest {
  pub fn from_composite(dispatch_doc_id: Uuid, item: &DispatchItemCompositeRequest) -> Self {
    Self {
      dispatch_doc_id,
      item: item.clone(),
    }
  }
}

#[request_dto]
pub struct UpdateDispatchItemRequest {
  pub product_id: Option<Uuid>,
  pub storage_id: Option<Uuid>,
  pub dispatched_amount: Option<Decimal>,
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
  #[validate(nested)]
  #[serde(flatten)]
  pub measurement: DispatchMeasurementCompositeRequest,
}

impl CreateDispatchMeasurementRequest {
  pub fn from_composite(
    dispatch_doc_id: Uuid,
    measurement: &DispatchMeasurementCompositeRequest,
  ) -> Self {
    Self {
      dispatch_doc_id,
      measurement: measurement.clone(),
    }
  }
}

#[request_dto]
pub struct UpdateDispatchMeasurementRequest {
  pub storage_id: Option<Uuid>,
  pub before_height: Option<Decimal>,
  pub before_volume: Option<Decimal>,
  pub before_density: Option<Decimal>,
  pub before_mass: Option<Decimal>,
  pub after_height: Option<Decimal>,
  pub after_volume: Option<Decimal>,
  pub after_density: Option<Decimal>,
  pub after_mass: Option<Decimal>,
}

#[request_dto]
pub struct DispatchItemCompositeRequest {
  pub product_id: Uuid,
  pub storage_id: Uuid,
  pub dispatched_amount: Decimal,
}

#[request_dto]
pub struct CreateDispatchCompositeRequest {
  #[validate(nested)]
  #[serde(flatten)]
  pub dispatch: CreateDispatchRequest,
  #[validate(length(min = 1), nested)]
  pub items: Vec<DispatchItemCompositeRequest>,
  #[validate(length(min = 1), nested)]
  pub storage_measurements: Option<Vec<DispatchMeasurementCompositeRequest>>,
}
