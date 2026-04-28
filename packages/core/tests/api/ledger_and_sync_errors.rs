use axum::http::StatusCode;
use sea_orm::prelude::Decimal;
use uuid::Uuid;
use voletu_core::endpoints::paths as api_paths;

use crate::common::{
  catalog_seed::{seed_inventory_catalog, seed_ledger_balance},
  http::{
    assert_api_error,
    assert_api_success,
    delete,
    get,
    post_json,
    setup_seeded_app_with_admin_token,
    with_auth_token,
  },
  payloads::{ledger_query, sync_push_invalid_action},
};

#[tokio::test]
async fn endpoints_lookup_entries_by_dimensions_and_return_matching_payloads() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let catalog = seed_inventory_catalog(&db).await;
  seed_ledger_balance(
    &db,
    catalog.storage_a_id,
    catalog.product_a_id,
    catalog.contractor_a_id,
    Decimal::from(11),
  )
  .await;
  with_auth_token(token, async {
    let lookup = post_json(
      &app,
      api_paths::ledger::QUERY,
      ledger_query(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      ),
    )
    .await;
    let lookup_json = assert_api_success(lookup).await;
    assert_eq!(
      lookup_json["data"]["storageId"],
      catalog.storage_a_id.to_string()
    );
    assert_eq!(lookup_json["data"]["currentAmount"], "11");

    let list = get(&app, api_paths::ledger::ROOT).await;
    let list_json = assert_api_success(list).await;
    assert_eq!(
      list_json["data"][0]["storageId"],
      catalog.storage_a_id.to_string()
    );
  })
  .await;
}

#[tokio::test]
async fn push_endpoint_rejects_invalid_action_value_as_bad_request() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let bad_action = post_json(
      &app,
      api_paths::sync::PUSH,
      sync_push_invalid_action(
        Uuid::now_v7(),
        Uuid::now_v7(),
        Uuid::now_v7(),
        Uuid::now_v7(),
      ),
    )
    .await;

    let bad_action_json = assert_api_error(
      bad_action,
      StatusCode::UNPROCESSABLE_ENTITY,
      "VALIDATION_ERROR",
      Some("action"),
    )
    .await;
    assert_eq!(bad_action_json["error"]["code"], "VALIDATION_ERROR");
  })
  .await;
}

#[tokio::test]
async fn user_delete_endpoint_returns_structured_not_found_for_unknown_uuid() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let missing_user = delete(
      &app,
      api_paths::users::BY_ID.replace("{id}", &Uuid::now_v7().to_string()),
    )
    .await;
    let missing_user_json = assert_api_error(
      missing_user,
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
      Some("not found"),
    )
    .await;
    assert_eq!(missing_user_json["error"]["code"], "NOT_FOUND");
  })
  .await;
}

#[tokio::test]
async fn user_delete_endpoint_returns_structured_validation_error_for_malformed_uuid() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let malformed_user = delete(&app, api_paths::users::BY_ID.replace("{id}", "not-a-uuid")).await;
    let malformed_user_json = assert_api_error(
      malformed_user,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      Some("UUID"),
    )
    .await;
    assert_eq!(malformed_user_json["error"]["code"], "VALIDATION_ERROR");
  })
  .await;
}
