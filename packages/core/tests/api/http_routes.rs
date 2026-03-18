use std::{collections::HashSet, sync::Arc};

use axum::http::StatusCode;
use uuid::Uuid;
use voletu_core::{api::router::build_router, db::seed_defaults, endpoints::paths as api_paths};

const INITIAL_AUDIT_CURSOR: Uuid = Uuid::from_u128(1);

use crate::common::{
  fixtures::{seed_inventory_fixture, seed_sync_node},
  http::{
    assert_api_error,
    assert_api_success,
    get,
    login_admin_token,
    post_json,
    response_json,
    with_auth_token,
  },
  payloads::{
    auth_login,
    catalog_base,
    node_initialize_replace,
    sync_push_insert_company,
    sync_watermark_upsert,
  },
  setup_db,
  test_api_state_with_default_restart_controls,
  test_config_for_db,
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
    api_paths::acceptance::ITEMS,
    api_paths::dispatch::ROOT,
    api_paths::dispatch::ITEMS,
    api_paths::dispatch::STORAGE_MEASUREMENTS,
    api_paths::operations::PHYSICAL_TRANSFERS,
    api_paths::operations::OWNERSHIP_TRANSFERS,
    api_paths::blending::ROOT,
    api_paths::blending::COMPONENTS,
    api_paths::blending::RESULTS,
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
async fn sync_push_pull_and_watermark_endpoints_accept_valid_requests_and_return_expected_dto_fields(
) {
  let db = Arc::new(setup_db().await);
  let _ = seed_defaults(&db).await.unwrap();
  let fixture = seed_inventory_fixture(&db).await;
  let node_id = seed_sync_node(&db, fixture.base_id, "Peripheral A").await;

  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);
  let token = login_admin_token(&app).await;

  with_auth_token(token, async {
    let pushed_log_id = Uuid::now_v7();
    let push_payload = sync_push_insert_company(
      pushed_log_id,
      Uuid::now_v7(),
      fixture.base_id,
      Uuid::now_v7(),
      Uuid::now_v7(),
      "ACME",
    );

    let push_res = post_json(&app, api_paths::sync::PUSH, push_payload).await;
    let push_json = assert_api_success(push_res).await;
    assert_eq!(push_json["data"]["accepted"], 1);
    assert_eq!(push_json["data"]["rejected"], 0);

    let watermark_payload = sync_watermark_upsert(node_id, "PUSH", pushed_log_id);
    let watermark_res = post_json(&app, api_paths::sync::WATERMARKS, watermark_payload).await;
    let watermark_json = assert_api_success(watermark_res).await;
    assert_eq!(watermark_json["data"]["targetNodeId"], node_id.to_string());
    assert_eq!(
      watermark_json["data"]["lastAuditLogId"],
      pushed_log_id.to_string()
    );
    assert_eq!(watermark_json["data"]["direction"], "PUSH");

    let pull_uri = format!(
      "{}?nodeId={}&lastAuditLogId={}&limit=50",
      api_paths::sync::PULL,
      node_id,
      INITIAL_AUDIT_CURSOR
    );
    let pull_res = get(&app, pull_uri).await;
    let pull_json = assert_api_success(pull_res).await;
    assert!(pull_json["data"]["highestEvaluatedId"].is_string());
    assert!(pull_json["data"]["logs"].is_array());
  })
  .await;
}

#[tokio::test]
async fn sync_pull_with_malformed_query_uuid_returns_structured_validation_error() {
  let db = Arc::new(setup_db().await);
  let _ = seed_defaults(&db).await.unwrap();
  let app = build_router(Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  )));
  let token = login_admin_token(&app).await;

  with_auth_token(token, async {
    let malformed_query = get(
      &app,
      &format!(
        "{}?nodeId=bad-uuid&lastAuditLogId=bad-uuid&limit=50",
        api_paths::sync::PULL
      ),
    )
    .await;
    let malformed_query_json = assert_api_error(
      malformed_query,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      Some("UUID"),
    )
    .await;
    assert_eq!(malformed_query_json["error"]["code"], "VALIDATION_ERROR");
  })
  .await;
}

