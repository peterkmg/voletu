use crate::{
  dtos::{
    AcceptanceAllocationResponse,
    AcceptanceItemResponse,
    AcceptanceResponse,
    DispatchItemResponse,
    DispatchMeasurementResponse,
    DispatchResponse,
  },
  entities::{
    acceptance_document,
    acceptance_item,
    acceptance_storage_allocation,
    dispatch_document,
    dispatch_item,
    dispatch_storage_measurement,
  },
};

pub fn map_acceptance_document(model: acceptance_document::Model) -> AcceptanceResponse {
  AcceptanceResponse {
    id: model.id,
    document_number: model.document_number,
    date_accepted: model.date_accepted.to_rfc3339(),
    arrival_type: model.arrival_type,
    source_entity: model.source_entity,
    truck_waybill_id: model.truck_waybill_id,
    rail_waybill_id: model.rail_waybill_id,
    transit_dispatch_id: model.transit_dispatch_id,
  }
}

pub fn map_acceptance_item(model: acceptance_item::Model) -> AcceptanceItemResponse {
  AcceptanceItemResponse {
    id: model.id,
    acceptance_doc_id: model.acceptance_doc_id,
    product_id: model.product_id,
    contractor_id: model.contractor_id,
    accepted_amount: model.accepted_amount,
  }
}

pub fn map_acceptance_allocation(
  model: acceptance_storage_allocation::Model,
) -> AcceptanceAllocationResponse {
  AcceptanceAllocationResponse {
    id: model.id,
    acceptance_item_id: model.acceptance_item_id,
    storage_id: model.storage_id,
    allocated_amount: model.allocated_amount,
  }
}

pub fn map_dispatch_document(model: dispatch_document::Model) -> DispatchResponse {
  DispatchResponse {
    id: model.id,
    document_number: model.document_number,
    date: model.date.to_rfc3339(),
    dispatch_purpose: model.dispatch_purpose,
    dispatch_method: model.dispatch_method,
    contractor_id: model.contractor_id,
    destination_base_id: model.destination_base_id,
    receiver_entity: model.receiver_entity,
    start_cargo_ops: model.start_cargo_ops.map(|v| v.to_rfc3339()),
    end_cargo_ops: model.end_cargo_ops.map(|v| v.to_rfc3339()),
    bunker_type: model.bunker_type,
    exporter_id: model.exporter_id,
    port_id: model.port_id,
  }
}

pub fn map_dispatch_item(item: dispatch_item::Model) -> DispatchItemResponse {
  DispatchItemResponse {
    id: item.id,
    dispatch_doc_id: item.dispatch_doc_id,
    product_id: item.product_id,
    storage_id: item.storage_id,
    dispatched_amount: item.dispatched_amount,
  }
}

pub fn map_dispatch_measurement(
  measurement: dispatch_storage_measurement::Model,
) -> DispatchMeasurementResponse {
  DispatchMeasurementResponse {
    id: measurement.id,
    dispatch_doc_id: measurement.dispatch_doc_id,
    storage_id: measurement.storage_id,
    before_height: measurement.before_height,
    before_volume: measurement.before_volume,
    before_density: measurement.before_density,
    before_mass: measurement.before_mass,
    after_height: measurement.after_height,
    after_volume: measurement.after_volume,
    after_density: measurement.after_density,
    after_mass: measurement.after_mass,
  }
}
