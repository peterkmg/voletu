use std::sync::Arc;

use axum::http::StatusCode;
use uuid::Uuid;
use voletu_core::{api::router::build_router, db::seed_defaults, endpoints::paths as api_paths};

const INITIAL_AUDIT_CURSOR: Uuid = Uuid::from_u128(1);

use crate::common::{
  catalog_seed::{seed_inventory_catalog, seed_sync_node},
  http::{
    assert_api_error,
    assert_api_success,
    get,
    login_admin_token,
    post_json,
    with_auth_token,
  },
  payloads::{sync_push_insert_company, sync_watermark_upsert},
  setup_db,
  test_api_state_with_default_restart_controls,
  test_config_for_db,
};

#[tokio::test]
async fn push_pull_and_watermark_accept_valid_requests_returning_expected_dto_fields() {
  let db = Arc::new(setup_db().await);
  let _ = seed_defaults(&db).await.unwrap();
  let catalog = seed_inventory_catalog(&db).await;
  let node_id = seed_sync_node(&db, catalog.base_id, "Peripheral A").await;

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
      catalog.base_id,
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
async fn pull_with_malformed_query_uuid_returns_structured_validation_error() {
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
async fn outbound_with_malformed_query_uuid_returns_structured_validation_error() {
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
