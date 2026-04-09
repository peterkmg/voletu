use voletu_core::endpoints::paths as api_paths;

use crate::common::http::{
  assert_api_success,
  get,
  post_empty,
  setup_seeded_app_with_admin_token,
  with_auth_token,
};

const DEV_SEED_PATH: &str = "/dev/seed";

#[tokio::test]
async fn dev_seed_returns_populated_seed_result() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let response = post_empty(&app, DEV_SEED_PATH).await;
    let json = assert_api_success(response).await;
    let data = &json["data"];

    assert!(data["productTypes"].as_u64().unwrap_or_default() > 0);
    assert!(data["productGroups"].as_u64().unwrap_or_default() > 0);
    assert!(data["products"].as_u64().unwrap_or_default() > 0);
    assert!(data["companies"].as_u64().unwrap_or_default() > 0);
    assert!(data["ports"].as_u64().unwrap_or_default() > 0);
    assert!(data["bases"].as_u64().unwrap_or_default() > 0);
    assert!(data["warehouses"].as_u64().unwrap_or_default() > 0);
    assert!(data["storages"].as_u64().unwrap_or_default() > 0);
    assert!(data["users"].as_u64().unwrap_or_default() > 0);
    assert!(data["truckWaybills"].as_u64().unwrap_or_default() > 0);
    assert!(data["railWaybills"].as_u64().unwrap_or_default() > 0);
    assert!(data["acceptanceDocs"].as_u64().unwrap_or_default() > 0);
    assert!(data["dispatchDocs"].as_u64().unwrap_or_default() > 0);
    assert!(data["blendingDocs"].as_u64().unwrap_or_default() > 0);
    assert!(data["ownershipTransfers"].as_u64().unwrap_or_default() > 0);
    assert!(data["physicalTransfers"].as_u64().unwrap_or_default() > 0);
    assert!(data["reconciliations"].as_u64().unwrap_or_default() > 0);
    assert!(data["ledgerEntries"].as_u64().unwrap_or_default() > 0);
  })
  .await;
}

#[tokio::test]
async fn dev_seed_is_additive_across_repeated_calls() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let first_seed = post_empty(&app, DEV_SEED_PATH).await;
    let _ = assert_api_success(first_seed).await;

    let first_products =
      assert_api_success(get(&app, api_paths::catalog::PRODUCT_TYPES).await).await;
    let first_count = first_products["data"].as_array().unwrap().len();

    let second_seed = post_empty(&app, DEV_SEED_PATH).await;
    let second_seed_json = assert_api_success(second_seed).await;
    assert!(
      second_seed_json["data"]["productTypes"]
        .as_u64()
        .unwrap_or_default()
        > 0
    );

    let second_products =
      assert_api_success(get(&app, api_paths::catalog::PRODUCT_TYPES).await).await;
    let second_count = second_products["data"].as_array().unwrap().len();

    assert!(second_count > first_count);
  })
  .await;
}
