use axum::http::StatusCode;
use uuid::Uuid;
use voletu_core::endpoints::paths as api_paths;

use crate::common::http::{
  assert_api_error,
  get,
  post_empty,
  setup_seeded_app_with_admin_token,
  with_auth_token,
};

#[tokio::test]
async fn endpoints_reject_zero_page_and_per_page_values() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let query_paths = [
      api_paths::acceptance::QUERY,
      api_paths::dispatch::QUERY,
      api_paths::blending::QUERY,
      api_paths::operations::PHYSICAL_TRANSFERS_QUERY,
      api_paths::operations::OWNERSHIP_TRANSFERS_QUERY,
      api_paths::operations::RECONCILIATIONS_QUERY,
    ];

    for query_path in query_paths {
      let zero_page = get(&app, format!("{}?page=0", query_path)).await;
      let _ = assert_api_error(
        zero_page,
        StatusCode::BAD_REQUEST,
        "VALIDATION_ERROR",
        Some("page and per_page"),
      )
      .await;

      let zero_per_page = get(&app, format!("{}?per_page=0", query_path)).await;
      let _ = assert_api_error(
        zero_per_page,
        StatusCode::BAD_REQUEST,
        "VALIDATION_ERROR",
        Some("page and per_page"),
      )
      .await;
    }
  })
  .await;
}

#[tokio::test]
async fn execute_endpoints_apply_route_specific_missing_semantics_and_error_structure() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;
  let unknown_id = Uuid::now_v7();

  with_auth_token(token, async {
    let acceptance_execute = post_empty(
      &app,
      api_paths::acceptance::EXECUTE_BY_ID.replace("{id}", &unknown_id.to_string()),
    )
    .await;
    let acceptance_json = assert_api_error(
      acceptance_execute,
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
      Some("Acceptance document"),
    )
    .await;
    assert_eq!(acceptance_json["error"]["code"], "NOT_FOUND");

    let dispatch_execute = post_empty(
      &app,
      api_paths::dispatch::EXECUTE_BY_ID.replace("{id}", &unknown_id.to_string()),
    )
    .await;
    let dispatch_json = assert_api_error(
      dispatch_execute,
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
      Some("Dispatch document"),
    )
    .await;
    assert_eq!(dispatch_json["error"]["code"], "NOT_FOUND");

    let blending_execute = post_empty(
      &app,
      api_paths::blending::EXECUTE_BY_ID.replace("{id}", &unknown_id.to_string()),
    )
    .await;
    let blending_json = assert_api_error(
      blending_execute,
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
      Some("Blending document"),
    )
    .await;
    assert_eq!(blending_json["error"]["code"], "NOT_FOUND");
  })
  .await;
}
