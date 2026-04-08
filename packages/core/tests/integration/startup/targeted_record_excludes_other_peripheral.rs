//! Verifies that a document targeted at one Peripheral's base assignment is
//! excluded from the sync destined for a different Peripheral, using the real
//! sync worker.
//!
//! Topology: Central + 2 Peripherals (PA on base_alpha, PB on base_beta).
//!
//! Property: after Central creates an acceptance scoped to storage_alpha
//! (which routes to base_alpha), PA receives the document but PB does not.

use std::time::Duration;

use crate::common::integration::{
  await_sync_cycle, create_acceptance_via_api, get_acceptance_composite_json,
  seed_catalog_via_api, setup_central_via_api, setup_peripheral_via_api, temp_db_path,
};

use super::parse_doc_id;

#[tokio::test]
async fn targeted_acceptance_excludes_other_peripheral_via_worker() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("s3-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("s3-pa"), &central, &[
    catalog.base_alpha,
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("s3-pb"), &central, &[
    catalog.base_beta,
  ])
  .await;

  // Create an acceptance on Central for storage_alpha (routes to base_alpha only)
  let acc = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-ALPHA-ONLY",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "100.0",
  )
  .await;
  let acc_id = parse_doc_id(&acc);

  // Let both peripherals sync
  await_sync_cycle(&client, &pa.url, &pa.token, Duration::from_secs(15)).await;
  await_sync_cycle(&client, &pb.url, &pb.token, Duration::from_secs(15)).await;

  // PA (base_alpha) should have the acceptance
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
      .await
      .is_some(),
    "PA (base_alpha) should have the alpha-scoped acceptance"
  );

  // PB (base_beta) should NOT have the acceptance
  assert!(
    get_acceptance_composite_json(&client, &pb.url, &pb.token, acc_id)
      .await
      .is_none(),
    "PB (base_beta) should NOT have the alpha-scoped acceptance"
  );

  // Central still has it
  assert!(
    get_acceptance_composite_json(&client, &central.url, &central.token, acc_id)
      .await
      .is_some(),
    "Central should have the acceptance"
  );

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
