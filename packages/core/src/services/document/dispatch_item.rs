use sea_orm::{prelude::Decimal, ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{dispatch_document, dispatch_item},
  services::{
    common::{ensure_doc_mod_allowed, ensure_storage_accepts_product, set_if_some},
    document::DocumentService,
    ledger::load_balance_by_dimensions_on,
  },
};

async fn get_by_id(
  conn: &impl ConnectionTrait,
  doc_id: Uuid,
) -> Result<dispatch_document::ModelEx, ApiError> {
  dispatch_document::Entity::load()
    .filter_by_id(doc_id)
    .filter(dispatch_document::Column::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Dispatch document '{}' not found", doc_id)))
}

async fn ensure_dispatch_item_create_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  req: &dtos::CreateDispatchItemRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, req.dispatch_doc_id).await?;
  ensure_doc_mod_allowed(doc.status)?;
  ensure_storage_accepts_product(conn, req.item.storage_id, req.item.product_id).await?;

  let current_amount_row = load_balance_by_dimensions_on(
    conn,
    req.item.storage_id,
    req.item.product_id,
    doc.contractor_id,
  )
  .await?;
  let current_amount = match current_amount_row {
    Some(entry) => entry.current_amount,
    None => Decimal::ZERO,
  };
  if req.item.dispatched_amount > current_amount {
    return Err(ApiError::Conflict(
      "dispatchedAmount exceeds available inventory balance".to_string(),
    ));
  }

  Ok(())
}

fn apply_dispatch_item_update(
  model: &mut dispatch_item::ActiveModel,
  req: &dtos::UpdateDispatchItemRequest,
) {
  set_if_some(&mut model.product_id, req.product_id);
  set_if_some(&mut model.storage_id, req.storage_id);
  set_if_some(&mut model.dispatched_amount, req.dispatched_amount);
}

async fn ensure_dispatch_item_update_allowed(
  _svc: &DocumentService,
  txn: &impl sea_orm::ConnectionTrait,
  existing: &dispatch_item::Model,
  req: &dtos::UpdateDispatchItemRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(txn, existing.dispatch_doc_id).await?;
  ensure_doc_mod_allowed(doc.status)?;

  let storage_id = req.storage_id.unwrap_or(existing.storage_id);
  let product_id = req.product_id.unwrap_or(existing.product_id);
  let dispatched_amount = req.dispatched_amount.unwrap_or(existing.dispatched_amount);

  ensure_storage_accepts_product(txn, storage_id, product_id).await?;

  let current_amount_row =
    load_balance_by_dimensions_on(txn, storage_id, product_id, doc.contractor_id).await?;
  let current_amount = match current_amount_row {
    Some(entry) => entry.current_amount,
    None => Decimal::ZERO,
  };
  if dispatched_amount > current_amount {
    return Err(ApiError::Conflict(
      "dispatchedAmount exceeds available inventory balance".to_string(),
    ));
  }

  Ok(())
}

async fn ensure_dispatch_item_soft_delete_allowed(
  _svc: &DocumentService,
  conn: &impl sea_orm::ConnectionTrait,
  existing: &dispatch_item::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.dispatch_doc_id).await?;
  ensure_doc_mod_allowed(doc.status)
}

#[voletu_core_macros::entity_service(
  entity_name = "Dispatch item",
  entity = dispatch_item,
  entity_mod = dispatch_item,
  create_req = dtos::CreateDispatchItemRequest,
  update_req = dtos::UpdateDispatchItemRequest,
  response = dtos::DispatchItemResponse,
  before_create = ensure_dispatch_item_create_allowed,
  apply_update = apply_dispatch_item_update,
  before_update = ensure_dispatch_item_update_allowed,
  before_soft_delete = ensure_dispatch_item_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
