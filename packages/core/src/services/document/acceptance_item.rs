use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{acceptance_document, acceptance_item},
  services::{
    common::{ensure_doc_mod_allowed, ensure_storage_accepts_product, set_if_some},
    document::DocumentService,
  },
};

fn apply_acceptance_item_update(
  model: &mut acceptance_item::ActiveModel,
  req: &dtos::UpdateAcceptanceItemRequest,
) {
  set_if_some(&mut model.product_id, req.product_id);
  set_if_some(&mut model.contractor_id, req.contractor_id);
  set_if_some(&mut model.storage_id, req.storage_id);
  set_if_some(&mut model.accepted_amount, req.accepted_amount);
}

async fn get_by_id(
  conn: &impl ConnectionTrait,
  doc_id: Uuid,
) -> Result<acceptance_document::ModelEx, ApiError> {
  let doc = acceptance_document::Entity::load()
    .filter_by_id(doc_id)
    .filter(acceptance_document::Column::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Acceptance document '{}' not found", doc_id)))?;

  Ok(doc)
}

async fn ensure_acceptance_item_create_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  req: &dtos::CreateAcceptanceItemRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, req.acceptance_doc_id).await?;

  ensure_doc_mod_allowed(doc.status)?;
  ensure_storage_accepts_product(conn, req.item.storage_id, req.item.product_id).await
}

async fn ensure_acceptance_item_update_allowed(
  _svc: &DocumentService,
  txn: &impl ConnectionTrait,
  existing: &acceptance_item::Model,
  req: &dtos::UpdateAcceptanceItemRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(txn, existing.acceptance_doc_id).await?;

  ensure_doc_mod_allowed(doc.status)?;

  let storage_id = req.storage_id.unwrap_or(existing.storage_id);
  let product_id = req.product_id.unwrap_or(existing.product_id);
  ensure_storage_accepts_product(txn, storage_id, product_id).await
}

async fn ensure_acceptance_item_soft_delete_allowed(
  _svc: &DocumentService,
  txn: &impl ConnectionTrait,
  existing: &acceptance_item::Model,
  undo: bool,
) -> Result<(), ApiError> {
  if undo {
    return Ok(());
  }

  let doc = get_by_id(txn, existing.acceptance_doc_id).await?;
  ensure_doc_mod_allowed(doc.status)
}

#[voletu_core_macros::entity_service(
  entity_name = "Acceptance item",
  entity = acceptance_item,
  entity_mod = acceptance_item,
  create_req = dtos::CreateAcceptanceItemRequest,
  update_req = dtos::UpdateAcceptanceItemRequest,
  response = dtos::AcceptanceItemResponse,
  before_create = ensure_acceptance_item_create_allowed,
  apply_update = apply_acceptance_item_update,
  before_update = ensure_acceptance_item_update_allowed,
  before_soft_delete = ensure_acceptance_item_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