#[tokio::test]
async fn sync_outbound_with_malformed_query_uuid_returns_structured_validation_error() {
  let db = Arc::new(setup_db().await);
  let _ = seed_defaults(&db).await.unwrap();
  let app = build_router(Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  )));
  let token = login_admin_token(&app).await;

  with_auth_token(token, async {
    let malformed_query = get(
      &app,
      &format!(
        "{}?afterAuditLogId=bad-uuid&limit=50",
        api_paths::sync::OUTBOUND
      ),
    )
    .await;
    let malformed_query_json = assert_api_error(
      malformed_query,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      Some("UUID"),
    )
    .await;
    assert_eq!(malformed_query_json["error"]["code"], "VALIDATION_ERROR");
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

#[tokio::test]
async fn openapi_route_contract_enforces_standardized_paths_and_no_legacy_aliases() {
  let db = Arc::new(setup_db().await);
  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);

  let openapi = get(&app, api_paths::docs::OPENAPI_JSON).await;
  assert_eq!(openapi.status(), StatusCode::OK);
  let openapi_json = response_json(openapi).await;
  let paths = openapi_json["paths"]
    .as_object()
    .expect("openapi paths object");

  let required_paths = [
    api_paths::health::ROOT,
    api_paths::auth::LOGIN,
    api_paths::auth::REFRESH,
    api_paths::auth::CHANGE_PASSWORD,
    api_paths::node::INITIALIZE,
    api_paths::node::RESTART,
    api_paths::catalog::COMPANIES,
    api_paths::catalog::PRODUCT_TYPES,
    api_paths::catalog::PRODUCT_GROUPS,
    api_paths::catalog::PRODUCTS,
    api_paths::catalog::BASES,
    api_paths::catalog::WAREHOUSES,
    api_paths::catalog::STORAGES,
    api_paths::catalog::PORTS,
    api_paths::acceptance::ROOT,
    api_paths::acceptance::SAVE,
    api_paths::acceptance::SAVE_AND_EXECUTE,
    api_paths::acceptance::QUERY,
    api_paths::acceptance::COMPOSITE_BY_ID,
    api_paths::acceptance::COMPOSITE_SAVE,
    api_paths::acceptance::COMPOSITE_SAVE_AND_EXECUTE,
    api_paths::dispatch::ROOT,
    api_paths::dispatch::SAVE,
    api_paths::dispatch::SAVE_AND_EXECUTE,
    api_paths::dispatch::QUERY,
    api_paths::dispatch::COMPOSITE_BY_ID,
    api_paths::dispatch::COMPOSITE_SAVE,
    api_paths::dispatch::COMPOSITE_SAVE_AND_EXECUTE,
    api_paths::blending::ROOT,
    api_paths::blending::SAVE,
    api_paths::blending::SAVE_AND_EXECUTE,
    api_paths::blending::QUERY,
    api_paths::blending::COMPOSITE_BY_ID,
    api_paths::blending::COMPOSITE_SAVE,
    api_paths::blending::COMPOSITE_SAVE_AND_EXECUTE,
    api_paths::operations::PHYSICAL_TRANSFERS,
    api_paths::operations::PHYSICAL_TRANSFERS_QUERY,
    api_paths::operations::PHYSICAL_TRANSFERS_SAVE,
    api_paths::operations::PHYSICAL_TRANSFERS_SAVE_AND_EXECUTE,
    api_paths::operations::PHYSICAL_TRANSFERS_COMPOSITE_BY_ID,
    api_paths::operations::OWNERSHIP_TRANSFERS,
    api_paths::operations::OWNERSHIP_TRANSFERS_QUERY,
    api_paths::operations::OWNERSHIP_TRANSFERS_SAVE,
    api_paths::operations::OWNERSHIP_TRANSFERS_SAVE_AND_EXECUTE,
    api_paths::operations::OWNERSHIP_TRANSFERS_COMPOSITE_BY_ID,
    api_paths::operations::RECONCILIATIONS,
    api_paths::operations::RECONCILIATIONS_QUERY,
    api_paths::operations::RECONCILIATIONS_SAVE,
    api_paths::operations::RECONCILIATIONS_SAVE_AND_EXECUTE,
    api_paths::operations::RECONCILIATION_ADJUSTMENTS,
    api_paths::operations::RECONCILIATION_ADJUSTMENTS_SAVE,
    api_paths::ledger::ROOT,
    api_paths::ledger::QUERY,
    api_paths::sync::AUDIT_LOGS,
    api_paths::sync::OUTBOUND,
    api_paths::sync::PULL,
    api_paths::sync::PUSH,
    api_paths::sync::STATUS,
    api_paths::sync::WATERMARKS,
    api_paths::transport::truck::SAVE,
    api_paths::transport::rail::SAVE,
    api_paths::users::ROOT,
    api_paths::users::BY_ID,
  ];

  for path in required_paths {
    assert!(
      paths.contains_key(path),
      "missing standardized path in OpenAPI: {path}"
    );
  }

  let legacy_paths = [
    "/auth/initialize",
    "/system/restart",
    "/reference/companies",
    "/reference/bases",
    "/acceptance/storage-allocations",
    "/ledger/entries",
    "/ledger/entries/query",
    "/sync/audit-logs",
    "/transport/truck/intake-complete",
    "/transport/rail/intake-complete",
    "/transport/truck/composite",
    "/transport/rail/composite",
  ];

  for legacy in legacy_paths {
    assert!(
      !paths.contains_key(legacy),
      "legacy route must not be present in OpenAPI: {legacy}"
    );
  }

  assert!(
    paths
      .get(api_paths::ledger::QUERY)
      .and_then(|v| v.get("post"))
      .is_some(),
    "{}/query must remain POST",
    api_paths::ledger::ROOT
  );
  assert!(
    paths
      .get(api_paths::sync::PULL)
      .and_then(|v| v.get("get"))
      .is_some(),
    "{} must remain GET",
    api_paths::sync::PULL
  );
  assert!(
    paths
      .get(api_paths::sync::OUTBOUND)
      .and_then(|v| v.get("get"))
      .is_some(),
    "{} must remain GET",
    api_paths::sync::OUTBOUND
  );
}

#[tokio::test]
async fn openapi_operations_define_unique_operation_ids() {
  let db = Arc::new(setup_db().await);
  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);

  let openapi = get(&app, api_paths::docs::OPENAPI_JSON).await;
  assert_eq!(openapi.status(), StatusCode::OK);
  let openapi_json = response_json(openapi).await;
  let paths = openapi_json["paths"]
    .as_object()
    .expect("openapi paths object");

  let mut operation_ids = HashSet::new();
  let mut operations_total = 0usize;
  let methods = ["get", "post", "put", "delete", "patch", "head", "options", "trace"];

  for (path, item) in paths {
    let Some(item_obj) = item.as_object() else {
      continue;
    };

    for method in methods {
      let Some(operation) = item_obj.get(method) else {
        continue;
      };

      operations_total += 1;
      let op_id = operation
        .get("operationId")
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| panic!("missing operationId for {method} {path}"));
      assert!(
        operation_ids.insert(op_id.to_owned()),
        "duplicate operationId detected: {op_id}"
      );
    }
  }

  assert!(operations_total > 0, "OpenAPI must contain at least one operation");
}

