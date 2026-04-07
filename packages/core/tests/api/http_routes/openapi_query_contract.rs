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
    (api_paths::acceptance::QUERY, &[
      "documentNumber",
      "status",
      "page",
      "per_page",
    ]),
    (api_paths::dispatch::QUERY, &[
      "documentNumber",
      "status",
      "contractorId",
      "page",
      "per_page",
    ]),
    (api_paths::blending::QUERY, &[
      "documentNumber",
      "status",
      "contractorId",
      "page",
      "per_page",
    ]),
    (api_paths::operations::PHYSICAL_TRANSFERS_QUERY, &[
      "documentNumber",
      "status",
      "page",
      "per_page",
    ]),
    (api_paths::operations::OWNERSHIP_TRANSFERS_QUERY, &[
      "status", "page", "per_page",
    ]),
    (api_paths::operations::RECONCILIATIONS_QUERY, &[
      "documentNumber",
      "status",
      "warehouseId",
      "page",
      "per_page",
    ]),
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
  for expected in ["lastAuditLogId", "baseIds", "limit"] {
    assert!(
      pull_params.contains(expected),
      "missing query parameter {expected} for GET {}",
      api_paths::sync::PULL
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
