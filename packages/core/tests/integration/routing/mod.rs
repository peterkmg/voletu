mod bidirectional_sync;
mod blending_routing_cross_base;
mod catalog_broadcast;
mod catalog_only_sync_no_base;
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
mod reconciliation_routing;
mod routing_envelope_populated;
mod routing_isolation;
mod seeded_database_ledger_parity;
mod soft_delete_propagation;
mod soft_delete_undo_propagation;
mod three_peripheral_convergence;

use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

use crate::common::integration::pull_from_central_to_target_after;

/// Extract document ID from composite response (handles both flattened and nested shapes).
fn parse_doc_id(response: &Value) -> Uuid {
  let id_str = response["id"]
    .as_str()
    .or_else(|| response["document"]["id"].as_str())
    .expect("response should have id or document.id");
  Uuid::parse_str(id_str).unwrap()
}

/// Pull all remaining logs from Central to target (loops until exhausted).
async fn pull_all(
  client: &Client,
  central_url: &str,
  central_token: &str,
  target_url: &str,
  target_token: &str,
  base_ids: &[Uuid],
) {
  let mut cursor = Uuid::from_u128(1);
  loop {
    let (pulled, new_cursor) = pull_from_central_to_target_after(
      client,
      central_url,
      central_token,
      target_url,
      target_token,
      base_ids,
      cursor,
    )
    .await;
    if pulled == 0 || new_cursor == cursor {
      break;
    }
    cursor = new_cursor;
  }
}