#[tokio::test]
async fn openapi_route_method_contract_enforces_standardized_verbs() {
  let db = Arc::new(setup_db().await);
  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);

  let openapi = get(&app, api_paths::docs::OPENAPI_JSON).await;
  assert_eq!(openapi.status(), StatusCode::OK);
  let openapi_json = response_json(openapi).await;
  let paths = openapi_json["paths"]
    .as_object()
    .expect("openapi paths object");

  let required_method_pairs = [
    (api_paths::health::ROOT, "get"),
    (api_paths::auth::LOGIN, "post"),
    (api_paths::auth::REFRESH, "post"),
    (api_paths::auth::CHANGE_PASSWORD, "post"),
    (api_paths::node::INITIALIZE, "post"),
    (api_paths::node::RESTART, "post"),
    (api_paths::users::ROOT, "get"),
    (api_paths::users::ROOT, "post"),
    (api_paths::users::BY_ID, "delete"),
    (api_paths::ledger::ROOT, "get"),
    (api_paths::ledger::QUERY, "post"),
    (api_paths::sync::AUDIT_LOGS, "get"),
    (api_paths::sync::OUTBOUND, "get"),
    (api_paths::sync::PULL, "get"),
    (api_paths::sync::PUSH, "post"),
    (api_paths::sync::STATUS, "get"),
    (api_paths::sync::WATERMARKS, "get"),
    (api_paths::sync::WATERMARKS, "post"),
    (api_paths::transport::truck::SAVE, "post"),
    (api_paths::transport::rail::SAVE, "post"),
    (api_paths::acceptance::ROOT, "get"),
    (api_paths::acceptance::SAVE, "post"),
    (api_paths::acceptance::SAVE_AND_EXECUTE, "post"),
    (api_paths::acceptance::QUERY, "get"),
    (api_paths::acceptance::EXECUTE_BY_ID, "post"),
    (api_paths::acceptance::REVERT_BY_ID, "post"),
    (api_paths::dispatch::ROOT, "get"),
    (api_paths::dispatch::SAVE, "post"),
    (api_paths::dispatch::SAVE_AND_EXECUTE, "post"),
    (api_paths::dispatch::QUERY, "get"),
    (api_paths::dispatch::EXECUTE_BY_ID, "post"),
    (api_paths::dispatch::REVERT_BY_ID, "post"),
    (api_paths::blending::ROOT, "get"),
    (api_paths::blending::SAVE, "post"),
    (api_paths::blending::SAVE_AND_EXECUTE, "post"),
    (api_paths::blending::QUERY, "get"),
    (api_paths::blending::EXECUTE_BY_ID, "post"),
    (api_paths::blending::REVERT_BY_ID, "post"),
    (api_paths::operations::PHYSICAL_TRANSFERS, "get"),
    (api_paths::operations::PHYSICAL_TRANSFERS_QUERY, "get"),
    (api_paths::operations::PHYSICAL_TRANSFERS_SAVE, "post"),
    (api_paths::operations::PHYSICAL_TRANSFERS_SAVE_AND_EXECUTE, "post"),
    (api_paths::operations::PHYSICAL_TRANSFERS_EXECUTE_BY_ID, "post"),
    (api_paths::operations::PHYSICAL_TRANSFERS_REVERT_BY_ID, "post"),
    (api_paths::operations::OWNERSHIP_TRANSFERS, "get"),
    (api_paths::operations::OWNERSHIP_TRANSFERS_QUERY, "get"),
    (api_paths::operations::OWNERSHIP_TRANSFERS_SAVE, "post"),
    (api_paths::operations::OWNERSHIP_TRANSFERS_SAVE_AND_EXECUTE, "post"),
    (api_paths::operations::OWNERSHIP_TRANSFERS_EXECUTE_BY_ID, "post"),
    (api_paths::operations::OWNERSHIP_TRANSFERS_REVERT_BY_ID, "post"),
    (api_paths::operations::RECONCILIATIONS, "get"),
    (api_paths::operations::RECONCILIATIONS_QUERY, "get"),
    (api_paths::operations::RECONCILIATIONS_SAVE, "post"),
    (api_paths::operations::RECONCILIATIONS_SAVE_AND_EXECUTE, "post"),
    (api_paths::operations::RECONCILIATIONS_EXECUTE_BY_ID, "post"),
    (api_paths::operations::RECONCILIATIONS_REVERT_BY_ID, "post"),
    (api_paths::operations::RECONCILIATION_ADJUSTMENTS, "get"),
    (api_paths::operations::RECONCILIATION_ADJUSTMENTS_SAVE, "post"),
  ];

  for (path, method) in required_method_pairs {
    let methods = paths
      .get(path)
      .and_then(|v| v.as_object())
      .unwrap_or_else(|| panic!("missing path in OpenAPI: {path}"));
    assert!(
      methods.contains_key(method),
      "missing method contract in OpenAPI: {method} {path}"
    );
  }
}

