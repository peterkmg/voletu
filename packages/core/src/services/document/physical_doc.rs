use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, IntoActiveModel, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{physical_storage_transfer, physical_transfer_item},
  enums::{LedgerEntrySourceEvent, LedgerEntrySourceKind},
  services::{
    common::{ensure_doc_mod_allowed, ensure_storage_accepts_product, set_if_some},
    document::DocumentService,
    ledger::LedgerDelta,
  },
};

async fn get_by_id(
  conn: &impl ConnectionTrait,
  doc_id: Uuid,
) -> Result<physical_storage_transfer::ModelEx, ApiError> {
  physical_storage_transfer::Entity::load()
    .filter_by_id(doc_id)
    .filter(physical_storage_transfer::Column::DeletedAt.is_null())
    .with(physical_transfer_item::Entity)
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Physical transfer '{}' not found", doc_id)))
}

fn apply_physical_transfer_update(
  model: &mut physical_storage_transfer::ActiveModel,
  req: &dtos::UpdatePhysicalTransferRequest,
) {
  set_if_some(&mut model.document_number, req.document_number.clone());
  set_if_some(&mut model.date, req.date);
  set_if_some(&mut model.contractor_id, req.contractor_id);
  set_if_some(&mut model.start_cargo_ops, req.start_cargo_ops);
  set_if_some(&mut model.end_cargo_ops, req.end_cargo_ops);
}

async fn ensure_physical_transfer_update_allowed(
  _svc: &DocumentService,
  _txn: &impl ConnectionTrait,
  existing: &physical_storage_transfer::Model,
  _req: &dtos::UpdatePhysicalTransferRequest,
) -> Result<(), ApiError> {
  ensure_doc_mod_allowed(existing.status)
}

async fn ensure_physical_transfer_soft_delete_allowed(
  _svc: &DocumentService,
  _txn: &impl ConnectionTrait,
  existing: &physical_storage_transfer::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  ensure_doc_mod_allowed(existing.status)
}

async fn before_physical_transfer_execute(
  svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &physical_storage_transfer::Model,
  _actor_id: Uuid,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, existing.id).await?;

  for item in doc.items {
    ensure_storage_accepts_product(conn, item.from_storage_id, item.product_id).await?;
    ensure_storage_accepts_product(conn, item.to_storage_id, item.product_id).await?;

    svc
      .ledger
      .append_delta_on(conn, LedgerDelta {
        storage_id: item.from_storage_id,
        product_id: item.product_id,
        contractor_id: existing.contractor_id,
        quantity_delta: -item.amount,
        source_kind: LedgerEntrySourceKind::PhysicalStorageTransfer,
        source_id: existing.id,
        source_event: LedgerEntrySourceEvent::Execution,
        reverses_entry_id: None,
      })
      .await?;
    svc
      .ledger
      .append_delta_on(conn, LedgerDelta {
        storage_id: item.to_storage_id,
        product_id: item.product_id,
        contractor_id: existing.contractor_id,
        quantity_delta: item.amount,
        source_kind: LedgerEntrySourceKind::PhysicalStorageTransfer,
        source_id: existing.id,
        source_event: LedgerEntrySourceEvent::Execution,
        reverses_entry_id: None,
      })
      .await?;
  }

  Ok(())
}

async fn before_physical_transfer_revert(
  svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &physical_storage_transfer::Model,
  _actor_id: Uuid,
) -> Result<(), ApiError> {
  svc
    .ledger
    .append_reversal_deltas_on(
      conn,
      LedgerEntrySourceKind::PhysicalStorageTransfer,
      existing.id,
    )
    .await?;
  Ok(())
}

#[voletu_core_macros::entity_service(
  entity_name = "Physical transfer",
  entity = physical_transfer,
  entity_mod = physical_storage_transfer,
  create_req = dtos::CreatePhysicalTransferRequest,
  update_req = dtos::UpdatePhysicalTransferRequest,
  response = dtos::PhysicalTransferResponse,
  apply_update = apply_physical_transfer_update,
  before_update = ensure_physical_transfer_update_allowed,
  before_soft_delete = ensure_physical_transfer_soft_delete_allowed,
  before_execute = before_physical_transfer_execute,
  before_revert = before_physical_transfer_revert,
  ops(create, list, get, update, soft_delete, hard_delete, create_and_execute, execute, revert),
)]
impl DocumentService {}
