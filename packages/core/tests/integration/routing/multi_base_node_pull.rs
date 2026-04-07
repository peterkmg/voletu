//! **Multi-base node pull**: A peripheral assigned to alpha+beta receives documents for both,
//! but not for an unassigned gamma base.
//!
//! **Topology:** Central + 1 Peripheral (assigned to base_alpha and base_beta, not gamma)
//! **Verifies:** Multi-base peripheral pulls documents for all assigned bases and excludes unassigned ones

use uuid::Uuid;

use super::parse_doc_id;
use crate::common::integration::{
  add_base_assignment_via_api,
  api_post,
  create_acceptance_via_api,
  get_acceptance_composite_json,
  pull_from_central_to_target,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

#[tokio::test]
async fn multi_base_node_pulls_all_assigned_bases() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r6-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  // Create gamma base + warehouse + storage on Central
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

  // PA handles alpha + beta (not gamma) — set up with alpha, add beta via API
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r6-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  add_base_assignment_via_api(&client, &pa.url, &pa.token, catalog.base_beta).await;

  // Create documents for all three bases
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

  // Pull with both assigned bases
  let _ = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha, catalog.base_beta],
  )
  .await;

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