#[tokio::test]
async fn openapi_route_method_contract_rejects_wrong_or_legacy_verbs() {
  let db = Arc::new(setup_db().await);
  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);

  let openapi = get(&app, api_paths::docs::OPENAPI_JSON).await;
  assert_eq!(openapi.status(), StatusCode::OK);
  let openapi_json = response_json(openapi).await;
  let paths = openapi_json["paths"]
    .as_object()
    .expect("openapi paths object");

  let forbidden_method_pairs = [
    (api_paths::ledger::QUERY, "get"),
    (api_paths::sync::PUSH, "get"),
    (api_paths::sync::PULL, "post"),
    (api_paths::sync::OUTBOUND, "post"),
    (api_paths::transport::truck::SAVE, "get"),
    (api_paths::transport::rail::SAVE, "get"),
    (api_paths::acceptance::QUERY, "post"),
    (api_paths::dispatch::QUERY, "post"),
    (api_paths::blending::QUERY, "post"),
    (api_paths::operations::PHYSICAL_TRANSFERS_QUERY, "post"),
    (api_paths::operations::OWNERSHIP_TRANSFERS_QUERY, "post"),
    (api_paths::operations::RECONCILIATIONS_QUERY, "post"),
    (api_paths::acceptance::EXECUTE_BY_ID, "get"),
    (api_paths::dispatch::EXECUTE_BY_ID, "get"),
    (api_paths::blending::EXECUTE_BY_ID, "get"),
    (api_paths::operations::PHYSICAL_TRANSFERS_EXECUTE_BY_ID, "get"),
    (api_paths::operations::OWNERSHIP_TRANSFERS_EXECUTE_BY_ID, "get"),
    (api_paths::operations::RECONCILIATIONS_EXECUTE_BY_ID, "get"),
    (api_paths::acceptance::REVERT_BY_ID, "get"),
    (api_paths::dispatch::REVERT_BY_ID, "get"),
    (api_paths::blending::REVERT_BY_ID, "get"),
    (api_paths::operations::PHYSICAL_TRANSFERS_REVERT_BY_ID, "get"),
    (api_paths::operations::OWNERSHIP_TRANSFERS_REVERT_BY_ID, "get"),
    (api_paths::operations::RECONCILIATIONS_REVERT_BY_ID, "get"),
  ];

  for (path, forbidden_method) in forbidden_method_pairs {
    let methods = paths
      .get(path)
      .and_then(|v| v.as_object())
      .unwrap_or_else(|| panic!("missing path in OpenAPI: {path}"));
    assert!(
      !methods.contains_key(forbidden_method),
      "unexpected method exposed in OpenAPI: {forbidden_method} {path}"
    );
  }
}

