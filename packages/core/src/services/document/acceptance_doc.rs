use sea_orm::{ConnectionTrait, EntityLoaderTrait, IntoActiveModel};
use uuid::Uuid;

use crate::{
  api::{
    ApiError,
    ApiError::{BadRequest, NotFound},
  },
  dtos,
  entities::{acceptance_document, acceptance_item},
  services::{
    common::{ensure_doc_mod_allowed, set_if_some, set_if_some_mapped},
    document::DocumentService,
  },
};

async fn before_acceptance_document_execute(
  svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &acceptance_document::Model,
  _actor_id: Uuid,
) -> Result<(), ApiError> {
  let doc = acceptance_document::Entity::load()
    .filter_by_id(existing.id)
    .with(acceptance_item::Entity)
    .one(conn)
    .await?
    .ok_or_else(|| NotFound(format!("Acceptance document '{}' not found", existing.id)))?;

  if doc.items.is_empty() {
    return Err(BadRequest(
      "Cannot execute acceptance document without items".to_string(),
    ));
  }

  for item in doc.items {
    svc
      .ledger
      .apply_delta_on(
        conn,
        item.storage_id,
        item.product_id,
        existing.contractor_id,
        item.accepted_amount,
      )
      .await?;
  }

  Ok(())
}

async fn before_acceptance_document_revert(
  svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &acceptance_document::Model,
  _actor_id: Uuid,
) -> Result<(), ApiError> {
  let doc = acceptance_document::Entity::load()
    .filter_by_id(existing.id)
    .with(acceptance_item::Entity)
    .one(conn)
    .await?
    .ok_or_else(|| NotFound(format!("Acceptance document '{}' not found", existing.id)))?;

  for item in doc.items {
    svc
      .ledger
      .apply_delta_on(
        conn,
        item.storage_id,
        item.product_id,
        existing.contractor_id,
        -item.accepted_amount,
      )
      .await?;
  }

  Ok(())
}

fn apply_acceptance_document_update(
  model: &mut acceptance_document::ActiveModel,
  req: &dtos::UpdateAcceptanceRequest,
) {
  set_if_some(&mut model.document_number, req.document_number.clone());
  set_if_some(&mut model.date_accepted, req.date_accepted);
  set_if_some(&mut model.arrival_type, req.arrival_type);
  set_if_some_mapped(&mut model.source_entity, req.source_entity.clone(), Some);
  set_if_some(&mut model.contractor_id, req.contractor_id);
  set_if_some_mapped(&mut model.truck_waybill_id, req.truck_waybill_id, Some);
  set_if_some_mapped(&mut model.rail_waybill_id, req.rail_waybill_id, Some);
  set_if_some_mapped(
    &mut model.transit_dispatch_id,
    req.transit_dispatch_id,
    Some,
  );
}

async fn ensure_acceptance_doc_update_allowed(
  _svc: &DocumentService,
  _txn: &impl ConnectionTrait,
  existing: &acceptance_document::Model,
  _req: &dtos::UpdateAcceptanceRequest,
) -> Result<(), ApiError> {
  ensure_doc_mod_allowed(existing.status)
}

async fn ensure_acceptance_doc_soft_delete_allowed(
  _svc: &DocumentService,
  _txn: &impl ConnectionTrait,
  existing: &acceptance_document::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  ensure_doc_mod_allowed(existing.status)
}

#[voletu_core_macros::entity_service(
  entity_name = "Acceptance document",
  entity = acceptance_document,
  entity_mod = acceptance_document,
  create_req = dtos::CreateAcceptanceRequest,
  update_req = dtos::UpdateAcceptanceRequest,
  response = dtos::AcceptanceResponse,
  apply_update = apply_acceptance_document_update,
  before_update = ensure_acceptance_doc_update_allowed,
  before_soft_delete = ensure_acceptance_doc_soft_delete_allowed,
  before_execute = before_acceptance_document_execute,
  before_revert = before_acceptance_document_revert,
  ops(create, list, get, update, soft_delete, hard_delete, create_and_execute, execute, revert),
)]
impl DocumentService {}
