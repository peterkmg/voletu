use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::{
  api::ApiError,
  entities::{inventory_adjustment, inventory_reconciliation},
  enums::AdjustmentType,
};

#[response_dto(service_fields(document))]
pub struct InventoryReconciliationResponse {
  pub id: Uuid,
  pub document_number: String,
  pub date: String,
  pub contractor_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub contractor_id_name: Option<String>,
  pub warehouse_id: Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub warehouse_id_name: Option<String>,
}

impl From<inventory_reconciliation::ModelEx> for InventoryReconciliationResponse {
  fn from(model: inventory_reconciliation::ModelEx) -> Self {
    let contractor_id_name = model.contractor.as_ref().map(|c| c.common_name.clone());
    let warehouse_id_name = model.warehouse.as_ref().map(|w| w.common_name.clone());
    let mut response = Self::from(inventory_reconciliation::Model::from(model));
    response.contractor_id_name = contractor_id_name;
    response.warehouse_id_name = warehouse_id_name;
    response
  }
}

impl From<inventory_reconciliation::Model> for InventoryReconciliationResponse {
  fn from(row: inventory_reconciliation::Model) -> Self {
    Self {
      id: row.id,
      document_number: row.document_number,
      date: row.date.to_rfc3339(),
      contractor_id: row.contractor_id,
      contractor_id_name: None,
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

#[response_dto(service_fields(common))]
pub struct InventoryAdjustmentResponse {
  pub id: Uuid,
  pub reconciliation_id: Uuid,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub adjustment_type: AdjustmentType,
  pub amount: Decimal,
  pub reason: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub storage_id_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(nullable)]
  pub product_id_name: Option<String>,
}

impl From<inventory_adjustment::Model> for InventoryAdjustmentResponse {
  fn from(row: inventory_adjustment::Model) -> Self {
    Self {
      id: row.id,
      reconciliation_id: row.reconciliation_id,
      storage_id: row.storage_id,
      product_id: row.product_id,
      adjustment_type: row.adjustment_type,
      amount: row.amount,
      reason: row.reason,
      storage_id_name: None,
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

impl From<inventory_adjustment::ModelEx> for InventoryAdjustmentResponse {
  fn from(model: inventory_adjustment::ModelEx) -> Self {
    let storage_id_name = model.storage.as_ref().map(|s| s.common_name.clone());
    let product_id_name = model.product.as_ref().map(|p| p.common_name.clone());
    let mut response = Self::from(inventory_adjustment::Model::from(model));
    response.storage_id_name = storage_id_name;
    response.product_id_name = product_id_name;
    response
  }
}

#[response_dto]
pub struct InventoryReconciliationCompositeResponse {
  #[serde(flatten)]
  pub document: InventoryReconciliationResponse,
  pub adjustments: Vec<InventoryAdjustmentResponse>,
}

impl TryFrom<inventory_reconciliation::ModelEx> for InventoryReconciliationCompositeResponse {
  type Error = ApiError;

  fn try_from(model: inventory_reconciliation::ModelEx) -> Result<Self, Self::Error> {
    let adjustments = model
      .adjustments
      .iter()
      .map(|adj| InventoryAdjustmentResponse::from(adj.clone()))
      .collect();

    let document = InventoryReconciliationResponse::from(model);

    Ok(Self {
      document,
      adjustments,
    })
  }
}