#[tokio::test]
async fn openapi_query_parameter_contract_enforces_pagination_and_filter_params() {
  let db = Arc::new(setup_db().await);
  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);

  let openapi = get(&app, api_paths::docs::OPENAPI_JSON).await;
  assert_eq!(openapi.status(), StatusCode::OK);
  let openapi_json = response_json(openapi).await;
  let paths = openapi_json["paths"]
    .as_object()
    .expect("openapi paths object");

  let param_names_for = |path: &str, method: &str| -> HashSet<String> {
    let params = paths
      .get(path)
      .and_then(|v| v.as_object())
      .and_then(|methods| methods.get(method))
      .and_then(|operation| operation.get("parameters"))
      .and_then(|v| v.as_array())
      .unwrap_or_else(|| panic!("missing OpenAPI parameters for {method} {path}"));

    params
      .iter()
      .filter_map(|param| {
        let name = param.get("name")?.as_str()?;
        let location = param.get("in")?.as_str()?;
        if location == "query" {
          Some(name.to_string())
        } else {
          None
        }
      })
      .collect()
  };

  let list_routes_with_pagination = [
    api_paths::acceptance::ROOT,
    api_paths::dispatch::ROOT,
    api_paths::blending::ROOT,
    api_paths::operations::PHYSICAL_TRANSFERS,
    api_paths::operations::OWNERSHIP_TRANSFERS,
    api_paths::operations::RECONCILIATIONS,
  ];

  for path in list_routes_with_pagination {
    let param_names = param_names_for(path, "get");
    assert!(
      param_names.contains("page"),
      "missing query parameter page for GET {path}"
    );
    assert!(
      param_names.contains("per_page"),
      "missing query parameter per_page for GET {path}"
    );
  }

  let query_route_contracts: [(&str, &[&str]); 6] = [
    (api_paths::acceptance::QUERY, &["documentNumber", "status", "page", "per_page"]),
    (
      api_paths::dispatch::QUERY,
      &["documentNumber", "status", "contractorId", "page", "per_page"],
    ),
    (
      api_paths::blending::QUERY,
      &["documentNumber", "status", "contractorId", "page", "per_page"],
    ),
    (
      api_paths::operations::PHYSICAL_TRANSFERS_QUERY,
      &["documentNumber", "status", "page", "per_page"],
    ),
    (
      api_paths::operations::OWNERSHIP_TRANSFERS_QUERY,
      &["status", "page", "per_page"],
    ),
    (
      api_paths::operations::RECONCILIATIONS_QUERY,
      &["documentNumber", "status", "warehouseId", "page", "per_page"],
    ),
  ];

  for (path, expected_query_params) in query_route_contracts {
    let param_names = param_names_for(path, "get");
    for expected in expected_query_params {
      assert!(
        param_names.contains(*expected),
        "missing query parameter {expected} for GET {path}"
      );
    }
  }
}

