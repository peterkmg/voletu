use crate::{dtos::LedgerEntryResponse, entities::inventory_ledger_entry};

pub fn map_ledger_entry(row: inventory_ledger_entry::Model) -> LedgerEntryResponse {
  LedgerEntryResponse {
    storage_id: row.storage_id,
    product_id: row.product_id,
    contractor_id: row.contractor_id,
    current_amount: row.current_amount,
  }
}
