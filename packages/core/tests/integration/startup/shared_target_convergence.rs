//! Verifies that a document whose target bases span both Peripherals converges
//! across all three nodes after the real sync worker runs.
//!
//! Topology: Central + 2 Peripherals (PA on base_alpha, PB on base_beta).
//!
//! Property: after Central creates a physical transfer from storage_alpha to
//! storage_beta (routes to both bases), both PA and PB receive the document
//! with field parity.

use std::time::Duration;

use super::parse_doc_id;
use crate::common::integration::{
  await_sync_cycle,
  create_physical_transfer_via_api,
  get_physical_transfer_composite_json,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

#[tokio::test]
async fn physical_transfer_reaches_both_peripherals_with_field_parity() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("s4-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("s4-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("s4-pb"), &central, &[
    catalog.base_beta
  ])
  .await;

  // Create a physical transfer from storage_alpha to storage_beta (cross-base, routes to both)
  let transfer = create_physical_transfer_via_api(
    &client,
    &central.url,
    &central.token,
    "PHYS-SHARED",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    catalog.storage_beta,
    "200.0",
  )
  .await;
  let transfer_id = parse_doc_id(&transfer);

  // Let both peripherals sync
  await_sync_cycle(&client, &pa.url, &pa.token, Duration::from_secs(15)).await;
  await_sync_cycle(&client, &pb.url, &pb.token, Duration::from_secs(15)).await;

  // Both peripherals should have the transfer
  let pa_t = get_physical_transfer_composite_json(&client, &pa.url, &pa.token, transfer_id).await;
  let pb_t = get_physical_transfer_composite_json(&client, &pb.url, &pb.token, transfer_id).await;
  assert!(
    pa_t.is_some(),
    "PA (base_alpha) should have cross-base transfer"
  );
  assert!(
    pb_t.is_some(),
    "PB (base_beta) should have cross-base transfer"
  );

  // Field parity: all three nodes agree on amount
  let central_t =
    get_physical_transfer_composite_json(&client, &central.url, &central.token, transfer_id)
      .await
      .unwrap();
  assert_eq!(
    central_t["items"][0]["amount"],
    pa_t.unwrap()["items"][0]["amount"],
    "PA amount should match Central"
  );
  assert_eq!(
    central_t["items"][0]["amount"],
    pb_t.unwrap()["items"][0]["amount"],
    "PB amount should match Central"
  );

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