#[tokio::test]
async fn openapi_sync_query_parameter_contract_enforces_pull_and_outbound_params() {
  let db = Arc::new(setup_db().await);
  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);

  let openapi = get(&app, api_paths::docs::OPENAPI_JSON).await;
  assert_eq!(openapi.status(), StatusCode::OK);
  let openapi_json = response_json(openapi).await;
  let paths = openapi_json["paths"]
    .as_object()
    .expect("openapi paths object");

  let param_names_for = |path: &str, method: &str| -> HashSet<String> {
    let params = paths
      .get(path)
      .and_then(|v| v.as_object())
      .and_then(|methods| methods.get(method))
      .and_then(|operation| operation.get("parameters"))
      .and_then(|v| v.as_array())
      .unwrap_or_else(|| panic!("missing OpenAPI parameters for {method} {path}"));

    params
      .iter()
      .filter_map(|param| {
        let name = param.get("name")?.as_str()?;
        let location = param.get("in")?.as_str()?;
        if location == "query" {
          Some(name.to_string())
        } else {
          None
        }
      })
      .collect()
  };

  let outbound_params = param_names_for(api_paths::sync::OUTBOUND, "get");
  for expected in ["afterAuditLogId", "limit"] {
    assert!(
      outbound_params.contains(expected),
      "missing query parameter {expected} for GET {}",
      api_paths::sync::OUTBOUND
    );
  }

  let pull_params = param_names_for(api_paths::sync::PULL, "get");
  for expected in ["nodeId", "lastAuditLogId", "limit"] {
    assert!(
      pull_params.contains(expected),
      "missing query parameter {expected} for GET {}",
      api_paths::sync::PULL
    );
  }
}

