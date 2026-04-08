//! Verifies that a basic sync cycle between a Central node and one Peripheral
//! node reconstructs catalog data to parity on both sides using the real sync worker.
//!
//! Topology: Central + 1 Peripheral (assigned to base_alpha).
//!
//! Property: after the Peripheral's worker syncs, a company created on the
//! Peripheral appears on Central, and catalog entities seeded on Central
//! appear on the Peripheral.

use std::time::Duration;

use serde_json::json;

use crate::common::integration::{
  api_post, await_sync_cycle, has_catalog_entity, seed_catalog_via_api, setup_central_via_api,
  setup_peripheral_via_api, temp_db_path,
};

#[tokio::test]
async fn sync_worker_central_and_one_peripheral_reconstructs_to_parity() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("s2-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("s2-pa"), &central, &[
    catalog.base_alpha,
  ])
  .await;

  // Create a company on the Peripheral
  let new_company = api_post(
    &client,
    &format!("{}/catalog/companies", pa.url),
    &pa.token,
    json!({
      "commonName": "PA Local Co",
      "legalName": null,
      "isContractor": true,
      "isExporter": false,
      "isManufacturer": false,
      "isSender": false,
    }),
  )
  .await;
  let company_id =
    uuid::Uuid::parse_str(new_company["id"].as_str().expect("company should have id")).unwrap();

  // Wait for the worker to push the company to Central and complete a cycle
  await_sync_cycle(&client, &pa.url, &pa.token, Duration::from_secs(15)).await;

  // Verify: Central has the company created on Peripheral
  assert!(
    has_catalog_entity(
      &client,
      &central.url,
      &central.token,
      "/catalog/companies",
      company_id,
    )
    .await,
    "Central should have the company created on Peripheral after sync"
  );

  // Verify: Peripheral has catalog entities seeded on Central
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/products",
      catalog.product,
    )
    .await,
    "Peripheral should have products seeded on Central"
  );
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/bases",
      catalog.base_alpha,
    )
    .await,
    "Peripheral should have bases seeded on Central"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
