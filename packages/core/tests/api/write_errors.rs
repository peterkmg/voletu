use axum::http::StatusCode;
use uuid::Uuid;
use voletu_core::endpoints::paths as api_paths;

use crate::common::{
  fixtures::seed_inventory_fixture,
  http::{
    assert_api_error,
    assert_api_success,
    delete,
    post_empty,
    post_json,
    setup_seeded_app_with_admin_token,
    with_auth_token,
  },
  payloads::{blending_component, blending_result, dispatch_item, user_create},
};

#[tokio::test]
async fn write_routes_reject_empty_payload_with_structured_validation_error_envelope() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  let post_routes = [
    api_paths::auth::LOGIN,
    api_paths::auth::CHANGE_PASSWORD,
    api_paths::users::ROOT,
    api_paths::catalog::COMPANIES,
    api_paths::catalog::PRODUCT_TYPES,
    api_paths::catalog::PRODUCT_GROUPS,
    api_paths::catalog::PRODUCTS,
    api_paths::catalog::BASES,
    api_paths::catalog::WAREHOUSES,
    api_paths::catalog::STORAGES,
    api_paths::catalog::PORTS,
    api_paths::transport::truck::WAYBILLS,
    api_paths::transport::truck::ITEMS,
    api_paths::transport::truck::WEIGHT_DOCS,
    api_paths::transport::rail::WAYBILLS,
    api_paths::transport::rail::MANIFESTS,
    api_paths::transport::rail::MEASUREMENTS,
    api_paths::transport::rail::WEIGHTS,
    api_paths::acceptance::SAVE,
    api_paths::acceptance::ITEMS,
    api_paths::dispatch::SAVE,
    api_paths::dispatch::ITEMS,
    api_paths::dispatch::STORAGE_MEASUREMENTS,
    api_paths::operations::PHYSICAL_TRANSFERS_SAVE,
    api_paths::operations::OWNERSHIP_TRANSFERS_SAVE,
    api_paths::blending::SAVE,
    api_paths::blending::COMPONENTS,
    api_paths::blending::RESULTS,
    api_paths::operations::RECONCILIATIONS_SAVE,
    api_paths::operations::RECONCILIATION_ADJUSTMENTS_SAVE,
    api_paths::ledger::QUERY,
    api_paths::sync::WATERMARKS,
    api_paths::sync::PUSH,
  ];

  with_auth_token(token, async {
    for route in post_routes {
      let response = post_json(&app, route, "{}".to_string()).await;
      let status = response.status();
      assert!(
        status == StatusCode::BAD_REQUEST || status == StatusCode::UNPROCESSABLE_ENTITY,
        "expected 400/422 for {route}, got {status}"
      );
      let json = assert_api_error(response, status, "VALIDATION_ERROR", None).await;
      assert_eq!(json["error"]["code"], "VALIDATION_ERROR", "route: {route}");
    }
  })
  .await;
}

#[tokio::test]
async fn execute_and_delete_routes_reject_malformed_path_ids_with_structured_validation_error() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  let execute_routes = [
    api_paths::acceptance::EXECUTE_BY_ID.replace("{id}", "not-a-uuid"),
    api_paths::dispatch::EXECUTE_BY_ID.replace("{id}", "not-a-uuid"),
    api_paths::blending::EXECUTE_BY_ID.replace("{id}", "not-a-uuid"),
  ];

  with_auth_token(token, async {
    for route in execute_routes {
      let response = post_empty(&app, &route).await;
      let json = assert_api_error(
        response,
        StatusCode::BAD_REQUEST,
        "VALIDATION_ERROR",
        Some("UUID"),
      )
      .await;
      assert_eq!(json["error"]["code"], "VALIDATION_ERROR", "route: {route}");
    }

    let user_delete = delete(&app, api_paths::users::BY_ID.replace("{id}", "not-a-uuid")).await;
    let user_delete_json = assert_api_error(
      user_delete,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      Some("UUID"),
    )
    .await;
    assert_eq!(user_delete_json["error"]["code"], "VALIDATION_ERROR");
  })
  .await;
}

#[tokio::test]
async fn write_routes_surface_expected_404_and_409_domain_errors_in_matrix() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let fixture = seed_inventory_fixture(&db).await;

  // 404 matrix
  let not_found_cases = [
    (
      api_paths::dispatch::ITEMS.to_string(),
      dispatch_item(
        Uuid::now_v7(),
        fixture.product_a_id,
        fixture.storage_a_id,
        "1.0",
      ),
      "Dispatch document",
    ),
    (
      api_paths::blending::COMPONENTS.to_string(),
      blending_component(
        Uuid::now_v7(),
        fixture.storage_a_id,
        fixture.product_a_id,
        "1.0",
      ),
      "Blending document",
    ),
    (
      api_paths::blending::RESULTS.to_string(),
      blending_result(Uuid::now_v7(), fixture.storage_b_id, "1.0"),
      "Blending document",
    ),
  ];

  with_auth_token(token, async {
    for (route, payload, msg) in not_found_cases {
      let response = post_json(&app, route, payload).await;
      let json = assert_api_error(response, StatusCode::NOT_FOUND, "NOT_FOUND", Some(msg)).await;
      assert_eq!(json["error"]["code"], "NOT_FOUND");
    }

    // execute endpoints with unknown IDs -> 404 for these two routes
    let dispatch_execute_missing = post_empty(
      &app,
      api_paths::dispatch::EXECUTE_BY_ID.replace("{id}", &Uuid::now_v7().to_string()),
    )
    .await;
    let dispatch_execute_json = assert_api_error(
      dispatch_execute_missing,
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
      Some("Dispatch document"),
    )
    .await;
    assert_eq!(dispatch_execute_json["error"]["code"], "NOT_FOUND");

    let blending_execute_missing = post_empty(
      &app,
      api_paths::blending::EXECUTE_BY_ID.replace("{id}", &Uuid::now_v7().to_string()),
    )
    .await;
    let blending_execute_json = assert_api_error(
      blending_execute_missing,
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
      Some("Blending document"),
    )
    .await;
    assert_eq!(blending_execute_json["error"]["code"], "NOT_FOUND");

    // 409 matrix
    let create_user_payload = user_create(
      "matrix-operator",
      "operator-pass",
      "Matrix Operator",
      "operator",
    );
    let create_user = post_json(&app, api_paths::users::ROOT, create_user_payload.clone()).await;
    let _create_json = assert_api_success(create_user).await;
    let duplicate_user = post_json(&app, api_paths::users::ROOT, create_user_payload).await;
    let duplicate_user_json = assert_api_error(
      duplicate_user,
      StatusCode::CONFLICT,
      "CONFLICT",
      Some("already taken"),
    )
    .await;
    assert_eq!(duplicate_user_json["error"]["code"], "CONFLICT");
  })
  .await;
}
