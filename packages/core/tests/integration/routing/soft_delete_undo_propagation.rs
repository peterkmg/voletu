//! **Soft delete undo propagates via sync**: A soft-deleted entity restored on Central
//! reappears in the peripheral's active catalog after sync.
//!
//! **Topology:** Central + 1 Peripheral (base_alpha)
//! **Verifies:** Entity is active after initial sync, absent after soft-delete sync, and active again after restore sync

use std::time::Duration;

use uuid::Uuid;

use crate::common::integration::{
  api_post,
  await_sync_cycle,
  has_catalog_entity,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  soft_delete_via_api,
  temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn restored_entity_reappears_in_active_catalog_on_peripheral() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r20-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r20-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Create company, sync, soft-delete, sync
  let company = api_post(&client, &format!("{}/catalog/companies", central.url), &central.token,
    serde_json::json!({"commonName": "Undo Co", "legalName": null, "isContractor": true, "isExporter": false, "isManufacturer": false, "isSender": false})).await;
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

  // Soft-delete, sync to PA
  soft_delete_via_api(
    &client,
    &central.url,
    &central.token,
    "/catalog/companies/{id}",
    company_id,
  )
  .await;
  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;
  assert!(
    !has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await,
    "should be soft-deleted"
  );

  // Undo soft-delete (restore) on Central
  api_post(
    &client,
    &format!("{}/catalog/companies/{company_id}/restore", central.url),
    &central.token,
    serde_json::json!({}),
  )
  .await;

  // Sync to PA — entity should be active again
  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;
  assert!(
    has_catalog_entity(
      &client,
      &pa.url,
      &pa.token,
      "/catalog/companies",
      company_id
    )
    .await,
    "should be restored after undo"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
