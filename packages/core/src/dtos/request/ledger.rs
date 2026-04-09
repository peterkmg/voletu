use uuid::Uuid;
use voletu_core_macros::request_dto;

#[request_dto]
pub struct LedgerEntryLookupRequest {
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub contractor_id: Uuid,
}
