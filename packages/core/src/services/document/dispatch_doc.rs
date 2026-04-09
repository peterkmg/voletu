use sea_orm::{ConnectionTrait, EntityLoaderTrait, IntoActiveModel};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{dispatch_document, dispatch_item},
  services::{
    common::{ensure_doc_mod_allowed, set_if_some, set_if_some_mapped},
    document::DocumentService,
  },
};

async fn get_by_id(
  conn: &impl ConnectionTrait,
  doc_id: Uuid,
) -> Result<dispatch_document::ModelEx, ApiError> {
  dispatch_document::Entity::load()
    .filter_by_id(doc_id)
    .with(dispatch_item::Entity)
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Dispatch document '{}' not found", doc_id)))
}

fn apply_dispatch_document_update(
  ml: &mut dispatch_document::ActiveModel,
  req: &dtos::UpdateDispatchRequest,
) {
  set_if_some(&mut ml.document_number, req.document_number.clone());
  set_if_some(&mut ml.date, req.date);
  set_if_some(&mut ml.dispatch_purpose, req.dispatch_purpose);
  set_if_some(&mut ml.dispatch_method, req.dispatch_method);
  set_if_some(&mut ml.contractor_id, req.contractor_id);
  set_if_some_mapped(&mut ml.destination_base_id, req.destination_base_id, Some);
  set_if_some_mapped(&mut ml.receiver_entity, req.receiver_entity.clone(), Some);
  set_if_some_mapped(&mut ml.start_cargo_ops, req.start_cargo_ops, Some);
  set_if_some_mapped(&mut ml.end_cargo_ops, req.end_cargo_ops, Some);
  set_if_some_mapped(&mut ml.bunker_type, req.bunker_type, Some);
  set_if_some_mapped(&mut ml.exporter_id, req.exporter_id, Some);
  set_if_some_mapped(&mut ml.port_id, req.port_id, Some);
}

async fn ensure_dispatch_document_update_allowed(
  _svc: &DocumentService,
  _txn: &impl sea_orm::ConnectionTrait,
  existing: &dispatch_document::Model,
  _req: &dtos::UpdateDispatchRequest,
) -> Result<(), ApiError> {
  ensure_doc_mod_allowed(existing.status)
}

async fn ensure_dispatch_document_soft_delete_allowed(
  _svc: &DocumentService,
  _txn: &impl sea_orm::ConnectionTrait,
  existing: &dispatch_document::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  ensure_doc_mod_allowed(existing.status)
}

async fn before_dispatch_document_execute(
  svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &dispatch_document::Model,
  _actor_id: Uuid,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.id).await?;

  for item in doc.items {
    svc
      .ledger
      .apply_delta_on(
        conn,
        item.storage_id,
        item.product_id,
        existing.contractor_id,
        -item.dispatched_amount,
      )
      .await?;
  }

  Ok(())
}

async fn before_dispatch_document_revert<C: sea_orm::ConnectionTrait>(
  svc: &DocumentService,
  conn: &C,
  existing: &dispatch_document::Model,
  _actor_id: Uuid,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.id).await?;

  for item in doc.items {
    svc
      .ledger
      .apply_delta_on(
        conn,
        item.storage_id,
        item.product_id,
        existing.contractor_id,
        item.dispatched_amount,
      )
      .await?;
  }

  Ok(())
}

#[voletu_core_macros::entity_service(
  entity_name = "Dispatch document",
  entity = dispatch_document,
  entity_mod = crate::entities::dispatch_document,
  create_req = crate::dtos::CreateDispatchRequest,
  update_req = crate::dtos::UpdateDispatchRequest,
  response = crate::dtos::DispatchResponse,
  apply_update = apply_dispatch_document_update,
  before_update = ensure_dispatch_document_update_allowed,
  before_soft_delete = ensure_dispatch_document_soft_delete_allowed,
  before_execute = before_dispatch_document_execute,
  before_revert = before_dispatch_document_revert,
  ops(create, list, get, update, soft_delete, hard_delete, create_and_execute, execute, revert),
)]
impl DocumentService {}
