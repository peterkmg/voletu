use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::services::common::resolve::{FkIdCollector, ResolveFkNames, ResolvedNames};

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

/// Response DTO for the `acceptance_document` entity.
#[response_dto(service_fields(document))]
pub struct AcceptanceResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date_accepted: String,
  pub arrival_type: ArrivalType,
  pub source_entity: Option<String>,
  pub truck_waybill_id: Option<Uuid>,
  pub rail_waybill_id: Option<Uuid>,
  pub transit_dispatch_id: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub truck_waybill_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub rail_waybill_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub transit_dispatch_id_name: Option<String>,
}

impl From<acceptance_document::Model> for AcceptanceResponse {
  fn from(model: acceptance_document::Model) -> Self {
    Self {
      id: model.id,
      document_number: model.document_number,
      date_accepted: model.date_accepted.to_rfc3339(),
      arrival_type: model.arrival_type,
      source_entity: model.source_entity,
      truck_waybill_id: model.truck_waybill_id,
      rail_waybill_id: model.rail_waybill_id,
      transit_dispatch_id: model.transit_dispatch_id,
      truck_waybill_id_name: None,
      rail_waybill_id_name: None,
      transit_dispatch_id_name: None,
      created_at: model.created_at.to_rfc3339(),
      updated_at: model.updated_at.to_rfc3339(),
      deleted_at: model.deleted_at.map(|v| v.to_rfc3339()),
      created_by: model.created_by,
      updated_by: model.updated_by,
      deleted_by: model.deleted_by,
      origin_db_id: model.origin_db_id,
      status: model.status,
      executed_at: model.executed_at.map(|v| v.to_rfc3339()),
      executed_by: model.executed_by,
      reverted_at: model.reverted_at.map(|v| v.to_rfc3339()),
      reverted_by: model.reverted_by,
    }
  }
}

/// Response DTO for the `acceptance_item` entity.
#[response_dto(service_fields(common))]
pub struct AcceptanceItemResponse {
  pub id: Uuid,
  pub acceptance_doc_id: Uuid,
  pub product_id: Uuid,
  pub contractor_id: Uuid,
  pub storage_id: Uuid,
  pub accepted_amount: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub contractor_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub storage_id_name: Option<String>,
}

/// Composite response DTO used by acceptance aggregate endpoints.
#[response_dto]
pub struct AcceptanceCompositeResponse {
  #[serde(flatten)]
  pub document: AcceptanceResponse,
  pub items: Vec<AcceptanceItemResponse>,
}

impl TryFrom<acceptance_document::ModelEx> for AcceptanceCompositeResponse {
  type Error = ApiError;

  fn try_from(model: acceptance_document::ModelEx) -> Result<Self, Self::Error> {
    let items = model
      .items
      .iter()
      .map(|item| AcceptanceItemResponse::from(acceptance_item::Model::from(item.clone())))
      .collect();

    let doc_model = acceptance_document::Model::try_from(model).map_err(|e| {
      ApiError::Internal(anyhow::anyhow!(
        "Failed to convert acceptance document model: {}",
        e
      ))
    })?;

    let document = AcceptanceResponse::from(doc_model);
    Ok(Self { document, items })
  }
}

/// Response DTO for the `dispatch_document` entity.
#[response_dto(service_fields(document))]
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
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub contractor_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub destination_base_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub exporter_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub port_id_name: Option<String>,
}

impl From<dispatch_document::Model> for DispatchResponse {
  fn from(model: dispatch_document::Model) -> Self {
    Self {
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
      contractor_id_name: None,
      destination_base_id_name: None,
      exporter_id_name: None,
      port_id_name: None,
      created_at: model.created_at.to_rfc3339(),
      updated_at: model.updated_at.to_rfc3339(),
      deleted_at: model.deleted_at.map(|v| v.to_rfc3339()),
      created_by: model.created_by,
      updated_by: model.updated_by,
      deleted_by: model.deleted_by,
      origin_db_id: model.origin_db_id,
      status: model.status,
      executed_at: model.executed_at.map(|v| v.to_rfc3339()),
      executed_by: model.executed_by,
      reverted_at: model.reverted_at.map(|v| v.to_rfc3339()),
      reverted_by: model.reverted_by,
    }
  }
}

