use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

use crate::entities::inventory_ledger_entry;

/// Response DTO for the `inventory_ledger_entry` entity.
#[response_dto(service_fields(common))]
pub struct LedgerEntryResponse {
  pub id: Uuid,
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub contractor_id: Uuid,
  pub current_amount: Decimal,
}

impl From<inventory_ledger_entry::Model> for LedgerEntryResponse {
  fn from(row: inventory_ledger_entry::Model) -> Self {
    Self {
      id: row.id,
      storage_id: row.storage_id,
      product_id: row.product_id,
      contractor_id: row.contractor_id,
      current_amount: row.current_amount,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}

impl From<inventory_ledger_entry::ModelEx> for LedgerEntryResponse {
  fn from(row: inventory_ledger_entry::ModelEx) -> Self {
    Self {
      id: row.id,
      storage_id: row.storage_id,
      product_id: row.product_id,
      contractor_id: row.contractor_id,
      current_amount: row.current_amount,
      created_at: row.created_at.to_rfc3339(),
      updated_at: row.updated_at.to_rfc3339(),
      deleted_at: row.deleted_at.map(|v| v.to_rfc3339()),
      created_by: row.created_by,
      updated_by: row.updated_by,
      deleted_by: row.deleted_by,
      origin_db_id: row.origin_db_id,
    }
  }
}
