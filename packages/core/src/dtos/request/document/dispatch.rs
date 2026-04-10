use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::Decimal, ActiveValue::Set};
use uuid::Uuid;
use validator::Validate;
use voletu_core_macros::request_dto;

use crate::{
  entities::{dispatch_document, dispatch_item, dispatch_storage_measurement},
  enums::{BunkerType, DispatchMethod, DispatchPurpose, DocumentStatus},
};

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

impl From<&DispatchItemCompositeRequest> for dispatch_item::ActiveModelEx {
  fn from(item: &DispatchItemCompositeRequest) -> Self {
    Self {
      product_id: Set(item.product_id),
      storage_id: Set(item.storage_id),
      dispatched_amount: Set(item.dispatched_amount),
      ..Default::default()
    }
  }
}

impl From<&DispatchMeasurementCompositeRequest> for dispatch_storage_measurement::ActiveModelEx {
  fn from(measurement: &DispatchMeasurementCompositeRequest) -> Self {
    Self {
      storage_id: Set(measurement.storage_id),
      before_height: Set(measurement.before_height),
      before_volume: Set(measurement.before_volume),
      before_density: Set(measurement.before_density),
      before_mass: Set(measurement.before_mass),
      after_height: Set(measurement.after_height),
      after_volume: Set(measurement.after_volume),
      after_density: Set(measurement.after_density),
      after_mass: Set(measurement.after_mass),
      ..Default::default()
    }
  }
}

impl From<&CreateDispatchCompositeRequest> for dispatch_document::ActiveModelEx {
  fn from(req: &CreateDispatchCompositeRequest) -> Self {
    Self {
      document_number: Set(req.dispatch.document_number.clone()),
      date: Set(req.dispatch.date),
      status: Set(DocumentStatus::Draft),
      version: Set(1),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      dispatch_purpose: Set(req.dispatch.dispatch_purpose),
      dispatch_method: Set(req.dispatch.dispatch_method),
      contractor_id: Set(req.dispatch.contractor_id),
      destination_base_id: Set(req.dispatch.destination_base_id),
      receiver_entity: Set(req.dispatch.receiver_entity.clone()),
      start_cargo_ops: Set(req.dispatch.start_cargo_ops),
      end_cargo_ops: Set(req.dispatch.end_cargo_ops),
      bunker_type: Set(req.dispatch.bunker_type),
      exporter_id: Set(req.dispatch.exporter_id),
      port_id: Set(req.dispatch.port_id),
      items: req
        .items
        .iter()
        .map(dispatch_item::ActiveModelEx::from)
        .collect::<Vec<_>>()
        .into(),
      storage_measurements: req
        .storage_measurements
        .as_deref()
        .unwrap_or_default()
        .iter()
        .map(dispatch_storage_measurement::ActiveModelEx::from)
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
  }
}
