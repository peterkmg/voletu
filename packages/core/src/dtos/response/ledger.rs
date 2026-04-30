use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::services::ledger::LedgerBalanceRow;

#[response_dto(service_fields(common))]
pub struct LedgerBalanceResponse {
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub contractor_id: Uuid,
  pub current_amount: Decimal,
}

impl From<LedgerBalanceRow> for LedgerBalanceResponse {
  fn from(row: LedgerBalanceRow) -> Self {
    let latest = row.latest_row;
    Self {
      storage_id: latest.storage_id,
      product_id: latest.product_id,
      contractor_id: latest.contractor_id,
      current_amount: row.current_amount,
      created_at: latest.created_at.to_rfc3339(),
      updated_at: latest.updated_at.to_rfc3339(),
      deleted_at: latest.deleted_at.map(|v| v.to_rfc3339()),
      created_by: latest.created_by,
      updated_by: latest.updated_by,
      deleted_by: latest.deleted_by,
      origin_db_id: latest.origin_db_id,
    }
  }
}
