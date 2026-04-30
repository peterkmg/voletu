mod basic_central_peripheral_parity;
mod ledger_affected_transfer_scoping;
mod node_initialization_triggers_restart;
mod shared_target_convergence;
mod targeted_record_excludes_other_peripheral;

use serde_json::Value;
use uuid::Uuid;

fn parse_doc_id(response: &Value) -> Uuid {
  let id_str = response["id"]
    .as_str()
    .or_else(|| response["document"]["id"].as_str())
    .expect("response should have id or document.id");
  Uuid::parse_str(id_str).unwrap()
}
