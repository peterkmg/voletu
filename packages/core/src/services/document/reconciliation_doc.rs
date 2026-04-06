use sea_orm::{
  entity::prelude::Decimal,
  ColumnTrait,
  ConnectionTrait,
  EntityLoaderTrait,
  EntityTrait,
  IntoActiveModel,
  QueryFilter,
};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{inventory_adjustment, inventory_reconciliation},
  enums,
  services::{
    common::{ensure_doc_mod_allowed, set_if_some},
    DocumentService,
  },
};

fn apply_reconciliation_update(
  model: &mut inventory_reconciliation::ActiveModel,
  req: &dtos::UpdateInventoryReconciliationRequest,
) {
  set_if_some(&mut model.document_number, req.document_number.clone());
  set_if_some(&mut model.date, req.date);
  set_if_some(&mut model.contractor_id, req.contractor_id);
  set_if_some(&mut model.warehouse_id, req.warehouse_id);
}

fn adjustment_delta(adjustment_type: &enums::AdjustmentType, amount: &Decimal) -> Decimal {
  match adjustment_type {
    enums::AdjustmentType::Surplus => *amount,
    enums::AdjustmentType::Loss => -*amount,
  }
}

async fn ensure_reconciliation_update_allowed(
  _svc: &DocumentService,
  _txn: &impl ConnectionTrait,
  existing: &inventory_reconciliation::Model,
  _req: &dtos::UpdateInventoryReconciliationRequest,
) -> Result<(), ApiError> {
  ensure_doc_mod_allowed(existing.status)
}

pub(super) async fn ensure_reconciliation_soft_delete_allowed(
  _svc: &DocumentService,
  _txn: &impl ConnectionTrait,
  existing: &inventory_reconciliation::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  ensure_doc_mod_allowed(existing.status)
}

async fn before_reconciliation_execute<C: ConnectionTrait>(
  svc: &DocumentService,
  conn: &C,
  existing: &inventory_reconciliation::Model,
  _actor_id: Uuid,
) -> Result<(), ApiError> {
  let doc = inventory_reconciliation::Entity::load()
    .filter_by_id(existing.id)
    .filter(inventory_reconciliation::Column::DeletedAt.is_null())
    .with(inventory_adjustment::Entity)
    .one(conn)
    .await?
    .ok_or_else(|| {
      ApiError::NotFound(format!(
        "Inventory reconciliation '{}' not found",
        existing.id
      ))
    })?;

  for adjustment in &doc.adjustments {
    svc
      .ledger
      .apply_delta_on(
        conn,
        adjustment.storage_id,
        adjustment.product_id,
        existing.contractor_id,
        adjustment_delta(&adjustment.adjustment_type, &adjustment.amount),
      )
      .await?;
  }

  Ok(())
}

async fn before_reconciliation_revert<C: ConnectionTrait>(
  svc: &DocumentService,
  conn: &C,
  existing: &inventory_reconciliation::Model,
  _actor_id: Uuid,
) -> Result<(), ApiError> {
  let doc = inventory_reconciliation::Entity::load()
    .filter_by_id(existing.id)
    .filter(inventory_reconciliation::Column::DeletedAt.is_null())
    .with(inventory_adjustment::Entity)
    .one(conn)
    .await?
    .ok_or_else(|| {
      ApiError::NotFound(format!(
        "Inventory reconciliation '{}' not found",
        existing.id
      ))
    })?;

  for adjustment in &doc.adjustments {
    svc
      .ledger
      .apply_delta_on(
        conn,
        adjustment.storage_id,
        adjustment.product_id,
        existing.contractor_id,
        -adjustment_delta(&adjustment.adjustment_type, &adjustment.amount),
      )
      .await?;
  }

  Ok(())
}

#[voletu_core_macros::entity_service(
  entity_name = "Reconciliation",
  entity = reconciliation,
  entity_mod = inventory_reconciliation,
  create_req = dtos::CreateInventoryReconciliationRequest,
  update_req = dtos::UpdateInventoryReconciliationRequest,
  response = dtos::InventoryReconciliationResponse,
  apply_update = apply_reconciliation_update,
  before_update = ensure_reconciliation_update_allowed,
  before_soft_delete = ensure_reconciliation_soft_delete_allowed,
  before_execute = before_reconciliation_execute,
  before_revert = before_reconciliation_revert,
  ops(create, list, get, update, soft_delete, hard_delete, create_and_execute, execute, revert),
)]
impl DocumentService {}
