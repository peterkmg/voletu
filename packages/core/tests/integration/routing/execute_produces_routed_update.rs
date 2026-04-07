//! **Execute produces routed update audit log**: After executing a draft acceptance,
//! both the INSERT and UPDATE audit logs carry correct routing, and the peripheral
//! receives the document in EXECUTED status.
//!
//! **Topology:** Central + 1 Peripheral (base_alpha)
//! **Verifies:** Both INSERT and UPDATE audit logs target the correct base; peripheral sees EXECUTED status

use super::parse_doc_id;
use crate::common::integration::{
  create_acceptance_via_api,
  execute_document_via_api,
  get_acceptance_composite_json,
  pull_from_central_to_target,
  query_audit_logs,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

#[tokio::test]
async fn execute_produces_routed_update_audit_log() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r13-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r13-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Create draft, then execute
  let acc = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-EXEC",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "200.0",
  )
  .await;
  let acc_id = parse_doc_id(&acc);
  execute_document_via_api(
    &client,
    &central.url,
    &central.token,
    "/acceptance/execute/{id}",
    acc_id,
  )
  .await;

  // Both INSERT and UPDATE audit logs should have routing
  let all_logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("acceptance_documents"),
    Some(acc_id),
  )
  .await;
  assert!(
    all_logs.len() >= 2,
    "expected INSERT + UPDATE audit logs, got {}",
    all_logs.len()
  );
  let base_str = catalog.base_alpha.to_string();
  for log in &all_logs {
    let action = log["action"].as_str().unwrap_or("");
    let target = log["targetBaseIds"].as_str().unwrap_or("");
    assert!(
      target.contains(&base_str),
      "action={action} should target base_alpha, got: '{target}'"
    );
  }

  // Peripheral gets the executed document
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let pa_acc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id).await;
  assert!(pa_acc.is_some());
  assert_eq!(pa_acc.unwrap()["status"], "EXECUTED");

  central.shutdown().await;
  pa.shutdown().await;
}
