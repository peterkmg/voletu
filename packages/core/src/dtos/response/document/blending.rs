use super::*;

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

impl From<blending_document::ModelEx> for BlendingResponse {
  fn from(row: blending_document::ModelEx) -> Self {
    Self {
      id: row.id,
      document_number: row.document_number,
      date: row.date.to_rfc3339(),
      contractor_id: row.contractor_id,
      target_product_id: row.target_product_id,
      contractor_id_name: row
        .contractor
        .as_ref()
        .map(|company| company.common_name.clone()),
      target_product_id_name: row
        .target_product
        .as_ref()
        .map(|product| product.common_name.clone()),
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

impl From<blending_component::ModelEx> for BlendingComponentResponse {
  fn from(row: blending_component::ModelEx) -> Self {
    Self {
      id: row.id,
      blending_doc_id: row.blending_doc_id,
      storage_id: row.storage_id,
      source_product_id: row.source_product_id,
      amount_used: row.amount_used,
      source_product_id_name: row
        .source_product
        .as_ref()
        .map(|product| product.common_name.clone()),
      storage_id_name: row
        .storage
        .as_ref()
        .map(|storage| storage.common_name.clone()),
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

impl From<blending_result::ModelEx> for BlendingResultResponse {
  fn from(row: blending_result::ModelEx) -> Self {
    Self {
      id: row.id,
      blending_doc_id: row.blending_doc_id,
      storage_id: row.storage_id,
      produced_amount: row.produced_amount,
      storage_id_name: row
        .storage
        .as_ref()
        .map(|storage| storage.common_name.clone()),
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
      .cloned()
      .map(BlendingComponentResponse::from)
      .collect();

    let results = model
      .results
      .iter()
      .cloned()
      .map(BlendingResultResponse::from)
      .collect();

    let document = BlendingResponse::from(model);
    Ok(Self {
      document,
      components,
      results,
    })
  }
}
