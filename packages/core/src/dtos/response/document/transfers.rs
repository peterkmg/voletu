use super::*;

#[response_dto(service_fields(document))]
pub struct PhysicalTransferResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub contractor_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub contractor_id_name: Option<String>,
  pub start_cargo_ops: String,
  pub end_cargo_ops: String,
  pub items: Vec<PhysicalTransferItemResponse>,
}

impl PhysicalTransferResponse {
  pub(crate) fn from_loaded_with_names(
    model: physical_storage_transfer::ModelEx,
    to_storage_names: &HashMap<Uuid, String>,
  ) -> Self {
    let contractor_id_name = model.contractor.as_ref().map(|c| c.common_name.clone());
    let items = model
      .items
      .iter()
      .cloned()
      .map(|item| PhysicalTransferItemResponse::from_loaded_with_names(item, to_storage_names))
      .collect();
    let mut response = Self::from(physical_storage_transfer::Model::from(model));
    response.contractor_id_name = contractor_id_name;
    response.items = items;
    response
  }
}

impl TryFrom<physical_storage_transfer::ModelEx> for PhysicalTransferResponse {
  type Error = ApiError;

  fn try_from(model: physical_storage_transfer::ModelEx) -> Result<Self, Self::Error> {
    let items = model
      .items
      .iter()
      .cloned()
      .map(PhysicalTransferItemResponse::from)
      .collect();

    let doc_model = physical_storage_transfer::Model::from(model);

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
      contractor_id: row.contractor_id,
      contractor_id_name: None,
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
      contractor_id: row.contractor_id,
      contractor_id_name: None,
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

#[response_dto(service_fields(common))]
pub struct PhysicalTransferItemResponse {
  pub id: Uuid,
  pub product_id: Uuid,
  pub from_storage_id: Uuid,
  pub to_storage_id: Uuid,
  pub amount: Decimal,
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

impl PhysicalTransferItemResponse {
  fn from_loaded_with_names(
    model: physical_transfer_item::ModelEx,
    to_storage_names: &HashMap<Uuid, String>,
  ) -> Self {
    let product_id_name = model.product.as_ref().map(|p| p.common_name.clone());
    let from_storage_id_name = model.from_storage.as_ref().map(|s| s.common_name.clone());
    let to_storage_id_name = to_storage_names.get(&model.to_storage_id).cloned();
    let mut response = Self::from(physical_transfer_item::Model::from(model));
    response.product_id_name = product_id_name;
    response.from_storage_id_name = from_storage_id_name;
    response.to_storage_id_name = to_storage_id_name;
    response
  }
}

impl From<physical_transfer_item::ModelEx> for PhysicalTransferItemResponse {
  fn from(model: physical_transfer_item::ModelEx) -> Self {
    Self::from(physical_transfer_item::Model::from(model))
  }
}

impl From<physical_transfer_item::Model> for PhysicalTransferItemResponse {
  fn from(row: physical_transfer_item::Model) -> Self {
    Self {
      id: row.id,
      product_id: row.product_id,
      from_storage_id: row.from_storage_id,
      to_storage_id: row.to_storage_id,
      amount: row.amount,
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

#[response_dto(service_fields(document))]
pub struct OwnershipTransferResponse {
  pub id: Uuid,
  pub date: String,
  pub items: Vec<OwnershipTransferItemResponse>,
}

impl OwnershipTransferResponse {
  pub(crate) fn from_loaded_with_names(
    model: ownership_transfer::ModelEx,
    contractor_names: &HashMap<Uuid, String>,
  ) -> Self {
    let items = model
      .items
      .iter()
      .cloned()
      .map(|item| OwnershipTransferItemResponse::from_loaded_with_names(item, contractor_names))
      .collect();
    let mut response = Self::from(ownership_transfer::Model::from(model));
    response.items = items;
    response
  }
}

impl TryFrom<ownership_transfer::ModelEx> for OwnershipTransferResponse {
  type Error = ApiError;

  fn try_from(model: ownership_transfer::ModelEx) -> Result<Self, Self::Error> {
    let items = model
      .items
      .iter()
      .cloned()
      .map(OwnershipTransferItemResponse::from)
      .collect();

    let doc_model = ownership_transfer::Model::from(model);

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

impl OwnershipTransferItemResponse {
  fn from_loaded_with_names(
    model: ownership_transfer_item::ModelEx,
    contractor_names: &HashMap<Uuid, String>,
  ) -> Self {
    let storage_id_name = model.storage.as_ref().map(|s| s.common_name.clone());
    let product_id_name = model.product.as_ref().map(|p| p.common_name.clone());
    let from_contractor_id_name = contractor_names.get(&model.from_contractor_id).cloned();
    let to_contractor_id_name = contractor_names.get(&model.to_contractor_id).cloned();
    let mut response = Self::from(ownership_transfer_item::Model::from(model));
    response.storage_id_name = storage_id_name;
    response.product_id_name = product_id_name;
    response.from_contractor_id_name = from_contractor_id_name;
    response.to_contractor_id_name = to_contractor_id_name;
    response
  }
}

impl From<ownership_transfer_item::ModelEx> for OwnershipTransferItemResponse {
  fn from(model: ownership_transfer_item::ModelEx) -> Self {
    Self::from(ownership_transfer_item::Model::from(model))
  }
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
