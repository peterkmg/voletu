//! **Routing isolation**: Peripheral A (base_alpha) receives only alpha documents;
//! Peripheral B (base_beta) receives only beta documents.
//!
//! **Topology:** Central + 2 Peripherals (one base each)
//! **Verifies:** Sync routing isolates documents by base assignment, with field parity on the correct node

use std::time::Duration;

use super::parse_doc_id;
use crate::common::integration::{
  await_sync_cycle,
  create_acceptance_via_api,
  get_acceptance_composite_json,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn peripheral_receives_only_documents_matching_its_assignment() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r2-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("r2-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("r2-pb"), &central, &[
    catalog.base_beta
  ])
  .await;

  // Create two documents: one for alpha, one for beta
  let acc_alpha = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-ALPHA",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "50.0",
  )
  .await;
  let acc_alpha_id = parse_doc_id(&acc_alpha);

  let acc_beta = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-BETA",
    catalog.contractor,
    catalog.product,
    catalog.storage_beta,
    "75.0",
  )
  .await;
  let acc_beta_id = parse_doc_id(&acc_beta);

  // Wait for each peripheral to sync
  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;
  await_sync_cycle(&client, &pb.url, &pb.token, SYNC_TIMEOUT).await;

  // PA: has alpha, NOT beta
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_alpha_id)
      .await
      .is_some(),
    "PA should have alpha doc"
  );
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_beta_id)
      .await
      .is_none(),
    "PA should NOT have beta doc"
  );

  // PB: has beta, NOT alpha
  assert!(
    get_acceptance_composite_json(&client, &pb.url, &pb.token, acc_beta_id)
      .await
      .is_some(),
    "PB should have beta doc"
  );
  assert!(
    get_acceptance_composite_json(&client, &pb.url, &pb.token, acc_alpha_id)
      .await
      .is_none(),
    "PB should NOT have alpha doc"
  );

  // Field parity: alpha on Central == alpha on PA
  let central_alpha =
    get_acceptance_composite_json(&client, &central.url, &central.token, acc_alpha_id)
      .await
      .unwrap();
  let pa_alpha = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_alpha_id)
    .await
    .unwrap();
  assert_eq!(central_alpha["documentNumber"], pa_alpha["documentNumber"]);
  assert_eq!(
    central_alpha["items"][0]["acceptedAmount"],
    pa_alpha["items"][0]["acceptedAmount"]
  );

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
