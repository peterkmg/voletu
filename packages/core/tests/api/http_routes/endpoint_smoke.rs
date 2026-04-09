use std::sync::Arc;

use axum::http::StatusCode;
use voletu_core::{api::router::build_router, db::seed_defaults, endpoints::paths as api_paths};

use crate::common::{
  http::{
    assert_api_error, assert_api_success, get, login_admin_token, post_json, response_json,
    with_auth_token,
  },
  payloads::{auth_login, catalog_base, node_initialize_replace},
  setup_db, test_api_state_with_default_restart_controls, test_config_for_db,
};

#[tokio::test]
async fn health_and_openapi_endpoints_are_available_and_return_expected_payload_shape() {
  let db = Arc::new(setup_db().await);
  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);

  let health = get(&app, api_paths::health::ROOT).await;
  let health_json = assert_api_success(health).await;
  assert_eq!(health_json["data"]["status"], "ok");

  let openapi = get(&app, api_paths::docs::OPENAPI_JSON).await;
  assert_eq!(openapi.status(), StatusCode::OK);
  let openapi_json = response_json(openapi).await;
  assert_eq!(openapi_json["openapi"], "3.1.0");
  assert_eq!(openapi_json["info"]["title"], "utoipa-axum");
}

#[tokio::test]
async fn reference_base_create_endpoint_returns_success_response_with_base_data() {
  let db = Arc::new(setup_db().await);
  let _ = seed_defaults(&db).await.unwrap();
  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);
  let token = login_admin_token(&app).await;

  with_auth_token(token, async {
    let payload = catalog_base("Base One", Some("Primary Base"));
    let res = post_json(&app, api_paths::catalog::BASES, payload).await;
    let json = assert_api_success(res).await;

    assert_eq!(json["data"]["commonName"], "Base One");
    assert_eq!(json["data"]["longName"], "Primary Base");
    assert!(json["data"]["id"].as_str().is_some());
  })
  .await;
}

#[tokio::test]
async fn list_endpoints_return_ok_for_empty_database_state_and_wrap_payload_in_api_response() {
  let db = Arc::new(setup_db().await);
  let _ = seed_defaults(&db).await.unwrap();
  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);
  let token = login_admin_token(&app).await;
  let uris = [
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
    api_paths::acceptance::ROOT,
    // acceptance::ITEMS removed — items managed via composite endpoints
    api_paths::dispatch::ROOT,
    // dispatch::ITEMS removed — items managed via composite endpoints
    api_paths::dispatch::STORAGE_MEASUREMENTS,
    api_paths::operations::PHYSICAL_TRANSFERS,
    api_paths::operations::OWNERSHIP_TRANSFERS,
    api_paths::blending::ROOT,
    // blending::COMPONENTS and ::RESULTS removed — managed via composite endpoints
    api_paths::operations::RECONCILIATIONS,
    api_paths::operations::RECONCILIATION_ADJUSTMENTS,
    api_paths::ledger::ROOT,
    api_paths::sync::AUDIT_LOGS,
    api_paths::sync::WATERMARKS,
    api_paths::users::ROOT,
  ];

  with_auth_token(token, async {
    for uri in uris {
      let res = get(&app, uri).await;
      let json = assert_api_success(res).await;
      assert!(json["data"].is_array(), "expected list array at {uri}");
    }
  })
  .await;
}

#[tokio::test]
async fn auth_initialize_endpoint_replaces_default_admin_and_blocks_old_credentials() {
  let db = Arc::new(setup_db().await);
  let _ = seed_defaults(&db).await.unwrap();
  let app = build_router(Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  )));
  let token = login_admin_token(&app).await;

  with_auth_token(token, async {
    let initialize_payload = node_initialize_replace("root", "root-password", "Root User");

    let init_res = post_json(&app, api_paths::node::INITIALIZE, initialize_payload).await;
    let init_json = assert_api_success(init_res).await;
    assert_eq!(init_json["data"]["message"], "Initialization completed");
  })
  .await;

  let old_admin_login = post_json(&app, api_paths::auth::LOGIN, auth_login("admin", "admin")).await;
  let _ = assert_api_error(
    old_admin_login,
    StatusCode::UNAUTHORIZED,
    "UNAUTHORIZED",
    Some("Invalid credentials"),
  )
  .await;

  let new_admin_login = post_json(
    &app,
    api_paths::auth::LOGIN,
    auth_login("root", "root-password"),
  )
  .await;
  let new_admin_json = assert_api_success(new_admin_login).await;
  assert_eq!(new_admin_json["data"]["user"]["username"], "root");
}
