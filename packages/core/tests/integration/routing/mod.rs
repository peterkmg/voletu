mod bidirectional_convergence;
mod bidirectional_sync;
mod blending_routing_cross_base;
mod catalog_broadcast;
mod catalog_only_sync_no_base;
mod central_seed_distributes;
mod central_url_change;
mod cross_base_physical_transfer;
mod data_parity_after_full_sync;
mod dispatch_routing_destination_base;
mod execute_produces_routed_update;
mod executed_document_ledger_parity;
mod full_document_lifecycle;
mod hard_delete_propagation;
mod incremental_pull_watermark;
mod multi_base_node_pull;
mod ownership_transfer_routing;
mod peripheral_seed_full_push;
mod reconciliation_routing;
mod routing_envelope_populated;
mod routing_isolation;
mod seeded_database_ledger_parity;
mod soft_delete_propagation;
mod soft_delete_undo_propagation;
mod three_peripheral_convergence;
mod worker_smoke;

use serde_json::Value;
use uuid::Uuid;

fn parse_doc_id(response: &Value) -> Uuid {
  let id_str = response["id"]
    .as_str()
    .or_else(|| response["document"]["id"].as_str())
    .expect("response should have id or document.id");
  Uuid::parse_str(id_str).unwrap()
}