#[tokio::test]
async fn openapi_guarded_execute_routes_document_forbidden_response() {
  let db = Arc::new(setup_db().await);
  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);

  let openapi = get(&app, api_paths::docs::OPENAPI_JSON).await;
  assert_eq!(openapi.status(), StatusCode::OK);
  let openapi_json = response_json(openapi).await;
  let paths = openapi_json["paths"]
    .as_object()
    .expect("openapi paths object");

  let guarded_execute_routes = [
    api_paths::acceptance::EXECUTE_BY_ID,
    api_paths::dispatch::EXECUTE_BY_ID,
    api_paths::blending::EXECUTE_BY_ID,
    api_paths::operations::PHYSICAL_TRANSFERS_EXECUTE_BY_ID,
    api_paths::operations::OWNERSHIP_TRANSFERS_EXECUTE_BY_ID,
    api_paths::operations::RECONCILIATIONS_EXECUTE_BY_ID,
  ];

  for path in guarded_execute_routes {
    let responses = paths
      .get(path)
      .and_then(|v| v.as_object())
      .and_then(|methods| methods.get("post"))
      .and_then(|operation| operation.get("responses"))
      .and_then(|v| v.as_object())
      .unwrap_or_else(|| panic!("missing OpenAPI responses for POST {path}"));

    assert!(
      responses.contains_key("403"),
      "missing forbidden response documentation for POST {path}"
    );
  }
}

#[tokio::test]
async fn openapi_document_query_routes_document_validation_error_response() {
  let db = Arc::new(setup_db().await);
  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);

  let openapi = get(&app, api_paths::docs::OPENAPI_JSON).await;
  assert_eq!(openapi.status(), StatusCode::OK);
  let openapi_json = response_json(openapi).await;
  let paths = openapi_json["paths"]
    .as_object()
    .expect("openapi paths object");

  let query_routes = [
    api_paths::acceptance::QUERY,
    api_paths::dispatch::QUERY,
    api_paths::blending::QUERY,
    api_paths::operations::PHYSICAL_TRANSFERS_QUERY,
    api_paths::operations::OWNERSHIP_TRANSFERS_QUERY,
    api_paths::operations::RECONCILIATIONS_QUERY,
  ];

  for path in query_routes {
    let responses = paths
      .get(path)
      .and_then(|v| v.as_object())
      .and_then(|methods| methods.get("get"))
      .and_then(|operation| operation.get("responses"))
      .and_then(|v| v.as_object())
      .unwrap_or_else(|| panic!("missing OpenAPI responses for GET {path}"));

    assert!(
      responses.contains_key("400"),
      "missing validation error response documentation for GET {path}"
    );
  }
}

#[tokio::test]
async fn openapi_sync_query_routes_document_validation_error_response() {
  let db = Arc::new(setup_db().await);
  let state = Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  ));
  let app = build_router(state);

  let openapi = get(&app, api_paths::docs::OPENAPI_JSON).await;
  assert_eq!(openapi.status(), StatusCode::OK);
  let openapi_json = response_json(openapi).await;
  let paths = openapi_json["paths"]
    .as_object()
    .expect("openapi paths object");

  let query_routes = [api_paths::sync::OUTBOUND, api_paths::sync::PULL];

  for path in query_routes {
    let responses = paths
      .get(path)
      .and_then(|v| v.as_object())
      .and_then(|methods| methods.get("get"))
      .and_then(|operation| operation.get("responses"))
      .and_then(|v| v.as_object())
      .unwrap_or_else(|| panic!("missing OpenAPI responses for GET {path}"));

    assert!(
      responses.contains_key("400"),
      "missing validation error response documentation for GET {path}"
    );
  }
}
