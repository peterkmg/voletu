use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{blending_component, blending_document},
  services::{
    common::{ensure_doc_mod_allowed, set_if_some},
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

fn ensure_comp_differs_from_target(src: Uuid, dst: Uuid) -> Result<(), ApiError> {
  if src == dst {
    return Err(ApiError::Validation(
      "sourceProductId must differ from target product for blending".to_string(),
    ));
  }
  Ok(())
}

async fn ensure_blending_component_create_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  req: &dtos::CreateBlendingComponentRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, req.blending_doc_id).await?;

  ensure_doc_mod_allowed(doc.status)?;
  ensure_comp_differs_from_target(req.component.source_product_id, doc.target_product_id)
}

fn apply_blending_component_update(
  model: &mut blending_component::ActiveModel,
  req: &dtos::UpdateBlendingComponentRequest,
) {
  set_if_some(&mut model.storage_id, req.storage_id);
  set_if_some(&mut model.source_product_id, req.source_product_id);
  set_if_some(&mut model.amount_used, req.amount_used);
}

async fn ensure_blending_component_update_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &blending_component::Model,
  req: &dtos::UpdateBlendingComponentRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.blending_doc_id).await?;

  ensure_doc_mod_allowed(doc.status)?;

  let source_product_id = req.source_product_id.unwrap_or(existing.source_product_id);

  ensure_comp_differs_from_target(source_product_id, doc.target_product_id)
}

async fn ensure_blending_component_soft_delete_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &blending_component::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.blending_doc_id).await?;
  ensure_doc_mod_allowed(doc.status)
}

#[voletu_core_macros::entity_service(
  entity_name = "Blending component",
  entity = blending_component,
  entity_mod = blending_component,
  create_req = dtos::CreateBlendingComponentRequest,
  update_req = dtos::UpdateBlendingComponentRequest,
  response = dtos::BlendingComponentResponse,
  before_create = ensure_blending_component_create_allowed,
  apply_update = apply_blending_component_update,
  before_update = ensure_blending_component_update_allowed,
  before_soft_delete = ensure_blending_component_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
