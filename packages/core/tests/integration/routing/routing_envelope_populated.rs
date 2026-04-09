//! **Routing envelope populated**: Verifies that audit log routing envelopes contain
//! the correct `target_base_ids` when a composite acceptance document is created.
//!
//! **Topology:** Central only (no peripherals)
//! **Verifies:** Audit logs for both the document and its items include the base derived from the storage

use serde_json::Value;

use super::parse_doc_id;
use crate::common::integration::{
  assert_audit_log_targets, create_acceptance_via_api, query_audit_logs, seed_catalog_via_api,
  setup_central_via_api, temp_db_path,
};

#[tokio::test]
async fn routing_envelope_populated_on_composite_document_create() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r1-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  // Create acceptance with item referencing storage_alpha → should route to base_alpha
  let acc = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-R1-001",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "100.0",
  )
  .await;
  let doc_id = parse_doc_id(&acc);

  // Verify document INSERT audit log has base_alpha in target_base_ids
  let logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("acceptance_documents"),
    Some(doc_id),
  )
  .await;
  assert!(
    !logs.is_empty(),
    "expected audit log for acceptance_documents/{doc_id}"
  );
  assert_audit_log_targets(&logs, "acceptance_documents", doc_id, catalog.base_alpha);

  // Verify item audit logs also have correct routing
  let item_logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("acceptance_items"),
    None,
  )
  .await;
  let item_logs_for_doc: Vec<&Value> = item_logs
    .iter()
    .filter(|l| {
      l["newValuesJson"]
        .as_str()
        .map(|nv| nv.contains(&doc_id.to_string()))
        .unwrap_or(false)
    })
    .collect();
  assert!(!item_logs_for_doc.is_empty());
  for il in &item_logs_for_doc {
    let target = il["targetBaseIds"].as_str().unwrap_or("");
    assert!(
      target.contains(&catalog.base_alpha.to_string()),
      "item should target base_alpha, got: {target}"
    );
  }

  central.shutdown().await;
}
