//! **Catalog-only sync with no base assignment**: A peripheral with no base assignments
//! receives catalog entities but no business documents.
//!
//! **Topology:** Central + 1 Peripheral (no base assignments)
//! **Verifies:** Pull with empty base_ids syncs global catalog but excludes routed documents

use std::time::Duration;

use super::parse_doc_id;
use crate::common::integration::{
  await_sync_cycle, create_acceptance_via_api, get_acceptance_composite_json, has_catalog_entity,
  seed_catalog_via_api, setup_central_via_api, setup_peripheral_via_api, temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn catalog_only_sync_with_no_base_assignment() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r7-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  // Create a document on Central
  let acc = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-CATONLY",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "99.0",
  )
  .await;
  let acc_id = parse_doc_id(&acc);

  // Peripheral with NO base assignments
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r7-pa"), &central, &[]).await;

  // Worker already synced during setup; run one more cycle to pick up the acceptance
  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;

  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/products",
      catalog.product
    )
    .await,
    "should have product"
  );
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
      .await
      .is_none(),
    "should NOT have acceptance doc"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
