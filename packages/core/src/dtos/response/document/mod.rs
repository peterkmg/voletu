use std::collections::HashMap;

use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::{
  api::ApiError,
  entities::{
    acceptance_document,
    acceptance_item,
    blending_component,
    blending_document,
    blending_result,
    dispatch_document,
    dispatch_item,
    dispatch_storage_measurement,
    inventory_adjustment,
    inventory_reconciliation,
    ownership_transfer,
    ownership_transfer_item,
    physical_storage_transfer,
    physical_transfer_item,
    rail_wagon_manifest,
    rail_wagon_measurement,
    rail_wagon_weight,
    rail_waybill,
    truck_waybill,
    truck_waybill_item,
    truck_weight_doc,
  },
  enums::{AdjustmentType, ArrivalType, BunkerType, DispatchMethod, DispatchPurpose},
};

mod acceptance;
mod blending;
mod dispatch;
mod flat;
mod reconciliation;
mod transfers;
mod transport;

pub use acceptance::{AcceptanceCompositeResponse, AcceptanceItemResponse, AcceptanceResponse};
pub use blending::{
  BlendingComponentResponse,
  BlendingCompositeResponse,
  BlendingResponse,
  BlendingResultResponse,
};
pub use dispatch::{
  DispatchCompositeResponse,
  DispatchItemResponse,
  DispatchMeasurementResponse,
  DispatchResponse,
};
pub use flat::{
  AcceptanceFlatRow,
  BlendingFlatRow,
  DispatchFlatRow,
  OwnershipTransferFlatRow,
  PhysicalTransferFlatRow,
  ReconciliationFlatRow,
};
pub use reconciliation::{InventoryAdjustmentResponse, InventoryReconciliationResponse};
pub use transfers::{
  OwnershipTransferItemResponse,
  OwnershipTransferResponse,
  PhysicalTransferItemResponse,
  PhysicalTransferResponse,
};
pub use transport::{
  RailWagonManifestResponse,
  RailWagonMeasurementResponse,
  RailWagonWeightResponse,
  RailWaybillCompositeResponse,
  RailWaybillResponse,
  TruckWaybillCompositeResponse,
  TruckWaybillItemResponse,
  TruckWaybillResponse,
  TruckWeightDocResponse,
};
