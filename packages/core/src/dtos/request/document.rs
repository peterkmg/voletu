use chrono::{DateTime, NaiveDate, Utc};
use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use validator::Validate;
use voletu_core_macros::request_dto;

use crate::enums::{AdjustmentType, ArrivalType, BunkerType, DispatchMethod, DispatchPurpose};

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

#[request_dto]
#[validate(schema(function = "crate::dtos::validators::validate_physical_transfer_request"))]
pub struct CreatePhysicalTransferRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub contractor_id: Uuid,
  pub start_cargo_ops: DateTime<Utc>,
  pub end_cargo_ops: DateTime<Utc>,
  #[validate(length(min = 1), nested)]
  pub items: Vec<PhysicalTransferItemCompositeRequest>,
}

#[request_dto]
pub struct UpdatePhysicalTransferRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date: Option<DateTime<Utc>>,
  pub contractor_id: Option<Uuid>,
  pub start_cargo_ops: Option<DateTime<Utc>>,
  pub end_cargo_ops: Option<DateTime<Utc>>,
}

#[request_dto]
pub struct CreateOwnershipTransferRequest {
  pub date: DateTime<Utc>,
  #[validate(length(min = 1), nested)]
  pub items: Vec<OwnershipTransferItemCompositeRequest>,
}

#[request_dto]
pub struct UpdateOwnershipTransferRequest {
  pub date: Option<DateTime<Utc>>,
}

#[request_dto]
pub struct PhysicalTransferItemCompositeRequest {
  pub product_id: Uuid,
  pub from_storage_id: Uuid,
  pub to_storage_id: Uuid,
  pub amount: Decimal,
}

#[request_dto]
pub struct CreatePhysicalTransferItemRequest {
  pub physical_transfer_id: Uuid,
  #[validate(nested)]
  #[serde(flatten)]
  pub item: PhysicalTransferItemCompositeRequest,
}

#[request_dto]
pub struct UpdatePhysicalTransferItemRequest {
  pub product_id: Option<Uuid>,
  pub from_storage_id: Option<Uuid>,
  pub to_storage_id: Option<Uuid>,
  pub amount: Option<Decimal>,
}

#[request_dto]
pub struct OwnershipTransferItemCompositeRequest {
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub from_contractor_id: Uuid,
  pub to_contractor_id: Uuid,
  pub amount: Decimal,
}

#[request_dto]
pub struct CreateOwnershipTransferItemRequest {
  pub ownership_transfer_id: Uuid,
  #[validate(nested)]
  #[serde(flatten)]
  pub item: OwnershipTransferItemCompositeRequest,
}

#[request_dto]
pub struct UpdateOwnershipTransferItemRequest {
  pub storage_id: Option<Uuid>,
  pub product_id: Option<Uuid>,
  pub from_contractor_id: Option<Uuid>,
  pub to_contractor_id: Option<Uuid>,
  pub amount: Option<Decimal>,
}

#[request_dto]
pub struct CreateBlendingRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub contractor_id: Uuid,
  pub target_product_id: Uuid,
}

#[request_dto]
pub struct UpdateBlendingRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date: Option<DateTime<Utc>>,
  pub contractor_id: Option<Uuid>,
  pub target_product_id: Option<Uuid>,
}

#[request_dto]
pub struct CreateBlendingComponentRequest {
  pub blending_doc_id: Uuid,
  #[validate(nested)]
  #[serde(flatten)]
  pub component: BlendingComponentCompositeRequest,
}

impl CreateBlendingComponentRequest {
  pub fn from_composite(
    blending_doc_id: Uuid,
    component: &BlendingComponentCompositeRequest,
  ) -> Self {
    Self {
      blending_doc_id,
      component: component.clone(),
    }
  }
}

#[request_dto]
pub struct UpdateBlendingComponentRequest {
  pub storage_id: Option<Uuid>,
  pub source_product_id: Option<Uuid>,
  pub amount_used: Option<Decimal>,
}

#[request_dto]
pub struct CreateBlendingResultRequest {
  pub blending_doc_id: Uuid,
  #[validate(nested)]
  #[serde(flatten)]
  pub result: BlendingResultCompositeRequest,
}

