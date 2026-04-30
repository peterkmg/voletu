use std::time::Duration;

use serde_json::json;

use crate::common::integration::{
  api_post,
  await_sync_cycle,
  has_catalog_entity,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

#[tokio::test]
async fn company_pushed_to_central_and_catalog_pulled_to_peripheral() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("s2-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("s2-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

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

  await_sync_cycle(&client, &pa.url, &pa.token, Duration::from_secs(15)).await;

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
