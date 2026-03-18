use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{blending_document, blending_result},
  services::{
    common::{ensure_doc_mod_allowed, ensure_storage_accepts_product, set_if_some},
    document::DocumentService,
  },
};

async fn get_by_id(
  conn: &impl ConnectionTrait,
  doc_id: Uuid,
) -> Result<blending_document::ModelEx, ApiError> {
  blending_document::Entity::load()
    .filter_by_id(doc_id)
    .filter(blending_document::Column::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Blending document '{}' not found", doc_id)))
}

async fn ensure_blending_result_create_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  req: &dtos::CreateBlendingResultRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, req.blending_doc_id).await?;
  ensure_doc_mod_allowed(doc.status)?;
  ensure_storage_accepts_product(conn, req.result.storage_id, doc.target_product_id).await
}

fn apply_blending_result_update(
  model: &mut blending_result::ActiveModel,
  req: &dtos::UpdateBlendingResultRequest,
) {
  set_if_some(&mut model.storage_id, req.storage_id);
  set_if_some(&mut model.produced_amount, req.produced_amount);
}

async fn ensure_blending_result_update_allowed(
  _svc: &DocumentService,
  conn: &impl sea_orm::ConnectionTrait,
  existing: &blending_result::Model,
  req: &dtos::UpdateBlendingResultRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.blending_doc_id).await?;
  ensure_doc_mod_allowed(doc.status)?;

  let storage_id = req.storage_id.unwrap_or(existing.storage_id);
  ensure_storage_accepts_product(conn, storage_id, doc.target_product_id).await
}

async fn ensure_blending_result_soft_delete_allowed(
  _svc: &DocumentService,
  conn: &impl sea_orm::ConnectionTrait,
  existing: &blending_result::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.blending_doc_id).await?;
  ensure_doc_mod_allowed(doc.status)
}

#[voletu_core_macros::entity_service(
  entity_name = "Blending result",
  entity = blending_result,
  entity_mod = blending_result,
  create_req = dtos::CreateBlendingResultRequest,
  update_req = dtos::UpdateBlendingResultRequest,
  response = dtos::BlendingResultResponse,
  before_create = ensure_blending_result_create_allowed,
  apply_update = apply_blending_result_update,
  before_update = ensure_blending_result_update_allowed,
  before_soft_delete = ensure_blending_result_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