/// Response DTO for the `dispatch_item` entity.
#[response_dto(service_fields(common))]
pub struct DispatchItemResponse {
  pub id: Uuid,
  pub dispatch_doc_id: Uuid,
  pub product_id: Uuid,
  pub storage_id: Uuid,
  pub dispatched_amount: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub storage_id_name: Option<String>,
}

impl From<dispatch_item::Model> for DispatchItemResponse {
  fn from(item: dispatch_item::Model) -> Self {
    Self {
      id: item.id,
      dispatch_doc_id: item.dispatch_doc_id,
      product_id: item.product_id,
      storage_id: item.storage_id,
      dispatched_amount: item.dispatched_amount,
      product_id_name: None,
      storage_id_name: None,
      created_at: item.created_at.to_rfc3339(),
      updated_at: item.updated_at.to_rfc3339(),
      deleted_at: item.deleted_at.map(|v| v.to_rfc3339()),
      created_by: item.created_by,
      updated_by: item.updated_by,
      deleted_by: item.deleted_by,
      origin_db_id: item.origin_db_id,
    }
  }
}

impl From<acceptance_item::Model> for AcceptanceItemResponse {
  fn from(model: acceptance_item::Model) -> Self {
    Self {
      id: model.id,
      acceptance_doc_id: model.acceptance_doc_id,
      product_id: model.product_id,
      contractor_id: model.contractor_id,
      storage_id: model.storage_id,
      accepted_amount: model.accepted_amount,
      contractor_id_name: None,
      product_id_name: None,
      storage_id_name: None,
      created_at: model.created_at.to_rfc3339(),
      updated_at: model.updated_at.to_rfc3339(),
      deleted_at: model.deleted_at.map(|v| v.to_rfc3339()),
      created_by: model.created_by,
      updated_by: model.updated_by,
      deleted_by: model.deleted_by,
      origin_db_id: model.origin_db_id,
    }
  }
}

/// Response DTO for the `dispatch_storage_measurement` entity.
#[response_dto(service_fields(common))]
pub struct DispatchMeasurementResponse {
  pub id: Uuid,
  pub dispatch_doc_id: Uuid,
  pub storage_id: Uuid,
  pub before_height: Option<Decimal>,
  pub before_volume: Option<Decimal>,
  pub before_density: Option<Decimal>,
  pub before_mass: Decimal,
  pub after_height: Option<Decimal>,
  pub after_volume: Option<Decimal>,
  pub after_density: Option<Decimal>,
  pub after_mass: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub storage_id_name: Option<String>,
}

impl From<dispatch_storage_measurement::Model> for DispatchMeasurementResponse {
  fn from(measurement: dispatch_storage_measurement::Model) -> Self {
    Self {
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
      storage_id_name: None,
      created_at: measurement.created_at.to_rfc3339(),
      updated_at: measurement.updated_at.to_rfc3339(),
      deleted_at: measurement.deleted_at.map(|v| v.to_rfc3339()),
      created_by: measurement.created_by,
      updated_by: measurement.updated_by,
      deleted_by: measurement.deleted_by,
      origin_db_id: measurement.origin_db_id,
    }
  }
}

/// Composite response DTO used by dispatch aggregate endpoints.
#[response_dto]
pub struct DispatchCompositeResponse {
  #[serde(flatten)]
  pub document: DispatchResponse,
  pub items: Vec<DispatchItemResponse>,
  pub storage_measurements: Vec<DispatchMeasurementResponse>,
}

