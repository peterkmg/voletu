use sea_orm::{entity::prelude::Decimal, ConnectionTrait, EntityLoaderTrait, IntoActiveModel};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{blending_component, blending_document, blending_result},
  enums::{LedgerEntrySourceEvent, LedgerEntrySourceKind},
  services::{
    common::{ensure_doc_mod_allowed, set_if_some},
    document::DocumentService,
    ledger::LedgerDelta,
  },
};

fn apply_blending_document_update(
  model: &mut blending_document::ActiveModel,
  req: &dtos::UpdateBlendingRequest,
) {
  set_if_some(&mut model.document_number, req.document_number.clone());
  set_if_some(&mut model.date, req.date);
  set_if_some(&mut model.contractor_id, req.contractor_id);
  set_if_some(&mut model.target_product_id, req.target_product_id);
}

async fn ensure_blending_document_update_allowed(
  _svc: &DocumentService,
  _txn: &impl sea_orm::ConnectionTrait,
  existing: &blending_document::Model,
  _req: &dtos::UpdateBlendingRequest,
) -> Result<(), ApiError> {
  ensure_doc_mod_allowed(existing.status)
}

async fn ensure_blending_document_soft_delete_allowed(
  _svc: &DocumentService,
  _txn: &impl sea_orm::ConnectionTrait,
  existing: &blending_document::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  ensure_doc_mod_allowed(existing.status)
}

async fn before_blending_document_execute(
  svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &blending_document::Model,
  _actor_id: Uuid,
) -> Result<(), ApiError> {
  let doc = blending_document::Entity::load()
    .filter_by_id(existing.id)
    .with(blending_component::Entity)
    .with(blending_result::Entity)
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Blending document '{}' not found", existing.id)))?;

  if doc.components.is_empty() {
    return Err(ApiError::Validation(
      "Cannot execute blending document without components".to_string(),
    ));
  }

  if doc.results.is_empty() {
    return Err(ApiError::Validation(
      "Cannot execute blending document without result rows".to_string(),
    ));
  }

  let comps_total = doc
    .components
    .iter()
    .fold(Decimal::ZERO, |acc, comp| acc + comp.amount_used);

  let res_total = doc
    .results
    .iter()
    .fold(Decimal::ZERO, |acc, res| acc + res.produced_amount);

  if comps_total != res_total {
    return Err(ApiError::Validation(
      "Blending document components and results do not match".to_string(),
    ));
  }

  for comp in doc.components {
    svc
      .ledger
      .append_delta_on(conn, LedgerDelta {
        storage_id: comp.storage_id,
        product_id: comp.source_product_id,
        contractor_id: existing.contractor_id,
        quantity_delta: -comp.amount_used,
        source_kind: LedgerEntrySourceKind::BlendingDocument,
        source_id: existing.id,
        source_event: LedgerEntrySourceEvent::Execution,
        reverses_entry_id: None,
      })
      .await?;
  }

  for res in doc.results {
    svc
      .ledger
      .append_delta_on(conn, LedgerDelta {
        storage_id: res.storage_id,
        product_id: existing.target_product_id,
        contractor_id: existing.contractor_id,
        quantity_delta: res.produced_amount,
        source_kind: LedgerEntrySourceKind::BlendingDocument,
        source_id: existing.id,
        source_event: LedgerEntrySourceEvent::Execution,
        reverses_entry_id: None,
      })
      .await?;
  }

  Ok(())
}

async fn before_blending_document_revert<C: sea_orm::ConnectionTrait>(
  svc: &DocumentService,
  conn: &C,
  existing: &blending_document::Model,
  _actor_id: Uuid,
) -> Result<(), ApiError> {
  svc
    .ledger
    .append_reversal_deltas_on(conn, LedgerEntrySourceKind::BlendingDocument, existing.id)
    .await?;
  Ok(())
}

#[voletu_core_macros::entity_service(
  entity_name = "Blending document",
  entity = blending_document,
  entity_mod = blending_document,
  create_req = dtos::CreateBlendingRequest,
  update_req = dtos::UpdateBlendingRequest,
  response = dtos::BlendingResponse,
  apply_update = apply_blending_document_update,
  before_update = ensure_blending_document_update_allowed,
  before_soft_delete = ensure_blending_document_soft_delete_allowed,
  before_execute = before_blending_document_execute,
  before_revert = before_blending_document_revert,
  ops(create, list, get, update, soft_delete, hard_delete, create_and_execute, execute, revert),
)]
impl DocumentService {}
