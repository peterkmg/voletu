use std::time::Duration;

use super::parse_doc_id;
use crate::common::integration::{
  api_get,
  api_post,
  create_acceptance_via_api,
  get_acceptance_composite_json,
  has_catalog_entity,
  poll_until,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
  wait_for_worker_online,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn pull_delivers_central_doc_and_push_relays_peripheral_doc() {
  let client = reqwest::Client::new();

  let central = setup_central_via_api(&client, &temp_db_path("smoke-central")).await;

  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("smoke-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  wait_for_worker_online(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;

  let acc = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-SMOKE-001",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "42.0",
  )
  .await;
  let acc_id = parse_doc_id(&acc);

  {
    let c = client.clone();
    let url = pa.url.clone();
    let tok = pa.token.clone();
    poll_until(
      || {
        let c = c.clone();
        let url = url.clone();
        let tok = tok.clone();
        async move {
          get_acceptance_composite_json(&c, &url, &tok, acc_id)
            .await
            .is_some()
        }
      },
      SYNC_TIMEOUT,
      "acceptance created on Central should sync to Peripheral",
    )
    .await;
  }

  let company = api_post(
    &client,
    &format!("{}/catalog/companies", pa.url),
    &pa.token,
    serde_json::json!({
      "commonName": "Smoke Co",
      "legalName": null,
      "isContractor": true,
      "isExporter": false,
      "isManufacturer": false,
      "isSender": false,
    }),
  )
  .await;
  let company_id =
    uuid::Uuid::parse_str(company["id"].as_str().expect("company should have id")).unwrap();

  poll_until(
    || {
      let c = client.clone();
      let url = central.url.clone();
      let tok = central.token.clone();
      async move { has_catalog_entity(&c, &url, &tok, "/catalog/companies", company_id).await }
    },
    Duration::from_secs(20),
    "company created on PA should propagate to Central",
  )
  .await;

  let status = api_get(&client, &format!("{}/node/status", pa.url), &pa.token).await;
  let last_sync = status["lastSyncAt"].as_str();
  assert!(
    last_sync.is_some(),
    "worker should report lastSyncAt; got status: {status}"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