impl TryFrom<dispatch_document::ModelEx> for DispatchCompositeResponse {
  type Error = ApiError;

  fn try_from(model: dispatch_document::ModelEx) -> Result<Self, Self::Error> {
    let items = model
      .items
      .iter()
      .map(|item| DispatchItemResponse::from(dispatch_item::Model::from(item.clone())))
      .collect();

    let storage_measurements = model
      .storage_measurements
      .iter()
      .map(|item| {
        DispatchMeasurementResponse::from(dispatch_storage_measurement::Model::from(item.clone()))
      })
      .collect();

    let doc_model = dispatch_document::Model::try_from(model).map_err(|e| {
      ApiError::Internal(anyhow::anyhow!(
        "Failed to convert dispatch document model: {}",
        e
      ))
    })?;

    let document = DispatchResponse::from(doc_model);
    Ok(Self {
      document,
      items,
      storage_measurements,
    })
  }
}

/// Response DTO for the `physical_storage_transfer` entity.
#[response_dto(service_fields(document))]
pub struct PhysicalTransferResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub start_cargo_ops: String,
  pub end_cargo_ops: String,
  pub items: Vec<PhysicalTransferItemResponse>,
}

impl TryFrom<physical_storage_transfer::ModelEx> for PhysicalTransferResponse {
  type Error = ApiError;

  fn try_from(model: physical_storage_transfer::ModelEx) -> Result<Self, Self::Error> {
    let items = model
      .items
      .iter()
      .map(|item| {
        PhysicalTransferItemResponse::from(physical_transfer_item::Model::from(item.clone()))
      })
      .collect();

    let doc_model = physical_storage_transfer::Model::try_from(model).map_err(|e| {
      ApiError::Internal(anyhow::anyhow!(
        "Failed to convert physical transfer model: {}",
        e
      ))
    })?;

    Ok(Self {
      items,
      ..Self::from(doc_model)
    })
  }
}

impl From<physical_storage_transfer::Model> for PhysicalTransferResponse {
  fn from(row: physical_storage_transfer::Model) -> Self {
    Self {
      id: row.id,
      document_number: row.document_number,
      date: row.date.to_rfc3339(),
      start_cargo_ops: row.start_cargo_ops.to_rfc3339(),
      end_cargo_ops: row.end_cargo_ops.to_rfc3339(),
      items: Vec::new(),
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
      status: row.status,
      executed_at: row.executed_at.map(|v| v.to_rfc3339()),
      executed_by: row.executed_by,
      reverted_at: row.reverted_at.map(|v| v.to_rfc3339()),
      reverted_by: row.reverted_by,
    }
  }
}

impl
  From<(
    physical_storage_transfer::Model,
    Vec<PhysicalTransferItemResponse>,
  )> for PhysicalTransferResponse
{
  fn from(
    (row, items): (
      physical_storage_transfer::Model,
      Vec<PhysicalTransferItemResponse>,
    ),
  ) -> Self {
    Self {
      id: row.id,
      document_number: row.document_number,
      date: row.date.to_rfc3339(),
      start_cargo_ops: row.start_cargo_ops.to_rfc3339(),
      end_cargo_ops: row.end_cargo_ops.to_rfc3339(),
      items,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
      status: row.status,
      executed_at: row.executed_at.map(|v| v.to_rfc3339()),
      executed_by: row.executed_by,
      reverted_at: row.reverted_at.map(|v| v.to_rfc3339()),
      reverted_by: row.reverted_by,
    }
  }
}

/// Response DTO for the `physical_transfer_item` entity.
#[response_dto(service_fields(common))]
pub struct PhysicalTransferItemResponse {
  pub id: Uuid,
  pub contractor_id: Uuid,
  pub product_id: Uuid,
  pub from_storage_id: Uuid,
  pub to_storage_id: Uuid,
  pub amount: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub contractor_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub from_storage_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub to_storage_id_name: Option<String>,
}

