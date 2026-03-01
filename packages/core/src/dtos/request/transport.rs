use chrono::{DateTime, NaiveDate, Utc};
use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::request_dto;

use crate::{dtos::AcceptanceItemCompositeRequest, enums::ArrivalType};

#[request_dto]
pub struct CreateTruckWaybillRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: NaiveDate,
  pub sender_id: Uuid,
}

#[request_dto]
pub struct CreateTruckWaybillItemRequest {
  pub truck_waybill_id: Uuid,
  #[serde(flatten)]
  pub item: TruckWaybillItemCompositeRequest,
}

#[request_dto]
pub struct CreateTruckWeightDocRequest {
  pub truck_waybill_id: Uuid,
  #[serde(flatten)]
  pub weight_doc: TruckWeightDocCompositeRequest,
}

#[request_dto]
pub struct CreateRailWaybillRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: NaiveDate,
  pub sender_id: Uuid,
}

#[request_dto]
pub struct CreateRailWagonManifestRequest {
  pub rail_waybill_id: Uuid,
  #[serde(flatten)]
  pub manifest: RailWagonManifestCompositeRequest,
}

#[request_dto]
pub struct CreateRailWagonMeasurementRequest {
  pub wagon_manifest_id: Uuid,
  pub measured_height: Decimal,
  pub lab_density: Option<Decimal>,
  pub calculated_mass: Decimal,
}

#[request_dto]
pub struct CreateRailWagonWeightRequest {
  pub wagon_manifest_id: Uuid,
  pub gross_weight: Decimal,
  pub tare_weight: Decimal,
  pub net_product_weight: Decimal,
}

#[request_dto]
pub struct TruckWaybillItemCompositeRequest {
  pub product_id: Uuid,
  pub declared_amount: Decimal,
}

#[request_dto]
pub struct TruckWeightDocCompositeRequest {
  pub total_weight: Decimal,
}

#[request_dto]
pub struct IntakeAcceptanceCompositeRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date_accepted: DateTime<Utc>,
  #[validate(length(min = 1))]
  pub source_entity: Option<String>,
  #[validate(length(min = 1))]
  pub items: Vec<AcceptanceItemCompositeRequest>,
}

#[request_dto]
pub struct TruckIntakeCompositeRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: NaiveDate,
  pub sender_id: Uuid,
  #[validate(length(min = 1))]
  pub items: Vec<TruckWaybillItemCompositeRequest>,
  pub weight_doc: Option<TruckWeightDocCompositeRequest>,
  pub acceptance: Option<IntakeAcceptanceCompositeRequest>,
}

#[request_dto]
pub struct RailWagonManifestCompositeRequest {
  #[validate(length(min = 1))]
  pub wagon_number: String,
  pub product_id: Uuid,
  pub declared_volume: Decimal,
  pub declared_density: Decimal,
  pub declared_mass: Decimal,
}

#[request_dto]
pub struct RailWagonMeasurementCompositeRequest {
  #[validate(length(min = 1))]
  pub wagon_number: String,
  pub measured_height: Decimal,
  pub lab_density: Option<Decimal>,
  pub calculated_mass: Decimal,
}

#[request_dto]
pub struct RailWagonWeightCompositeRequest {
  #[validate(length(min = 1))]
  pub wagon_number: String,
  pub gross_weight: Decimal,
  pub tare_weight: Decimal,
  pub net_product_weight: Decimal,
}

#[request_dto]
pub struct RailIntakeCompositeRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: NaiveDate,
  pub sender_id: Uuid,
  #[validate(length(min = 1))]
  pub manifests: Vec<RailWagonManifestCompositeRequest>,
  #[validate(length(min = 1))]
  pub measurements: Option<Vec<RailWagonMeasurementCompositeRequest>>,
  #[validate(length(min = 1))]
  pub weights: Option<Vec<RailWagonWeightCompositeRequest>>,
  pub acceptance: Option<IntakeAcceptanceCompositeRequest>,
  pub arrival_type: Option<ArrivalType>,
}
