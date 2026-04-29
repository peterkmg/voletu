use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, IntoActiveModel, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{ownership_transfer, ownership_transfer_item},
  enums::{LedgerEntrySourceEvent, LedgerEntrySourceKind},
  services::{
    common::{ensure_doc_mod_allowed, set_if_some},
    document::DocumentService,
    ledger::LedgerDelta,
  },
};

async fn get_by_id(
  conn: &impl ConnectionTrait,
  doc_id: Uuid,
) -> Result<ownership_transfer::ModelEx, ApiError> {
  ownership_transfer::Entity::load()
    .filter_by_id(doc_id)
    .filter(ownership_transfer::Column::DeletedAt.is_null())
    .with(ownership_transfer_item::Entity)
    .one(conn)
    .await?
    .ok_or_else(|| {
      ApiError::NotFound(format!(
        "Ownership transfer document '{}' not found",
        doc_id
      ))
    })
}

fn apply_ownership_transfer_update(
  model: &mut ownership_transfer::ActiveModel,
  req: &dtos::UpdateOwnershipTransferRequest,
) {
  set_if_some(&mut model.date, req.date);
}

async fn ensure_ownership_transfer_update_allowed(
  _svc: &DocumentService,
  _txn: &impl ConnectionTrait,
  existing: &ownership_transfer::Model,
  _req: &dtos::UpdateOwnershipTransferRequest,
) -> Result<(), ApiError> {
  ensure_doc_mod_allowed(existing.status)
}

async fn ensure_ownership_transfer_soft_delete_allowed(
  _svc: &DocumentService,
  _txn: &impl ConnectionTrait,
  existing: &ownership_transfer::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  ensure_doc_mod_allowed(existing.status)
}

async fn before_ownership_transfer_execute(
  svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &ownership_transfer::Model,
  _actor_id: Uuid,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.id).await?;

  for item in doc.items {
    svc
      .ledger
      .append_delta_on(conn, LedgerDelta {
        storage_id: item.storage_id,
        product_id: item.product_id,
        contractor_id: item.from_contractor_id,
        quantity_delta: -item.amount,
        source_kind: LedgerEntrySourceKind::OwnershipTransfer,
        source_id: existing.id,
        source_event: LedgerEntrySourceEvent::Execution,
        reverses_entry_id: None,
      })
      .await?;
    svc
      .ledger
      .append_delta_on(conn, LedgerDelta {
        storage_id: item.storage_id,
        product_id: item.product_id,
        contractor_id: item.to_contractor_id,
        quantity_delta: item.amount,
        source_kind: LedgerEntrySourceKind::OwnershipTransfer,
        source_id: existing.id,
        source_event: LedgerEntrySourceEvent::Execution,
        reverses_entry_id: None,
      })
      .await?;
  }

  Ok(())
}

async fn before_ownership_transfer_revert(
  svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &ownership_transfer::Model,
  _actor_id: Uuid,
) -> Result<(), ApiError> {
  svc
    .ledger
    .append_reversal_deltas_on(conn, LedgerEntrySourceKind::OwnershipTransfer, existing.id)
    .await?;
  Ok(())
}

#[voletu_core_macros::entity_service(
  entity_name = "Ownership transfer",
  entity = ownership_transfer,
  entity_mod = ownership_transfer,
  create_req = dtos::CreateOwnershipTransferRequest,
  update_req = dtos::UpdateOwnershipTransferRequest,
  response = dtos::OwnershipTransferResponse,
  apply_update = apply_ownership_transfer_update,
  before_update = ensure_ownership_transfer_update_allowed,
  before_soft_delete = ensure_ownership_transfer_soft_delete_allowed,
  before_execute = before_ownership_transfer_execute,
  before_revert = before_ownership_transfer_revert,
  ops(create, list, get, update, soft_delete, hard_delete, create_and_execute, execute, revert),
)]
impl DocumentService {}
