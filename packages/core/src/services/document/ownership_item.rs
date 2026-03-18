use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{ownership_transfer, ownership_transfer_item},
  services::{
    common::{ensure_doc_mod_allowed, ensure_storage_accepts_product, set_if_some},
    document::DocumentService,
  },
};

async fn get_by_id(
  conn: &impl ConnectionTrait,
  doc_id: Uuid,
) -> Result<ownership_transfer::ModelEx, ApiError> {
  ownership_transfer::Entity::load()
    .filter_by_id(doc_id)
    .filter(ownership_transfer::Column::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Ownership transfer '{}' not found", doc_id)))
}

async fn ensure_ownership_item_create_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  req: &dtos::CreateOwnershipTransferItemRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, req.ownership_transfer_id).await?;
  ensure_doc_mod_allowed(doc.status)?;
  ensure_storage_accepts_product(conn, req.item.storage_id, req.item.product_id).await
}

fn apply_ownership_item_update(
  model: &mut ownership_transfer_item::ActiveModel,
  req: &dtos::UpdateOwnershipTransferItemRequest,
) {
  set_if_some(&mut model.storage_id, req.storage_id);
  set_if_some(&mut model.product_id, req.product_id);
  set_if_some(&mut model.from_contractor_id, req.from_contractor_id);
  set_if_some(&mut model.to_contractor_id, req.to_contractor_id);
  set_if_some(&mut model.amount, req.amount);
}

async fn ensure_ownership_item_update_allowed(
  _svc: &DocumentService,
  conn: &impl sea_orm::ConnectionTrait,
  existing: &ownership_transfer_item::Model,
  req: &dtos::UpdateOwnershipTransferItemRequest,
) -> Result<(), ApiError> {
  let transfer = get_by_id(conn, existing.ownership_transfer_id).await?;
  ensure_doc_mod_allowed(transfer.status)?;

  let storage_id = req.storage_id.unwrap_or(existing.storage_id);
  let product_id = req.product_id.unwrap_or(existing.product_id);
  ensure_storage_accepts_product(conn, storage_id, product_id).await
}

async fn ensure_ownership_item_soft_delete_allowed(
  _svc: &DocumentService,
  conn: &impl sea_orm::ConnectionTrait,
  existing: &ownership_transfer_item::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.ownership_transfer_id).await?;
  ensure_doc_mod_allowed(doc.status)
}

#[voletu_core_macros::entity_service(
	entity_name = "Ownership transfer item",
	entity = ownership_item,
	entity_mod = ownership_transfer_item,
  create_req = dtos::CreateOwnershipTransferItemRequest,
	update_req = dtos::UpdateOwnershipTransferItemRequest,
	response = dtos::OwnershipTransferItemResponse,
  before_create = ensure_ownership_item_create_allowed,
	apply_update = apply_ownership_item_update,
	before_update = ensure_ownership_item_update_allowed,
	before_soft_delete = ensure_ownership_item_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
