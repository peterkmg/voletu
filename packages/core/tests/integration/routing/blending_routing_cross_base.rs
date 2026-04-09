//! **Blending routing across bases**: A blending document with components from base_alpha
//! and result in base_beta routes to both bases.
//!
//! **Topology:** Central + 2 Peripherals (one base each)
//! **Verifies:** Audit log targets both component and result bases; both peripherals receive the blending document

use std::time::Duration;

use uuid::Uuid;

use crate::common::integration::{
  await_sync_cycle, create_blending_via_api, get_composite_json, query_audit_logs,
  seed_catalog_via_api, setup_central_via_api, setup_peripheral_via_api, temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn blending_routing_spans_component_and_result_bases() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r10-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(
    &client,
    &temp_db_path("r10-pa"),
    &central,
    &[catalog.base_alpha],
  )
  .await;
  let pb = setup_peripheral_via_api(
    &client,
    &temp_db_path("r10-pb"),
    &central,
    &[catalog.base_beta],
  )
  .await;

  let blending = create_blending_via_api(
    &client,
    &central.url,
    &central.token,
    "BLEND-CROSS",
    catalog.contractor,
    catalog.product_b,
    catalog.storage_alpha,
    catalog.product,
    "40.0",
    catalog.storage_beta,
    "38.0",
  )
  .await;
  let blending_id = Uuid::parse_str(
    blending["document"]["id"]
      .as_str()
      .unwrap_or(blending["id"].as_str().unwrap_or("")),
  )
  .unwrap();

  let logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("blending_documents"),
    Some(blending_id),
  )
  .await;
  assert!(!logs.is_empty());
  for log in &logs {
    let target = log["targetBaseIds"].as_str().unwrap_or("");
    assert!(
      target.contains(&catalog.base_alpha.to_string()),
      "should contain alpha (component), got: {target}"
    );
    assert!(
      target.contains(&catalog.base_beta.to_string()),
      "should contain beta (result), got: {target}"
    );
  }

  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;
  await_sync_cycle(&client, &pb.url, &pb.token, SYNC_TIMEOUT).await;
  assert!(get_composite_json(
    &client,
    &pa.url,
    &pa.token,
    "/blending/composite/{id}",
    blending_id
  )
  .await
  .is_some());
  assert!(get_composite_json(
    &client,
    &pb.url,
    &pb.token,
    "/blending/composite/{id}",
    blending_id
  )
  .await
  .is_some());

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
