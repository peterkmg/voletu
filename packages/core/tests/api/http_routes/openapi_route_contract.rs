use std::{collections::HashSet, sync::Arc};

use axum::http::StatusCode;
use voletu_core::{api::router::build_router, endpoints::paths as api_paths};

use crate::common::{
  http::{get, response_json},
  setup_db,
  test_api_state_with_default_restart_controls,
  test_config_for_db,
};

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
  let methods = [
    "get", "post", "put", "delete", "patch", "head", "options", "trace",
  ];

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

  assert!(
    operations_total > 0,
    "OpenAPI must contain at least one operation"
  );
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
    (
      api_paths::operations::PHYSICAL_TRANSFERS_SAVE_AND_EXECUTE,
      "post",
    ),
    (
      api_paths::operations::PHYSICAL_TRANSFERS_EXECUTE_BY_ID,
      "post",
    ),
    (
      api_paths::operations::PHYSICAL_TRANSFERS_REVERT_BY_ID,
      "post",
    ),
    (api_paths::operations::OWNERSHIP_TRANSFERS, "get"),
    (api_paths::operations::OWNERSHIP_TRANSFERS_QUERY, "get"),
    (api_paths::operations::OWNERSHIP_TRANSFERS_SAVE, "post"),
    (
      api_paths::operations::OWNERSHIP_TRANSFERS_SAVE_AND_EXECUTE,
      "post",
    ),
    (
      api_paths::operations::OWNERSHIP_TRANSFERS_EXECUTE_BY_ID,
      "post",
    ),
    (
      api_paths::operations::OWNERSHIP_TRANSFERS_REVERT_BY_ID,
      "post",
    ),
    (api_paths::operations::RECONCILIATIONS, "get"),
    (api_paths::operations::RECONCILIATIONS_QUERY, "get"),
    (api_paths::operations::RECONCILIATIONS_SAVE, "post"),
    (
      api_paths::operations::RECONCILIATIONS_SAVE_AND_EXECUTE,
      "post",
    ),
    (api_paths::operations::RECONCILIATIONS_EXECUTE_BY_ID, "post"),
    (api_paths::operations::RECONCILIATIONS_REVERT_BY_ID, "post"),
    (api_paths::operations::RECONCILIATION_ADJUSTMENTS, "get"),
    (
      api_paths::operations::RECONCILIATION_ADJUSTMENTS_SAVE,
      "post",
    ),
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
    (
      api_paths::operations::PHYSICAL_TRANSFERS_EXECUTE_BY_ID,
      "get",
    ),
    (
      api_paths::operations::OWNERSHIP_TRANSFERS_EXECUTE_BY_ID,
      "get",
    ),
    (api_paths::operations::RECONCILIATIONS_EXECUTE_BY_ID, "get"),
    (api_paths::acceptance::REVERT_BY_ID, "get"),
    (api_paths::dispatch::REVERT_BY_ID, "get"),
    (api_paths::blending::REVERT_BY_ID, "get"),
    (
      api_paths::operations::PHYSICAL_TRANSFERS_REVERT_BY_ID,
      "get",
    ),
    (
      api_paths::operations::OWNERSHIP_TRANSFERS_REVERT_BY_ID,
      "get",
    ),
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
