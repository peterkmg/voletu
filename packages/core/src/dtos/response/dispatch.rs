use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::dtos::enums::{BunkerType, DispatchMethod, DispatchPurpose};

#[response_dto]
pub struct DispatchResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub dispatch_purpose: DispatchPurpose,
  pub dispatch_method: DispatchMethod,
  pub contractor_id: Uuid,
  pub destination_base_id: Option<Uuid>,
  pub receiver_entity: Option<String>,
  pub start_cargo_ops: Option<String>,
  pub end_cargo_ops: Option<String>,
  pub bunker_type: Option<BunkerType>,
  pub exporter_id: Option<Uuid>,
  pub port_id: Option<Uuid>,
}

#[response_dto]
pub struct DispatchItemResponse {
  pub id: Uuid,
  pub dispatch_doc_id: Uuid,
  pub product_id: Uuid,
  pub storage_id: Uuid,
  pub dispatched_amount: f64,
}

#[response_dto]
pub struct DispatchMeasurementResponse {
  pub id: Uuid,
  pub dispatch_doc_id: Uuid,
  pub storage_id: Uuid,
  pub before_height: Option<f64>,
  pub before_volume: Option<f64>,
  pub before_density: Option<f64>,
  pub before_mass: f64,
  pub after_height: Option<f64>,
  pub after_volume: Option<f64>,
  pub after_density: Option<f64>,
  pub after_mass: f64,
}

#[response_dto]
pub struct DispatchCompositeResponse {
  pub document: DispatchResponse,
  pub items: Vec<DispatchItemResponse>,
  pub storage_measurements: Vec<DispatchMeasurementResponse>,
  pub executed: bool,
}
