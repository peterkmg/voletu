use super::*;

#[response_dto(service_fields(common))]
pub struct TruckWaybillResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub sender_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub sender_id_name: Option<String>,
  pub base_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub base_id_name: Option<String>,
}

impl From<truck_waybill::Model> for TruckWaybillResponse {
  fn from(row: truck_waybill::Model) -> Self {
    Self {
      id: row.id,
      document_number: row.document_number,
      date: row.date.to_string(),
      sender_id: row.sender_id,
      sender_id_name: None,
      base_id: row.base_id,
      base_id_name: None,
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

impl From<truck_waybill::ModelEx> for TruckWaybillResponse {
  fn from(row: truck_waybill::ModelEx) -> Self {
    Self {
      id: row.id,
      document_number: row.document_number,
      date: row.date.to_string(),
      sender_id: row.sender_id,
      sender_id_name: row.sender.as_ref().map(|sender| sender.common_name.clone()),
      base_id: row.base_id,
      base_id_name: row.base.as_ref().map(|base| base.common_name.clone()),
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

impl From<truck_waybill_item::ModelEx> for TruckWaybillItemResponse {
  fn from(row: truck_waybill_item::ModelEx) -> Self {
    Self {
      id: row.id,
      truck_waybill_id: row.truck_waybill_id,
      product_id: row.product_id,
      declared_amount: row.declared_amount,
      product_id_name: row
        .product
        .as_ref()
        .map(|product| product.common_name.clone()),
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

impl From<truck_weight_doc::ModelEx> for TruckWeightDocResponse {
  fn from(row: truck_weight_doc::ModelEx) -> Self {
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

#[response_dto(service_fields(common))]
pub struct RailWaybillResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub sender_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub sender_id_name: Option<String>,
  pub base_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub base_id_name: Option<String>,
}

impl From<rail_waybill::Model> for RailWaybillResponse {
  fn from(row: rail_waybill::Model) -> Self {
    Self {
      id: row.id,
      document_number: row.document_number,
      date: row.date.to_string(),
      sender_id: row.sender_id,
      sender_id_name: None,
      base_id: row.base_id,
      base_id_name: None,
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

impl From<rail_waybill::ModelEx> for RailWaybillResponse {
  fn from(row: rail_waybill::ModelEx) -> Self {
    Self {
      id: row.id,
      document_number: row.document_number,
      date: row.date.to_string(),
      sender_id: row.sender_id,
      sender_id_name: row.sender.as_ref().map(|sender| sender.common_name.clone()),
      base_id: row.base_id,
      base_id_name: row.base.as_ref().map(|base| base.common_name.clone()),
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

impl From<rail_wagon_manifest::ModelEx> for RailWagonManifestResponse {
  fn from(row: rail_wagon_manifest::ModelEx) -> Self {
    let measurements = row
      .measurements
      .iter()
      .cloned()
      .map(RailWagonMeasurementResponse::from)
      .collect::<Vec<_>>();
    let weights = row
      .weights
      .iter()
      .cloned()
      .map(RailWagonWeightResponse::from)
      .collect::<Vec<_>>();

    Self {
      id: row.id,
      rail_waybill_id: row.rail_waybill_id,
      wagon_number: row.wagon_number,
      product_id: row.product_id,
      declared_volume: row.declared_volume,
      declared_density: row.declared_density,
      declared_mass: row.declared_mass,
      measurements: (!measurements.is_empty()).then_some(measurements),
      weights: (!weights.is_empty()).then_some(weights),
      product_id_name: row
        .product
        .as_ref()
        .map(|product| product.common_name.clone()),
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

impl From<rail_wagon_measurement::ModelEx> for RailWagonMeasurementResponse {
  fn from(row: rail_wagon_measurement::ModelEx) -> Self {
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

impl From<rail_wagon_weight::ModelEx> for RailWagonWeightResponse {
  fn from(row: rail_wagon_weight::ModelEx) -> Self {
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

#[response_dto]
pub struct TruckWaybillCompositeResponse {
  pub waybill: TruckWaybillResponse,
  pub items: Option<Vec<TruckWaybillItemResponse>>,
  pub weight_docs: Option<Vec<TruckWeightDocResponse>>,
}

#[response_dto]
pub struct RailWaybillCompositeResponse {
  pub waybill: RailWaybillResponse,
  pub wagon_manifests: Option<Vec<RailWagonManifestResponse>>,
}
