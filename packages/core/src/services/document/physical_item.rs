use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{physical_storage_transfer, physical_transfer_item},
  services::{
    common::{ensure_doc_mod_allowed, ensure_storage_accepts_product, set_if_some},
    document::DocumentService,
  },
};

async fn get_by_id(
  conn: &impl ConnectionTrait,
  doc_id: Uuid,
) -> Result<physical_storage_transfer::ModelEx, ApiError> {
  physical_storage_transfer::Entity::load()
    .filter_by_id(doc_id)
    .filter(physical_storage_transfer::Column::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Physical transfer '{}' not found", doc_id)))
}

async fn ensure_physical_item_create_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  req: &dtos::CreatePhysicalTransferItemRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, req.physical_transfer_id).await?;

  ensure_doc_mod_allowed(doc.status)?;
  ensure_storage_accepts_product(conn, req.item.to_storage_id, req.item.product_id).await
}

fn apply_physical_item_update(
  model: &mut physical_transfer_item::ActiveModel,
  req: &dtos::UpdatePhysicalTransferItemRequest,
) {
  set_if_some(&mut model.product_id, req.product_id);
  set_if_some(&mut model.from_storage_id, req.from_storage_id);
  set_if_some(&mut model.to_storage_id, req.to_storage_id);
  set_if_some(&mut model.amount, req.amount);
}

async fn ensure_physical_item_update_allowed(
  _svc: &DocumentService,
  conn: &impl sea_orm::ConnectionTrait,
  existing: &physical_transfer_item::Model,
  req: &dtos::UpdatePhysicalTransferItemRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.physical_transfer_id).await?;
  ensure_doc_mod_allowed(doc.status)?;

  let to_storage_id = req.to_storage_id.unwrap_or(existing.to_storage_id);
  let product_id = req.product_id.unwrap_or(existing.product_id);
  ensure_storage_accepts_product(conn, to_storage_id, product_id).await
}

async fn ensure_physical_item_soft_delete_allowed(
  _svc: &DocumentService,
  conn: &impl sea_orm::ConnectionTrait,
  existing: &physical_transfer_item::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.physical_transfer_id).await?;
  ensure_doc_mod_allowed(doc.status)
}

#[voletu_core_macros::entity_service(
	entity_name = "Physical transfer item",
	entity = physical_item,
	entity_mod = physical_transfer_item,
  create_req = dtos::CreatePhysicalTransferItemRequest,
	update_req = dtos::UpdatePhysicalTransferItemRequest,
	response = dtos::PhysicalTransferItemResponse,
  before_create = ensure_physical_item_create_allowed,
	apply_update = apply_physical_item_update,
	before_update = ensure_physical_item_update_allowed,
	before_soft_delete = ensure_physical_item_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
