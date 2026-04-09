//! **Cross-base physical transfer**: A physical transfer from storage_alpha to storage_beta
//! routes to both bases and is visible on both peripherals.
//!
//! **Topology:** Central + 2 Peripherals (one base each)
//! **Verifies:** Audit log targets both bases; both peripherals receive the transfer with field parity

use std::time::Duration;

use uuid::Uuid;

use crate::common::integration::{
  await_sync_cycle,
  create_physical_transfer_via_api,
  get_physical_transfer_composite_json,
  query_audit_logs,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn cross_base_physical_transfer_routes_to_both_peripherals() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r3-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("r3-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r3-pb"), &central, &[
    catalog.base_beta
  ])
  .await;

  let transfer = create_physical_transfer_via_api(
    &client,
    &central.url,
    &central.token,
    "PHYS-CROSS",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    catalog.storage_beta,
    "200.0",
  )
  .await;
  let transfer_id = Uuid::parse_str(transfer["id"].as_str().unwrap()).unwrap();

  // Verify routing has BOTH bases
  let logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("physical_storage_transfers"),
    Some(transfer_id),
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

  // Both peripherals get it
  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;
  await_sync_cycle(&client, &pb.url, &pb.token, SYNC_TIMEOUT).await;

  let pa_t = get_physical_transfer_composite_json(&client, &pa.url, &pa.token, transfer_id).await;
  let pb_t = get_physical_transfer_composite_json(&client, &pb.url, &pb.token, transfer_id).await;
  assert!(pa_t.is_some(), "PA should have cross-base transfer");
  assert!(pb_t.is_some(), "PB should have cross-base transfer");

  // Field parity
  let central_t =
    get_physical_transfer_composite_json(&client, &central.url, &central.token, transfer_id)
      .await
      .unwrap();
  assert_eq!(
    central_t["items"][0]["amount"],
    pa_t.unwrap()["items"][0]["amount"]
  );
  assert_eq!(
    central_t["items"][0]["amount"],
    pb_t.unwrap()["items"][0]["amount"]
  );

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
