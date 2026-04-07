//! **Soft delete propagates via sync**: A soft-deleted catalog entity on Central
//! disappears from the peripheral's active list after sync.
//!
//! **Topology:** Central + 1 Peripheral (base_alpha)
//! **Verifies:** Soft-deleted entity is no longer in the active catalog list on the peripheral after pull

use uuid::Uuid;

use super::pull_all;
use crate::common::integration::{
  api_post,
  has_catalog_entity,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  soft_delete_via_api,
  temp_db_path,
};

#[tokio::test]
async fn soft_delete_propagates_via_sync() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r16-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r16-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Create and sync a company
  let company = api_post(&client, &format!("{}/catalog/companies", central.url), &central.token,
    serde_json::json!({"commonName": "SoftDel Co", "legalName": null, "isContractor": true, "isExporter": false, "isManufacturer": false, "isSender": false})).await;
  let company_id = Uuid::parse_str(company["id"].as_str().unwrap()).unwrap();

  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await,
    "PA should have company"
  );

  // Soft-delete on Central
  soft_delete_via_api(
    &client,
    &central.url,
    &central.token,
    "/catalog/companies/{id}",
    company_id,
  )
  .await;

  // Verify soft-deleted on Central (no longer in active list)
  assert!(
    !has_catalog_entity(
      &client,
      &central.url,
      &central.token,
      "/catalog/companies",
      company_id
    )
    .await,
    "Central: company should be soft-deleted"
  );

  // Sync to PA
  pull_all(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;

  // PA should also not have it in active list
  assert!(
    !has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await,
    "PA: company should be soft-deleted after sync"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
