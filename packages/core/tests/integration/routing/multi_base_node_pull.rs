use std::time::Duration;

use uuid::Uuid;

use super::parse_doc_id;
use crate::common::integration::{
  add_base_assignment_via_api,
  api_post,
  await_sync_cycle,
  create_acceptance_via_api,
  get_acceptance_composite_json,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn pulls_all_assigned_bases_and_excludes_unassigned() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r6-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let base_gamma = api_post(
    &client,
    &format!("{}/catalog/bases", central.url),
    &central.token,
    serde_json::json!({"commonName": "Base Gamma", "longName": null}),
  )
  .await;
  let base_gamma_id = Uuid::parse_str(base_gamma["id"].as_str().unwrap()).unwrap();
  let wh_gamma = api_post(
    &client,
    &format!("{}/catalog/warehouses", central.url),
    &central.token,
    serde_json::json!({"baseId": base_gamma_id, "commonName": "WH Gamma", "longName": null}),
  )
  .await;
  let wh_gamma_id = Uuid::parse_str(wh_gamma["id"].as_str().unwrap()).unwrap();
  let st_gamma = api_post(&client, &format!("{}/catalog/storages", central.url), &central.token,
    serde_json::json!({"warehouseId": wh_gamma_id, "commonName": "Tank Gamma", "longName": null, "capacity": null, "isTypeSpecific": false, "productTypeId": null})).await;
  let storage_gamma_id = Uuid::parse_str(st_gamma["id"].as_str().unwrap()).unwrap();

  let pa = setup_peripheral_via_api(&client, &temp_db_path("r6-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  add_base_assignment_via_api(&client, &pa.url, &pa.token, catalog.base_beta).await;

  let acc_a = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-A",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "10.0",
  )
  .await;
  let acc_b = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-B",
    catalog.contractor,
    catalog.product,
    catalog.storage_beta,
    "20.0",
  )
  .await;
  let acc_g = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-G",
    catalog.contractor,
    catalog.product,
    storage_gamma_id,
    "30.0",
  )
  .await;
  let acc_a_id = parse_doc_id(&acc_a);
  let acc_b_id = parse_doc_id(&acc_b);
  let acc_g_id = parse_doc_id(&acc_g);

  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;

  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_a_id)
      .await
      .is_some(),
    "PA should have alpha doc"
  );
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_b_id)
      .await
      .is_some(),
    "PA should have beta doc"
  );
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_g_id)
      .await
      .is_none(),
    "PA should NOT have gamma doc"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
