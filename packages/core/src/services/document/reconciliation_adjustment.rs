use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{inventory_adjustment, inventory_reconciliation},
  services::{
    common::{ensure_doc_mod_allowed, set_if_some, set_if_some_mapped},
    DocumentService,
  },
};

async fn get_by_id(
  conn: &impl ConnectionTrait,
  adj_id: Uuid,
) -> Result<inventory_reconciliation::ModelEx, ApiError> {
  inventory_reconciliation::Entity::load()
    .filter_by_id(adj_id)
    .filter(inventory_reconciliation::Column::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Inventory reconciliation '{}' not found", adj_id)))
}

async fn ensure_adjustment_create_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  req: &dtos::CreateInventoryAdjustmentRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, req.reconciliation_id).await?;
  ensure_doc_mod_allowed(doc.status)
}

fn apply_adjustment_update(
  model: &mut inventory_adjustment::ActiveModel,
  req: &dtos::UpdateInventoryAdjustmentRequest,
) {
  set_if_some(&mut model.storage_id, req.storage_id);
  set_if_some(&mut model.product_id, req.product_id);
  set_if_some(&mut model.contractor_id, req.contractor_id);
  set_if_some(&mut model.adjustment_type, req.adjustment_type);
  set_if_some(&mut model.amount, req.amount);
  set_if_some_mapped(&mut model.reason, req.reason.clone(), Some);
}

async fn ensure_adjustment_update_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &inventory_adjustment::Model,
  _req: &dtos::UpdateInventoryAdjustmentRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.reconciliation_id).await?;
  ensure_doc_mod_allowed(doc.status)
}

async fn ensure_adjustment_soft_delete_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &inventory_adjustment::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.reconciliation_id).await?;
  ensure_doc_mod_allowed(doc.status)
}

#[voletu_core_macros::entity_service(
  entity_name = "Adjustment",
  entity = adjustment,
  entity_mod = inventory_adjustment,
  create_req = dtos::CreateInventoryAdjustmentRequest,
  update_req = dtos::UpdateInventoryAdjustmentRequest,
  response = dtos::InventoryAdjustmentResponse,
  before_create = ensure_adjustment_create_allowed,
  apply_update = apply_adjustment_update,
  before_update = ensure_adjustment_update_allowed,
  before_soft_delete = ensure_adjustment_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
