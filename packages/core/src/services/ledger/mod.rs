mod create;
mod query;

use std::sync::Arc;

pub use create::LedgerDelta;
use sea_orm::{
  entity::prelude::Decimal,
  ColumnTrait,
  ConnectionTrait,
  DatabaseConnection,
  QueryFilter,
};
use uuid::Uuid;

use crate::{api::ApiError, entities::inventory_ledger_entry};

pub struct LedgerService {
  db: Arc<DatabaseConnection>,
}

pub(crate) struct LedgerBalanceRow {
  pub latest_row: inventory_ledger_entry::ModelEx,
  pub current_amount: Decimal,
}

impl LedgerService {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }
}

pub(crate) async fn load_balance_by_dimensions_on<C: ConnectionTrait>(
  conn: &C,
  storage_id: Uuid,
  product_id: Uuid,
  contractor_id: Uuid,
) -> Result<Option<LedgerBalanceRow>, ApiError> {
  let rows: Vec<inventory_ledger_entry::ModelEx> = inventory_ledger_entry::Entity::load()
    .filter(inventory_ledger_entry::Column::StorageId.eq(storage_id))
    .filter(inventory_ledger_entry::Column::ProductId.eq(product_id))
    .filter(inventory_ledger_entry::Column::ContractorId.eq(contractor_id))
    .all(conn)
    .await
    .map_err(ApiError::from)?;

  Ok(derive_balance(rows))
}

pub(crate) fn derive_balance(
  rows: impl IntoIterator<Item = inventory_ledger_entry::ModelEx>,
) -> Option<LedgerBalanceRow> {
  let mut rows = rows.into_iter();
  let first = rows.next()?;
  let mut balance = LedgerBalanceRow {
    current_amount: first.quantity_delta,
    latest_row: first,
  };

  for row in rows {
    add_row_to_balance(&mut balance, row);
  }

  Some(balance)
}

pub(crate) fn add_row_to_balance(
  balance: &mut LedgerBalanceRow,
  row: inventory_ledger_entry::ModelEx,
) {
  balance.current_amount += row.quantity_delta;
  if row.created_at > balance.latest_row.created_at {
    balance.latest_row = row;
  }
}
