use uuid::Uuid;
use voletu_core_macros::response_dto;

#[response_dto]
pub struct TruckWaybillResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub sender_id: Uuid,
}

#[response_dto]
pub struct TruckWaybillItemResponse {
  pub id: Uuid,
  pub truck_waybill_id: Uuid,
  pub product_id: Uuid,
  pub declared_amount: f64,
}

#[response_dto]
pub struct TruckWeightDocResponse {
  pub id: Uuid,
  pub truck_waybill_id: Uuid,
  pub total_weight: f64,
}

#[response_dto]
pub struct RailWaybillResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub sender_id: Uuid,
}

#[response_dto]
pub struct RailWagonManifestResponse {
  pub id: Uuid,
  pub rail_waybill_id: Uuid,
  pub wagon_number: String,
  pub product_id: Uuid,
  pub declared_volume: f64,
  pub declared_density: f64,
  pub declared_mass: f64,
}

#[response_dto]
pub struct RailWagonMeasurementResponse {
  pub id: Uuid,
  pub wagon_manifest_id: Uuid,
  pub measured_height: f64,
  pub lab_density: Option<f64>,
  pub calculated_mass: f64,
}

#[response_dto]
pub struct RailWagonWeightResponse {
  pub id: Uuid,
  pub wagon_manifest_id: Uuid,
  pub gross_weight: f64,
  pub tare_weight: f64,
  pub net_product_weight: f64,
}

#[response_dto]
pub struct TruckIntakeCompositeResponse {
  pub waybill: TruckWaybillResponse,
  pub items: Vec<TruckWaybillItemResponse>,
  pub weight_doc: Option<TruckWeightDocResponse>,
  pub acceptance: Option<crate::dtos::AcceptanceCompositeResponse>,
}

#[response_dto]
pub struct RailIntakeCompositeResponse {
  pub waybill: RailWaybillResponse,
  pub manifests: Vec<RailWagonManifestResponse>,
  pub measurements: Vec<RailWagonMeasurementResponse>,
  pub weights: Vec<RailWagonWeightResponse>,
  pub acceptance: Option<crate::dtos::AcceptanceCompositeResponse>,
}
