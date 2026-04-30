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
async fn active_catalog_excludes_entity_after_deletion_on_central() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r16-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r16-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  let company = api_post(&client, &format!("{}/catalog/companies", central.url), &central.token,
    serde_json::json!({"commonName": "SoftDel Co", "legalName": null, "isContractor": true, "isExporter": false, "isManufacturer": false, "isSender": false})).await;
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
    .await,
    "PA should have company"
  );

  soft_delete_via_api(
    &client,
    &central.url,
    &central.token,
    "/catalog/companies/{id}",
    company_id,
  )
  .await;

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
    "PA: company should be soft-deleted after sync"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
