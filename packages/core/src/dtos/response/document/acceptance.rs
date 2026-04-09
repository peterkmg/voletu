use super::*;

#[response_dto(service_fields(document))]
pub struct AcceptanceResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date_accepted: String,
  pub arrival_type: ArrivalType,
  pub source_entity: Option<String>,
  pub contractor_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub contractor_id_name: Option<String>,
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
      contractor_id: model.contractor_id,
      contractor_id_name: None,
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

impl From<acceptance_document::ModelEx> for AcceptanceResponse {
  fn from(model: acceptance_document::ModelEx) -> Self {
    Self {
      id: model.id,
      document_number: model.document_number,
      date_accepted: model.date_accepted.to_rfc3339(),
      arrival_type: model.arrival_type,
      source_entity: model.source_entity,
      contractor_id: model.contractor_id,
      contractor_id_name: model
        .contractor
        .as_ref()
        .map(|contractor| contractor.common_name.clone()),
      truck_waybill_id: model.truck_waybill_id,
      rail_waybill_id: model.rail_waybill_id,
      transit_dispatch_id: model.transit_dispatch_id,
      truck_waybill_id_name: model
        .truck_waybill
        .as_ref()
        .map(|truck_waybill| truck_waybill.document_number.clone()),
      rail_waybill_id_name: model
        .rail_waybill
        .as_ref()
        .map(|rail_waybill| rail_waybill.document_number.clone()),
      transit_dispatch_id_name: model
        .transit_dispatch
        .as_ref()
        .map(|dispatch| dispatch.document_number.clone()),
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
pub struct AcceptanceItemResponse {
  pub id: Uuid,
  pub acceptance_doc_id: Uuid,
  pub product_id: Uuid,
  pub storage_id: Uuid,
  pub accepted_amount: Decimal,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub storage_id_name: Option<String>,
}

impl From<acceptance_item::Model> for AcceptanceItemResponse {
  fn from(model: acceptance_item::Model) -> Self {
    Self {
      id: model.id,
      acceptance_doc_id: model.acceptance_doc_id,
      product_id: model.product_id,
      storage_id: model.storage_id,
      accepted_amount: model.accepted_amount,
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

impl From<acceptance_item::ModelEx> for AcceptanceItemResponse {
  fn from(model: acceptance_item::ModelEx) -> Self {
    Self {
      id: model.id,
      acceptance_doc_id: model.acceptance_doc_id,
      product_id: model.product_id,
      storage_id: model.storage_id,
      accepted_amount: model.accepted_amount,
      product_id_name: model
        .product
        .as_ref()
        .map(|product| product.common_name.clone()),
      storage_id_name: model
        .storage
        .as_ref()
        .map(|storage| storage.common_name.clone()),
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

    let doc_model = acceptance_document::Model::from(model);
    let document = AcceptanceResponse::from(doc_model);

    Ok(Self { document, items })
  }
}
