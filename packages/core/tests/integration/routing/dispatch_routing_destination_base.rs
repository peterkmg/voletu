//! **Dispatch routing with destination base**: A dispatch from alpha with destination_base = beta
//! routes to both bases, making it visible on both peripherals.
//!
//! **Topology:** Central + 2 Peripherals (one base each)
//! **Verifies:** Audit log targets both source and destination bases; both peripherals receive the dispatch

use crate::common::integration::{
  api_post,
  create_dispatch_via_api,
  get_composite_json,
  pull_from_central_to_target,
  query_audit_logs,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

use super::parse_doc_id;

#[tokio::test]
async fn dispatch_routing_includes_destination_base() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r9-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r9-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r9-pb"), &central, &[
    catalog.base_beta
  ])
  .await;

  // Pre-fill inventory so dispatch validation passes
  api_post(&client, &format!("{}/acceptance/composite/save-and-execute", central.url), &central.token,
    serde_json::json!({
      "documentNumber": "ACC-PRE", "dateAccepted": "2026-01-14T10:00:00Z", "arrivalType": "TRUCK",
      "sourceEntity": null, "contractorId": catalog.contractor, "truckWaybillId": null, "railWaybillId": null, "transitDispatchId": null,
      "items": [{"productId": catalog.product, "storageId": catalog.storage_alpha, "acceptedAmount": "500.0"}]
    })).await;

  // Dispatch from alpha with destination = beta → routes to BOTH
  let dispatch = create_dispatch_via_api(
    &client,
    &central.url,
    &central.token,
    "DISP-DEST",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "50.0",
    Some(catalog.base_beta),
  )
  .await;
  let dispatch_id = parse_doc_id(&dispatch);

  let logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("dispatch_documents"),
    Some(dispatch_id),
  )
  .await;
  assert!(!logs.is_empty());
  for log in &logs {
    let target = log["targetBaseIds"].as_str().unwrap_or("");
    assert!(
      target.contains(&catalog.base_alpha.to_string()),
      "should contain alpha, got: {target}"
    );
    assert!(
      target.contains(&catalog.base_beta.to_string()),
      "should contain beta, got: {target}"
    );
  }

  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pb.url,
    &pb.token,
    &[catalog.base_beta],
  )
  .await;
  assert!(get_composite_json(
    &client,
    &pa.url,
    &pa.token,
    "/dispatch/composite/{id}",
    dispatch_id
  )
  .await
  .is_some());
  assert!(get_composite_json(
    &client,
    &pb.url,
    &pb.token,
    "/dispatch/composite/{id}",
    dispatch_id
  )
  .await
  .is_some());

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