impl CreateBlendingResultRequest {
  pub fn from_composite(blending_doc_id: Uuid, result: &BlendingResultCompositeRequest) -> Self {
    Self {
      blending_doc_id,
      result: result.clone(),
    }
  }
}

#[request_dto]
pub struct UpdateBlendingResultRequest {
  pub storage_id: Option<Uuid>,
  pub produced_amount: Option<Decimal>,
}

#[request_dto]
pub struct BlendingComponentCompositeRequest {
  pub storage_id: Uuid,
  pub source_product_id: Uuid,
  pub amount_used: Decimal,
}

#[request_dto]
pub struct BlendingResultCompositeRequest {
  pub storage_id: Uuid,
  pub produced_amount: Decimal,
}

#[request_dto]
pub struct CreateInventoryReconciliationRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub contractor_id: Uuid,
  pub warehouse_id: Uuid,
}

#[request_dto]
pub struct UpdateInventoryReconciliationRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date: Option<DateTime<Utc>>,
  pub contractor_id: Option<Uuid>,
  pub warehouse_id: Option<Uuid>,
}

#[request_dto]
pub struct CreateInventoryAdjustmentRequest {
  pub reconciliation_id: Uuid,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub adjustment_type: AdjustmentType,
  pub amount: Decimal,
  #[validate(length(min = 1))]
  pub reason: Option<String>,
}

#[request_dto]
pub struct UpdateInventoryAdjustmentRequest {
  pub storage_id: Option<Uuid>,
  pub product_id: Option<Uuid>,
  pub adjustment_type: Option<AdjustmentType>,
  pub amount: Option<Decimal>,
  #[validate(length(min = 1))]
  pub reason: Option<String>,
}

#[request_dto]
pub struct CreateBlendingCompositeRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: DateTime<Utc>,
  pub contractor_id: Uuid,
  pub target_product_id: Uuid,
  #[validate(length(min = 1), nested)]
  pub components: Vec<BlendingComponentCompositeRequest>,
  #[validate(length(min = 1), nested)]
  pub results: Vec<BlendingResultCompositeRequest>,
}

impl CreateBlendingRequest {
  pub fn from_composite(req: &CreateBlendingCompositeRequest) -> Self {
    Self {
      document_number: req.document_number.clone(),
      date: req.date,
      contractor_id: req.contractor_id,
      target_product_id: req.target_product_id,
    }
  }
}

#[request_dto]
pub struct CreateTruckWaybillRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: NaiveDate,
  pub sender_id: Uuid,
}

#[request_dto]
pub struct UpdateTruckWaybillRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date: Option<NaiveDate>,
  pub sender_id: Option<Uuid>,
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
}

#[request_dto]
pub struct UpdateRailWaybillRequest {
  #[validate(length(min = 1))]
  pub document_number: Option<String>,
  pub date: Option<NaiveDate>,
  pub sender_id: Option<Uuid>,
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
pub struct TransportAcceptanceCompositeRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date_accepted: DateTime<Utc>,
  #[validate(length(min = 1))]
  pub source_entity: Option<String>,
  #[validate(length(min = 1), nested)]
  pub items: Vec<AcceptanceItemCompositeRequest>,
}

#[request_dto]
pub struct TruckWaybillCompositeRequest {
  #[validate(length(min = 1))]
  pub document_number: String,
  pub date: NaiveDate,
  pub sender_id: Uuid,
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
  #[validate(length(min = 1), nested)]
  pub manifests: Option<Vec<RailWagonManifestCompositeRequest>>,
}

impl CreateRailWaybillRequest {
  pub fn from_composite(req: &RailWaybillCompositeRequest) -> Self {
    Self {
      document_number: req.document_number.clone(),
      date: req.date,
      sender_id: req.sender_id,
    }
  }
}

impl CreatePhysicalTransferItemRequest {
  pub fn from_composite(
    physical_transfer_id: Uuid,
    item: &PhysicalTransferItemCompositeRequest,
  ) -> Self {
    Self {
      physical_transfer_id,
      item: item.clone(),
    }
  }
}

impl CreateOwnershipTransferItemRequest {
  pub fn from_composite(
    ownership_transfer_id: Uuid,
    item: &OwnershipTransferItemCompositeRequest,
  ) -> Self {
    Self {
      ownership_transfer_id,
      item: item.clone(),
    }
  }
}
