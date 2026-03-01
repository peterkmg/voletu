use uuid::Uuid;

use crate::{
  dtos::{
    AcceptanceAllocationCompositeRequest,
    AcceptanceItemCompositeRequest,
    CreateAcceptanceCompositeRequest,
    CreateAcceptanceRequest,
    IntakeAcceptanceCompositeRequest,
    RailWagonManifestResponse,
    RailWagonMeasurementResponse,
    RailWagonWeightResponse,
    RailWaybillResponse,
    TruckWaybillItemResponse,
    TruckWaybillResponse,
    TruckWeightDocResponse,
  },
  entities::{
    rail_wagon_manifest,
    rail_wagon_measurement,
    rail_wagon_weight,
    rail_waybill,
    truck_waybill,
    truck_waybill_item,
    truck_weight_doc,
  },
  enums::ArrivalType,
};

pub fn map_truck_waybill(row: truck_waybill::Model) -> TruckWaybillResponse {
  TruckWaybillResponse {
    id: row.id,
    document_number: row.document_number,
    date: row.date.to_string(),
    sender_id: row.sender_id,
  }
}

pub fn map_truck_waybill_item(row: truck_waybill_item::Model) -> TruckWaybillItemResponse {
  TruckWaybillItemResponse {
    id: row.id,
    truck_waybill_id: row.truck_waybill_id,
    product_id: row.product_id,
    declared_amount: row.declared_amount,
  }
}

pub fn map_truck_weight_doc(row: truck_weight_doc::Model) -> TruckWeightDocResponse {
  TruckWeightDocResponse {
    id: row.id,
    truck_waybill_id: row.truck_waybill_id,
    total_weight: row.total_weight,
  }
}

pub fn map_rail_waybill(row: rail_waybill::Model) -> RailWaybillResponse {
  RailWaybillResponse {
    id: row.id,
    document_number: row.document_number,
    date: row.date.to_string(),
    sender_id: row.sender_id,
  }
}

pub fn map_rail_manifest(row: rail_wagon_manifest::Model) -> RailWagonManifestResponse {
  RailWagonManifestResponse {
    id: row.id,
    rail_waybill_id: row.rail_waybill_id,
    wagon_number: row.wagon_number,
    product_id: row.product_id,
    declared_volume: row.declared_volume,
    declared_density: row.declared_density,
    declared_mass: row.declared_mass,
  }
}

pub fn map_rail_measurement(row: rail_wagon_measurement::Model) -> RailWagonMeasurementResponse {
  RailWagonMeasurementResponse {
    id: row.id,
    wagon_manifest_id: row.wagon_manifest_id,
    measured_height: row.measured_height,
    lab_density: row.lab_density,
    calculated_mass: row.calculated_mass,
  }
}

pub fn map_rail_weight(row: rail_wagon_weight::Model) -> RailWagonWeightResponse {
  RailWagonWeightResponse {
    id: row.id,
    wagon_manifest_id: row.wagon_manifest_id,
    gross_weight: row.gross_weight,
    tare_weight: row.tare_weight,
    net_product_weight: row.net_product_weight,
  }
}

fn map_acceptance_request(
  acceptance: &IntakeAcceptanceCompositeRequest,
  arrival_type: ArrivalType,
  truck_waybill_id: Option<Uuid>,
  rail_waybill_id: Option<Uuid>,
) -> CreateAcceptanceCompositeRequest {
  CreateAcceptanceCompositeRequest {
    acceptance: CreateAcceptanceRequest {
      document_number: acceptance.document_number.clone(),
      date_accepted: acceptance.date_accepted,
      arrival_type,
      source_entity: acceptance.source_entity.clone(),
      truck_waybill_id,
      rail_waybill_id,
      transit_dispatch_id: None,
    },
    items: acceptance
      .items
      .iter()
      .map(|item| AcceptanceItemCompositeRequest {
        product_id: item.product_id,
        contractor_id: item.contractor_id,
        accepted_amount: item.accepted_amount,
        allocations: item
          .allocations
          .iter()
          .map(|allocation| AcceptanceAllocationCompositeRequest {
            storage_id: allocation.storage_id,
            allocated_amount: allocation.allocated_amount,
          })
          .collect(),
      })
      .collect(),
  }
}

pub fn map_truck_acceptance_request(
  acceptance: &IntakeAcceptanceCompositeRequest,
  truck_waybill_id: Uuid,
) -> CreateAcceptanceCompositeRequest {
  map_acceptance_request(acceptance, ArrivalType::Truck, Some(truck_waybill_id), None)
}

pub fn map_rail_acceptance_request(
  acceptance: &IntakeAcceptanceCompositeRequest,
  rail_waybill_id: Uuid,
) -> CreateAcceptanceCompositeRequest {
  map_acceptance_request(acceptance, ArrivalType::Rail, None, Some(rail_waybill_id))
}