impl From<physical_transfer_item::Model> for PhysicalTransferItemResponse {
  fn from(row: physical_transfer_item::Model) -> Self {
    Self {
      id: row.id,
      contractor_id: row.contractor_id,
      product_id: row.product_id,
      from_storage_id: row.from_storage_id,
      to_storage_id: row.to_storage_id,
      amount: row.amount,
      contractor_id_name: None,
      product_id_name: None,
      from_storage_id_name: None,
      to_storage_id_name: None,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

/// Response DTO for the `ownership_transfer` entity.
#[response_dto(service_fields(document))]
pub struct OwnershipTransferResponse {
  pub id: Uuid,
  pub date: String,
  pub items: Vec<OwnershipTransferItemResponse>,
}

impl TryFrom<ownership_transfer::ModelEx> for OwnershipTransferResponse {
  type Error = ApiError;

  fn try_from(model: ownership_transfer::ModelEx) -> Result<Self, Self::Error> {
    let items = model
      .items
      .iter()
      .map(|item| {
        OwnershipTransferItemResponse::from(ownership_transfer_item::Model::from(item.clone()))
      })
      .collect();

    let doc_model = ownership_transfer::Model::try_from(model).map_err(|e| {
      ApiError::Internal(anyhow::anyhow!(
        "Failed to convert ownership transfer model: {}",
        e
      ))
    })?;

    Ok(Self {
      items,
      ..Self::from(doc_model)
    })
  }
}

impl From<ownership_transfer::Model> for OwnershipTransferResponse {
  fn from(row: ownership_transfer::Model) -> Self {
    Self {
      id: row.id,
      date: row.date.to_rfc3339(),
      items: Vec::new(),
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
      status: row.status,
      executed_at: row.executed_at.map(|v| v.to_rfc3339()),
      executed_by: row.executed_by,
      reverted_at: row.reverted_at.map(|v| v.to_rfc3339()),
      reverted_by: row.reverted_by,
    }
  }
}

impl
  From<(
    ownership_transfer::Model,
    Vec<OwnershipTransferItemResponse>,
  )> for OwnershipTransferResponse
{
  fn from(
    (row, items): (
      ownership_transfer::Model,
      Vec<OwnershipTransferItemResponse>,
    ),
  ) -> Self {
    Self {
      id: row.id,
      date: row.date.to_rfc3339(),
      items,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
      status: row.status,
      executed_at: row.executed_at.map(|v| v.to_rfc3339()),
      executed_by: row.executed_by,
      reverted_at: row.reverted_at.map(|v| v.to_rfc3339()),
      reverted_by: row.reverted_by,
    }
  }
}

/// Response DTO for the `ownership_transfer_item` entity.
#[response_dto(service_fields(common))]
pub struct OwnershipTransferItemResponse {
  pub id: Uuid,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub from_contractor_id: Uuid,
  pub to_contractor_id: Uuid,
  pub amount: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub storage_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub from_contractor_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub to_contractor_id_name: Option<String>,
}

impl From<ownership_transfer_item::Model> for OwnershipTransferItemResponse {
  fn from(row: ownership_transfer_item::Model) -> Self {
    Self {
      id: row.id,
      storage_id: row.storage_id,
      product_id: row.product_id,
      from_contractor_id: row.from_contractor_id,
      to_contractor_id: row.to_contractor_id,
      amount: row.amount,
      storage_id_name: None,
      product_id_name: None,
      from_contractor_id_name: None,
      to_contractor_id_name: None,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

/// Response DTO for the `blending_document` entity.
#[response_dto(service_fields(document))]
pub struct BlendingResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub contractor_id: Uuid,
  pub target_product_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub contractor_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub target_product_id_name: Option<String>,
}

impl From<blending_document::Model> for BlendingResponse {
  fn from(row: blending_document::Model) -> Self {
    Self {
      id: row.id,
      document_number: row.document_number,
      date: row.date.to_rfc3339(),
      contractor_id: row.contractor_id,
      target_product_id: row.target_product_id,
      contractor_id_name: None,
      target_product_id_name: None,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
      status: row.status,
      executed_at: row.executed_at.map(|v| v.to_rfc3339()),
      executed_by: row.executed_by,
      reverted_at: row.reverted_at.map(|v| v.to_rfc3339()),
      reverted_by: row.reverted_by,
    }
  }
}

/// Response DTO for the `blending_component` entity.
#[response_dto(service_fields(common))]
pub struct BlendingComponentResponse {
  pub id: Uuid,
  pub blending_doc_id: Uuid,
  pub storage_id: Uuid,
  pub source_product_id: Uuid,
  pub amount_used: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub source_product_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub storage_id_name: Option<String>,
}

impl From<blending_component::Model> for BlendingComponentResponse {
  fn from(row: blending_component::Model) -> Self {
    Self {
      id: row.id,
      blending_doc_id: row.blending_doc_id,
      storage_id: row.storage_id,
      source_product_id: row.source_product_id,
      amount_used: row.amount_used,
      source_product_id_name: None,
      storage_id_name: None,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

/// Response DTO for the `blending_result` entity.
#[response_dto(service_fields(common))]
pub struct BlendingResultResponse {
  pub id: Uuid,
  pub blending_doc_id: Uuid,
  pub storage_id: Uuid,
  pub produced_amount: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub storage_id_name: Option<String>,
}

impl From<blending_result::Model> for BlendingResultResponse {
  fn from(row: blending_result::Model) -> Self {
    Self {
      id: row.id,
      blending_doc_id: row.blending_doc_id,
      storage_id: row.storage_id,
      produced_amount: row.produced_amount,
      storage_id_name: None,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

/// Response DTO for the `inventory_reconciliation` entity.
#[response_dto(service_fields(document))]
pub struct InventoryReconciliationResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub warehouse_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub warehouse_id_name: Option<String>,
}

impl From<inventory_reconciliation::Model> for InventoryReconciliationResponse {
  fn from(row: inventory_reconciliation::Model) -> Self {
    Self {
      id: row.id,
      document_number: row.document_number,
      date: row.date.to_rfc3339(),
      warehouse_id: row.warehouse_id,
      warehouse_id_name: None,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
      status: row.status,
      executed_at: row.executed_at.map(|v| v.to_rfc3339()),
      executed_by: row.executed_by,
      reverted_at: row.reverted_at.map(|v| v.to_rfc3339()),
      reverted_by: row.reverted_by,
    }
  }
}

/// Response DTO for the `inventory_adjustment` entity.
#[response_dto(service_fields(common))]
pub struct InventoryAdjustmentResponse {
  pub id: Uuid,
  pub reconciliation_id: Uuid,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub contractor_id: Uuid,
  pub adjustment_type: AdjustmentType,
  pub amount: Decimal,
  pub reason: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub storage_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub contractor_id_name: Option<String>,
}

impl From<inventory_adjustment::Model> for InventoryAdjustmentResponse {
  fn from(row: inventory_adjustment::Model) -> Self {
    Self {
      id: row.id,
      reconciliation_id: row.reconciliation_id,
      storage_id: row.storage_id,
      product_id: row.product_id,
      contractor_id: row.contractor_id,
      adjustment_type: row.adjustment_type,
      amount: row.amount,
      reason: row.reason,
      storage_id_name: None,
      product_id_name: None,
      contractor_id_name: None,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

/// Composite response DTO used by blending aggregate endpoints.
#[response_dto]
pub struct BlendingCompositeResponse {
  pub document: BlendingResponse,
  pub components: Vec<BlendingComponentResponse>,
  pub results: Vec<BlendingResultResponse>,
}

impl TryFrom<blending_document::ModelEx> for BlendingCompositeResponse {
  type Error = ApiError;

  fn try_from(model: blending_document::ModelEx) -> Result<Self, Self::Error> {
    let components = model
      .components
      .iter()
      .map(|item| BlendingComponentResponse::from(blending_component::Model::from(item.clone())))
      .collect();

    let results = model
      .results
      .iter()
      .map(|item| BlendingResultResponse::from(blending_result::Model::from(item.clone())))
      .collect();

    let doc_model = blending_document::Model::try_from(model).map_err(|e| {
      ApiError::Internal(anyhow::anyhow!(
        "Failed to convert blending document model: {}",
        e
      ))
    })?;

    let document = BlendingResponse::from(doc_model);
    Ok(Self {
      document,
      components,
      results,
    })
  }
}

/// Response DTO for the `truck_waybill` entity.
#[response_dto(service_fields(common))]
pub struct TruckWaybillResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub sender_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub sender_id_name: Option<String>,
}

impl From<truck_waybill::Model> for TruckWaybillResponse {
  fn from(row: truck_waybill::Model) -> Self {
    Self {
      id: row.id,
      document_number: row.document_number,
      date: row.date.to_string(),
      sender_id: row.sender_id,
      sender_id_name: None,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

/// Response DTO for the `truck_waybill_item` entity.
#[response_dto(service_fields(common))]
pub struct TruckWaybillItemResponse {
  pub id: Uuid,
  pub truck_waybill_id: Uuid,
  pub product_id: Uuid,
  pub declared_amount: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_id_name: Option<String>,
}

impl From<truck_waybill_item::Model> for TruckWaybillItemResponse {
  fn from(row: truck_waybill_item::Model) -> Self {
    Self {
      id: row.id,
      truck_waybill_id: row.truck_waybill_id,
      product_id: row.product_id,
      declared_amount: row.declared_amount,
      product_id_name: None,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

/// Response DTO for the `truck_weight_doc` entity.
#[response_dto(service_fields(common))]
pub struct TruckWeightDocResponse {
  pub id: Uuid,
  pub truck_waybill_id: Uuid,
  pub total_weight: Decimal,
}

impl From<truck_weight_doc::Model> for TruckWeightDocResponse {
  fn from(row: truck_weight_doc::Model) -> Self {
    Self {
      id: row.id,
      truck_waybill_id: row.truck_waybill_id,
      total_weight: row.total_weight,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

/// Response DTO for the `rail_waybill` entity.
#[response_dto(service_fields(common))]
pub struct RailWaybillResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub sender_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub sender_id_name: Option<String>,
}

impl From<rail_waybill::Model> for RailWaybillResponse {
  fn from(row: rail_waybill::Model) -> Self {
    Self {
      id: row.id,
      document_number: row.document_number,
      date: row.date.to_string(),
      sender_id: row.sender_id,
      sender_id_name: None,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

/// Response DTO for the `rail_wagon_manifest` entity.
#[response_dto(service_fields(common))]
pub struct RailWagonManifestResponse {
  pub id: Uuid,
  pub rail_waybill_id: Uuid,
  pub wagon_number: String,
  pub product_id: Uuid,
  pub declared_volume: Decimal,
  pub declared_density: Decimal,
  pub declared_mass: Decimal,
  pub measurements: Option<Vec<RailWagonMeasurementResponse>>,
  pub weights: Option<Vec<RailWagonWeightResponse>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_id_name: Option<String>,
}

impl From<rail_wagon_manifest::Model> for RailWagonManifestResponse {
  fn from(row: rail_wagon_manifest::Model) -> Self {
    Self {
      id: row.id,
      rail_waybill_id: row.rail_waybill_id,
      wagon_number: row.wagon_number,
      product_id: row.product_id,
      declared_volume: row.declared_volume,
      declared_density: row.declared_density,
      declared_mass: row.declared_mass,
      measurements: None,
      weights: None,
      product_id_name: None,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

/// Response DTO for the `rail_wagon_measurement` entity.
#[response_dto(service_fields(common))]
pub struct RailWagonMeasurementResponse {
  pub id: Uuid,
  pub wagon_manifest_id: Uuid,
  pub measured_height: Decimal,
  pub lab_density: Option<Decimal>,
  pub calculated_mass: Decimal,
}

impl From<rail_wagon_measurement::Model> for RailWagonMeasurementResponse {
  fn from(row: rail_wagon_measurement::Model) -> Self {
    Self {
      id: row.id,
      wagon_manifest_id: row.wagon_manifest_id,
      measured_height: row.measured_height,
      lab_density: row.lab_density,
      calculated_mass: row.calculated_mass,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

/// Response DTO for the `rail_wagon_weight` entity.
#[response_dto(service_fields(common))]
pub struct RailWagonWeightResponse {
  pub id: Uuid,
  pub wagon_manifest_id: Uuid,
  pub gross_weight: Decimal,
  pub tare_weight: Decimal,
  pub net_product_weight: Decimal,
}

impl From<rail_wagon_weight::Model> for RailWagonWeightResponse {
  fn from(row: rail_wagon_weight::Model) -> Self {
    Self {
      id: row.id,
      wagon_manifest_id: row.wagon_manifest_id,
      gross_weight: row.gross_weight,
      tare_weight: row.tare_weight,
      net_product_weight: row.net_product_weight,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

/// Composite response DTO used by truck waybill aggregate endpoints.
#[response_dto]
pub struct TruckWaybillCompositeResponse {
  pub waybill: TruckWaybillResponse,
  pub items: Option<Vec<TruckWaybillItemResponse>>,
  pub weight_docs: Option<Vec<TruckWeightDocResponse>>,
}

/// Composite response DTO used by rail waybill aggregate endpoints.
#[response_dto]
pub struct RailWaybillCompositeResponse {
  pub waybill: RailWaybillResponse,
  pub wagon_manifests: Option<Vec<RailWagonManifestResponse>>,
}

impl ResolveFkNames for AcceptanceResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    if let Some(id) = self.truck_waybill_id { c.truck_waybill_ids.insert(id); }
    if let Some(id) = self.rail_waybill_id { c.rail_waybill_ids.insert(id); }
    if let Some(id) = self.transit_dispatch_id { c.dispatch_document_ids.insert(id); }
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.truck_waybill_id_name = self.truck_waybill_id.and_then(|id| r.truck_waybills.get(&id).cloned());
    self.rail_waybill_id_name = self.rail_waybill_id.and_then(|id| r.rail_waybills.get(&id).cloned());
    self.transit_dispatch_id_name = self.transit_dispatch_id.and_then(|id| r.dispatch_documents.get(&id).cloned());
  }
}

impl ResolveFkNames for AcceptanceItemResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.company_ids.insert(self.contractor_id);
    c.product_ids.insert(self.product_id);
    c.storage_ids.insert(self.storage_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.contractor_id_name = r.companies.get(&self.contractor_id).cloned();
    self.product_id_name = r.products.get(&self.product_id).cloned();
    self.storage_id_name = r.storages.get(&self.storage_id).cloned();
  }
}

impl ResolveFkNames for DispatchResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.company_ids.insert(self.contractor_id);
    if let Some(id) = self.destination_base_id { c.base_ids.insert(id); }
    if let Some(id) = self.exporter_id { c.company_ids.insert(id); }
    if let Some(id) = self.port_id { c.port_ids.insert(id); }
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.contractor_id_name = r.companies.get(&self.contractor_id).cloned();
    self.destination_base_id_name = self.destination_base_id.and_then(|id| r.bases.get(&id).cloned());
    self.exporter_id_name = self.exporter_id.and_then(|id| r.companies.get(&id).cloned());
    self.port_id_name = self.port_id.and_then(|id| r.ports.get(&id).cloned());
  }
}

impl ResolveFkNames for DispatchItemResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.product_ids.insert(self.product_id);
    c.storage_ids.insert(self.storage_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.product_id_name = r.products.get(&self.product_id).cloned();
    self.storage_id_name = r.storages.get(&self.storage_id).cloned();
  }
}

impl ResolveFkNames for DispatchMeasurementResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.storage_ids.insert(self.storage_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.storage_id_name = r.storages.get(&self.storage_id).cloned();
  }
}

impl ResolveFkNames for PhysicalTransferItemResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.company_ids.insert(self.contractor_id);
    c.product_ids.insert(self.product_id);
    c.storage_ids.insert(self.from_storage_id);
    c.storage_ids.insert(self.to_storage_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.contractor_id_name = r.companies.get(&self.contractor_id).cloned();
    self.product_id_name = r.products.get(&self.product_id).cloned();
    self.from_storage_id_name = r.storages.get(&self.from_storage_id).cloned();
    self.to_storage_id_name = r.storages.get(&self.to_storage_id).cloned();
  }
}

impl ResolveFkNames for OwnershipTransferItemResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.storage_ids.insert(self.storage_id);
    c.product_ids.insert(self.product_id);
    c.company_ids.insert(self.from_contractor_id);
    c.company_ids.insert(self.to_contractor_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.storage_id_name = r.storages.get(&self.storage_id).cloned();
    self.product_id_name = r.products.get(&self.product_id).cloned();
    self.from_contractor_id_name = r.companies.get(&self.from_contractor_id).cloned();
    self.to_contractor_id_name = r.companies.get(&self.to_contractor_id).cloned();
  }
}

impl ResolveFkNames for BlendingResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.company_ids.insert(self.contractor_id);
    c.product_ids.insert(self.target_product_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.contractor_id_name = r.companies.get(&self.contractor_id).cloned();
    self.target_product_id_name = r.products.get(&self.target_product_id).cloned();
  }
}

impl ResolveFkNames for BlendingComponentResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.product_ids.insert(self.source_product_id);
    c.storage_ids.insert(self.storage_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.source_product_id_name = r.products.get(&self.source_product_id).cloned();
    self.storage_id_name = r.storages.get(&self.storage_id).cloned();
  }
}

impl ResolveFkNames for BlendingResultResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.storage_ids.insert(self.storage_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.storage_id_name = r.storages.get(&self.storage_id).cloned();
  }
}

impl ResolveFkNames for InventoryReconciliationResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.warehouse_ids.insert(self.warehouse_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.warehouse_id_name = r.warehouses.get(&self.warehouse_id).cloned();
  }
}

impl ResolveFkNames for InventoryAdjustmentResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.storage_ids.insert(self.storage_id);
    c.product_ids.insert(self.product_id);
    c.company_ids.insert(self.contractor_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.storage_id_name = r.storages.get(&self.storage_id).cloned();
    self.product_id_name = r.products.get(&self.product_id).cloned();
    self.contractor_id_name = r.companies.get(&self.contractor_id).cloned();
  }
}

impl ResolveFkNames for TruckWaybillResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.company_ids.insert(self.sender_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.sender_id_name = r.companies.get(&self.sender_id).cloned();
  }
}

impl ResolveFkNames for TruckWaybillItemResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.product_ids.insert(self.product_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.product_id_name = r.products.get(&self.product_id).cloned();
  }
}

impl ResolveFkNames for RailWaybillResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.company_ids.insert(self.sender_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.sender_id_name = r.companies.get(&self.sender_id).cloned();
  }
}

impl ResolveFkNames for RailWagonManifestResponse {
  fn collect_fk_ids(&self, c: &mut FkIdCollector) {
    c.product_ids.insert(self.product_id);
  }
  fn apply_resolved_names(&mut self, r: &ResolvedNames) {
    self.product_id_name = r.products.get(&self.product_id).cloned();
  }
}
