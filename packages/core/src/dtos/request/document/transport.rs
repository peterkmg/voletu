use chrono::NaiveDate;
use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use validator::Validate;
use voletu_core_macros::request_dto;

#[request_dto]
pub struct CreateTruckWaybillRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: NaiveDate,
  pub sender_id: Uuid,
  pub base_id: Uuid,
}

#[request_dto]
pub struct UpdateTruckWaybillRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date: Option<NaiveDate>,
  pub sender_id: Option<Uuid>,
  pub base_id: Option<Uuid>,
}

#[request_dto]
pub struct CreateTruckWaybillItemRequest {
  pub truck_waybill_id: Uuid,
  #[validate(nested)]
  #[serde(flatten)]
  pub item: TruckWaybillItemCompositeRequest,
}

impl CreateTruckWaybillItemRequest {
  pub fn from_composite(truck_waybill_id: Uuid, item: &TruckWaybillItemCompositeRequest) -> Self {
    Self {
      truck_waybill_id,
      item: item.clone(),
    }
  }
}

#[request_dto]
pub struct UpdateTruckWaybillItemRequest {
  pub product_id: Option<Uuid>,
  pub declared_amount: Option<Decimal>,
}

#[request_dto]
pub struct CreateTruckWeightDocRequest {
  pub truck_waybill_id: Uuid,
  #[validate(nested)]
  #[serde(flatten)]
  pub weight_doc: TruckWeightDocCompositeRequest,
}

impl CreateTruckWeightDocRequest {
  pub fn from_composite(
    truck_waybill_id: Uuid,
    weight_doc: &TruckWeightDocCompositeRequest,
  ) -> Self {
    Self {
      truck_waybill_id,
      weight_doc: weight_doc.clone(),
    }
  }
}

#[request_dto]
pub struct UpdateTruckWeightDocRequest {
  pub total_weight: Option<Decimal>,
}

#[request_dto]
pub struct CreateRailWaybillRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: NaiveDate,
  pub sender_id: Uuid,
  pub base_id: Uuid,
}

#[request_dto]
pub struct UpdateRailWaybillRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date: Option<NaiveDate>,
  pub sender_id: Option<Uuid>,
  pub base_id: Option<Uuid>,
}

#[request_dto]
pub struct CreateRailWagonManifestRequest {
  pub rail_waybill_id: Uuid,
  #[validate(nested)]
  #[serde(flatten)]
  pub manifest: RailWagonManifestCompositeRequest,
}

impl CreateRailWagonManifestRequest {
  pub fn from_composite(
    rail_waybill_id: Uuid,
    manifest: &RailWagonManifestCompositeRequest,
  ) -> Self {
    Self {
      rail_waybill_id,
      manifest: manifest.clone(),
    }
  }
}

#[request_dto]
pub struct UpdateRailWagonManifestRequest {
  #[validate(length(min = 1))]
  pub wagon_number: Option<String>,
  pub product_id: Option<Uuid>,
  pub declared_volume: Option<Decimal>,
  pub declared_density: Option<Decimal>,
  pub declared_mass: Option<Decimal>,
}

#[request_dto]
pub struct CreateRailWagonMeasurementRequest {
  pub wagon_manifest_id: Uuid,
  pub measured_height: Decimal,
  pub lab_density: Option<Decimal>,
  pub calculated_mass: Decimal,
}

impl CreateRailWagonMeasurementRequest {
  pub fn from_composite(
    wagon_manifest_id: Uuid,
    measurement: &RailWagonMeasurementCompositeRequest,
  ) -> Self {
    Self {
      wagon_manifest_id,
      measured_height: measurement.measured_height,
      lab_density: measurement.lab_density,
      calculated_mass: measurement.calculated_mass,
    }
  }
}

#[request_dto]
pub struct UpdateRailWagonMeasurementRequest {
  pub measured_height: Option<Decimal>,
  pub lab_density: Option<Decimal>,
  pub calculated_mass: Option<Decimal>,
}

#[request_dto]
pub struct CreateRailWagonWeightRequest {
  pub wagon_manifest_id: Uuid,
  pub gross_weight: Decimal,
  pub tare_weight: Decimal,
  pub net_product_weight: Decimal,
}

impl CreateRailWagonWeightRequest {
  pub fn from_composite(wagon_manifest_id: Uuid, weight: &RailWagonWeightCompositeRequest) -> Self {
    Self {
      wagon_manifest_id,
      gross_weight: weight.gross_weight,
      tare_weight: weight.tare_weight,
      net_product_weight: weight.net_product_weight,
    }
  }
}

#[request_dto]
pub struct UpdateRailWagonWeightRequest {
  pub gross_weight: Option<Decimal>,
  pub tare_weight: Option<Decimal>,
  pub net_product_weight: Option<Decimal>,
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
pub struct TruckWaybillCompositeRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: NaiveDate,
  pub sender_id: Uuid,
  pub base_id: Uuid,
  #[validate(length(min = 1), nested)]
  pub items: Option<Vec<TruckWaybillItemCompositeRequest>>,
  #[validate(nested)]
  pub weight_docs: Option<Vec<TruckWeightDocCompositeRequest>>,
}

impl CreateTruckWaybillRequest {
  pub fn from_composite(req: &TruckWaybillCompositeRequest) -> Self {
    Self {
      document_number: req.document_number.clone(),
      date: req.date,
      sender_id: req.sender_id,
      base_id: req.base_id,
    }
  }
}

#[request_dto]
pub struct RailWagonManifestCompositeRequest {
  #[validate(length(min = 1))]
  pub wagon_number: String,
  pub product_id: Uuid,
  pub declared_volume: Decimal,
  pub declared_density: Decimal,
  pub declared_mass: Decimal,
  #[validate(length(min = 1), nested)]
  pub measurements: Option<Vec<RailWagonMeasurementCompositeRequest>>,
  #[validate(length(min = 1), nested)]
  pub weights: Option<Vec<RailWagonWeightCompositeRequest>>,
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
pub struct RailWaybillCompositeRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: NaiveDate,
  pub sender_id: Uuid,
  pub base_id: Uuid,
  #[validate(length(min = 1), nested)]
  pub manifests: Option<Vec<RailWagonManifestCompositeRequest>>,
}

impl CreateRailWaybillRequest {
  pub fn from_composite(req: &RailWaybillCompositeRequest) -> Self {
    Self {
      document_number: req.document_number.clone(),
      date: req.date,
      sender_id: req.sender_id,
      base_id: req.base_id,
    }
  }
}
