//! **Hard delete propagates via sync**: A hard-deleted catalog entity on Central
//! is completely removed from the peripheral after sync.
//!
//! **Topology:** Central + 1 Peripheral (base_alpha)
//! **Verifies:** Hard-deleted entity is absent from the peripheral's catalog after pull

use std::time::Duration;

use uuid::Uuid;

use crate::common::integration::{
  api_post, await_sync_cycle, hard_delete_via_api, has_catalog_entity, seed_catalog_via_api,
  setup_central_via_api, setup_peripheral_via_api, soft_delete_via_api, temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn hard_delete_propagates_via_sync() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r17-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(
    &client,
    &temp_db_path("r17-pa"),
    &central,
    &[catalog.base_alpha],
  )
  .await;

  // Create and sync a company
  let company = api_post(&client, &format!("{}/catalog/companies", central.url), &central.token,
    serde_json::json!({"commonName": "HardDel Co", "legalName": null, "isContractor": true, "isExporter": false, "isManufacturer": false, "isSender": false})).await;
  let company_id = Uuid::parse_str(company["id"].as_str().unwrap()).unwrap();

  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await
  );

  // Soft-delete first (required before hard-delete)
  soft_delete_via_api(
    &client,
    &central.url,
    &central.token,
    "/catalog/companies/{id}",
    company_id,
  )
  .await;
  // Hard-delete
  hard_delete_via_api(
    &client,
    &central.url,
    &central.token,
    "/catalog/companies/{id}/hard",
    company_id,
  )
  .await;

  // Sync to PA
  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;

  // PA should not have it at all
  assert!(
    !has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await,
    "PA: company should be gone after hard delete sync"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
