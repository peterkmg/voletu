use serde_json::json;
use uuid::Uuid;

pub fn ledger_query(storage_id: Uuid, product_id: Uuid, contractor_id: Uuid) -> String {
  json!({
    "storageId": storage_id,
    "productId": product_id,
    "contractorId": contractor_id,
  })
  .to_string()
}
