//! **Catalog-only sync with no base assignment**: A peripheral with no base assignments
//! receives catalog entities but no business documents.
//!
//! **Topology:** Central + 1 Peripheral (no base assignments)
//! **Verifies:** Pull with empty base_ids syncs global catalog but excludes routed documents

use crate::common::integration::{
  create_acceptance_via_api,
  get_acceptance_composite_json,
  has_catalog_entity,
  pull_from_central_to_target,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

use super::parse_doc_id;

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

  // Pull with empty base_ids → catalog only
  let (pulled, _) = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[],
  )
  .await;
  assert!(pulled > 0, "should pull catalog entities");

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
