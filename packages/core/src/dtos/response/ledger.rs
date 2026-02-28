use sea_orm::entity::prelude::Decimal;
use uuid::Uuid;
use voletu_core_macros::response_dto;

#[response_dto]
pub struct LedgerEntryResponse {
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub contractor_id: Uuid,
  pub current_amount: Decimal,
}
