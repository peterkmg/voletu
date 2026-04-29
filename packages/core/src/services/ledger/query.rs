use std::collections::HashMap;

use uuid::Uuid;

use super::{add_row_to_balance, load_balance_by_dimensions_on, LedgerBalanceRow, LedgerService};
use crate::{api::ApiError, dtos::LedgerBalanceResponse, entities::inventory_ledger_entry};

impl LedgerService {
  pub async fn list_balances(&self) -> Result<Vec<LedgerBalanceResponse>, ApiError> {
    let rows: Vec<inventory_ledger_entry::ModelEx> = inventory_ledger_entry::Entity::load()
      .all(self.db.as_ref())
      .await?;

    let mut grouped: HashMap<(Uuid, Uuid, Uuid), LedgerBalanceRow> = HashMap::new();
    for row in rows {
      let key = (row.storage_id, row.product_id, row.contractor_id);
      match grouped.get_mut(&key) {
        Some(balance) => add_row_to_balance(balance, row),
        None => {
          grouped.insert(key, LedgerBalanceRow {
            current_amount: row.quantity_delta,
            latest_row: row,
          });
        }
      }
    }

    let balances: Vec<LedgerBalanceRow> = grouped.into_values().collect();

    Ok(
      balances
        .into_iter()
        .map(LedgerBalanceResponse::from)
        .collect(),
    )
  }

  pub async fn balance_by_dimensions(
    &self,
    storage_id: Uuid,
    product_id: Uuid,
    contractor_id: Uuid,
  ) -> Result<Option<LedgerBalanceResponse>, ApiError> {
    let row =
      load_balance_by_dimensions_on(self.db.as_ref(), storage_id, product_id, contractor_id)
        .await?;

    Ok(row.map(LedgerBalanceResponse::from))
  }
}
