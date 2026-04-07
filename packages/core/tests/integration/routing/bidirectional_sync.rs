//! **Bidirectional sync**: A document created on Peripheral A is pushed to Central,
//! then pulled to Peripheral B, verifying the full relay path.
//!
//! **Topology:** Central + 2 Peripherals (both assigned to base_alpha)
//! **Verifies:** Push from peripheral to Central and pull to another peripheral preserves document with field parity

use uuid::Uuid;

use crate::common::integration::{
  create_acceptance_via_api,
  get_acceptance_composite_json,
  pull_from_central_to_target,
  push_outbound_to_central,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

use super::parse_doc_id;

const INITIAL_AUDIT_CURSOR: Uuid = Uuid::from_u128(1);

#[tokio::test]
async fn bidirectional_sync_peripheral_to_central_to_peripheral() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r5-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  // Both peripherals handle base_alpha
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r5-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r5-pb"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Sync catalog to PA so it can create documents
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;

  // PA creates an acceptance document
  let acc = create_acceptance_via_api(
    &client,
    &pa.url,
    &pa.token,
    "ACC-PA-001",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "333.0",
  )
  .await;
  let acc_id = parse_doc_id(&acc);

  // Push PA → Central
  let pushed = push_outbound_to_central(
    &client,
    &pa.url,
    &pa.token,
    &central.url,
    &central.token,
    INITIAL_AUDIT_CURSOR,
  )
  .await;
  assert!(
    pushed > 0,
    "should push at least one log from PA to Central"
  );

  // Central now has it
  let central_acc =
    get_acceptance_composite_json(&client, &central.url, &central.token, acc_id).await;
  assert!(central_acc.is_some(), "Central should have doc after push");

  // Pull Central → PB
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pb.url,
    &pb.token,
    &[catalog.base_alpha],
  )
  .await;

  // PB has the doc created on PA
  let pb_acc = get_acceptance_composite_json(&client, &pb.url, &pb.token, acc_id).await;
  assert!(
    pb_acc.is_some(),
    "PB should have doc from PA via Central relay"
  );

  // Field parity across all three nodes
  let pa_doc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
    .await
    .unwrap();
  let pb_doc = pb_acc.unwrap();
  assert_eq!(pa_doc["documentNumber"], pb_doc["documentNumber"]);
  assert_eq!(
    pa_doc["items"][0]["acceptedAmount"],
    pb_doc["items"][0]["acceptedAmount"]
  );

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
