//! **Ownership transfer routing via storage**: An ownership transfer on storage_alpha
//! routes only to base_alpha, not to base_beta.
//!
//! **Topology:** Central + 2 Peripherals (one base each)
//! **Verifies:** Ownership transfer audit log targets the storage's base; only the correct peripheral receives it

use std::time::Duration;

use uuid::Uuid;

use crate::common::integration::{
  assert_audit_log_targets,
  await_sync_cycle,
  create_ownership_transfer_via_api,
  get_composite_json,
  query_audit_logs,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn ownership_transfer_routing_via_storage() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r11-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r11-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r11-pb"), &central, &[
    catalog.base_beta
  ])
  .await;

  let transfer = create_ownership_transfer_via_api(
    &client,
    &central.url,
    &central.token,
    catalog.storage_alpha,
    catalog.product,
    catalog.contractor,
    catalog.contractor_b,
    "100.0",
  )
  .await;
  let transfer_id = Uuid::parse_str(transfer["id"].as_str().unwrap()).unwrap();

  let logs = query_audit_logs(
    &client,
    &central.url,
    &central.token,
    Some("ownership_transfers"),
    Some(transfer_id),
  )
  .await;
  assert!(!logs.is_empty());
  assert_audit_log_targets(
    &logs,
    "ownership_transfers",
    transfer_id,
    catalog.base_alpha,
  );

  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;
  await_sync_cycle(&client, &pb.url, &pb.token, SYNC_TIMEOUT).await;
  assert!(get_composite_json(
    &client,
    &pa.url,
    &pa.token,
    "/ownership-transfers/composite/{id}",
    transfer_id
  )
  .await
  .is_some());
  assert!(get_composite_json(
    &client,
    &pb.url,
    &pb.token,
    "/ownership-transfers/composite/{id}",
    transfer_id
  )
  .await
  .is_none());

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
