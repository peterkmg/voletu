use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::{
  api::ApiError,
  entities::{dispatch_document, dispatch_item, dispatch_storage_measurement},
  enums::{BunkerType, DispatchMethod, DispatchPurpose},
};

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

impl DispatchResponse {
  pub(crate) fn from_loaded(
    model: dispatch_document::ModelEx,
    exporter_id_name: Option<String>,
  ) -> Self {
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
      contractor_id_name: model
        .contractor
        .as_ref()
        .map(|contractor| contractor.common_name.clone()),
      destination_base_id_name: model
        .destination_base
        .as_ref()
        .map(|base| base.common_name.clone()),
      exporter_id_name,
      port_id_name: model.port.as_ref().map(|port| port.common_name.clone()),
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

impl From<dispatch_item::ModelEx> for DispatchItemResponse {
  fn from(item: dispatch_item::ModelEx) -> Self {
    Self {
      id: item.id,
      dispatch_doc_id: item.dispatch_doc_id,
      product_id: item.product_id,
      storage_id: item.storage_id,
      dispatched_amount: item.dispatched_amount,
      product_id_name: item
        .product
        .as_ref()
        .map(|product| product.common_name.clone()),
      storage_id_name: item
        .storage
        .as_ref()
        .map(|storage| storage.common_name.clone()),
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

impl From<dispatch_storage_measurement::ModelEx> for DispatchMeasurementResponse {
  fn from(measurement: dispatch_storage_measurement::ModelEx) -> Self {
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
      storage_id_name: measurement
        .storage
        .as_ref()
        .map(|storage| storage.common_name.clone()),
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

    let doc_model = dispatch_document::Model::from(model);
    let document = DispatchResponse::from(doc_model);

    Ok(Self {
      document,
      items,
      storage_measurements,
    })
  }
}
