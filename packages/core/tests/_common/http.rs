#![allow(dead_code)]

use std::{future::Future, sync::Arc};

use axum::{
  body::{to_bytes, Body},
  http::{Request, StatusCode},
  response::Response,
  Router,
};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use tower::ServiceExt;
use uuid::Uuid;
use voletu_core::{api::router::build_router, db::seed_defaults, endpoints::paths as api_paths};

use super::{
  payloads::auth_login,
  setup_db,
  test_api_state_with_default_restart_controls,
  test_config_for_db,
};

tokio::task_local! {
  static AUTH_TOKEN: String;
}

fn maybe_auth(builder: axum::http::request::Builder) -> axum::http::request::Builder {
  if let Ok(token) = AUTH_TOKEN.try_with(|token| token.clone()) {
    builder.header("authorization", format!("Bearer {}", token))
  } else {
    builder
  }
}

fn with_idempotency_key(builder: axum::http::request::Builder) -> axum::http::request::Builder {
  builder.header("idempotency-key", Uuid::now_v7().to_string())
}

pub async fn post_json(app: &Router, uri: impl AsRef<str>, json_body: impl Into<Body>) -> Response {
  app
    .clone()
    .oneshot(
      maybe_auth(with_idempotency_key(
        Request::builder()
          .method("POST")
          .uri(uri.as_ref())
          .header("content-type", "application/json"),
      ))
      .body(json_body.into())
      .unwrap(),
    )
    .await
    .unwrap()
}

pub async fn post_empty(app: &Router, uri: impl AsRef<str>) -> Response {
  app
    .clone()
    .oneshot(
      maybe_auth(with_idempotency_key(
        Request::builder().method("POST").uri(uri.as_ref()),
      ))
      .body(Body::empty())
      .unwrap(),
    )
    .await
    .unwrap()
}

pub async fn get(app: &Router, uri: impl AsRef<str>) -> Response {
  app
    .clone()
    .oneshot(
      maybe_auth(Request::builder().uri(uri.as_ref()))
        .body(Body::empty())
        .unwrap(),
    )
    .await
    .unwrap()
}

pub async fn delete(app: &Router, uri: impl AsRef<str>) -> Response {
  app
    .clone()
    .oneshot(
      maybe_auth(with_idempotency_key(
        Request::builder().method("DELETE").uri(uri.as_ref()),
      ))
      .body(Body::empty())
      .unwrap(),
    )
    .await
    .unwrap()
}

pub fn assert_ok(status: StatusCode) {
  assert_eq!(status, StatusCode::OK);
}

pub async fn response_json(response: Response) -> Value {
  let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
  serde_json::from_slice(&bytes).unwrap()
}

pub async fn assert_api_success(response: Response) -> Value {
  let status = response.status();
  let json = response_json(response).await;
  assert_eq!(
    status,
    StatusCode::OK,
    "unexpected status {status}; response body: {json}"
  );
  assert_eq!(json["success"], Value::Bool(true));
  assert!(json.get("data").is_some());
  assert!(json.get("error").is_none());
  json
}

pub async fn assert_api_error(
  response: Response,
  expected_status: StatusCode,
  expected_code: &str,
  message_substring: Option<&str>,
) -> Value {
  assert_eq!(response.status(), expected_status);
  let json = response_json(response).await;
  assert_eq!(json["success"], Value::Bool(false));
  assert!(json.get("data").is_none());

  let error = json.get("error").and_then(Value::as_object).unwrap();
  assert_eq!(
    error.get("code").and_then(Value::as_str),
    Some(expected_code)
  );
  let message = error.get("message").and_then(Value::as_str).unwrap();
  if let Some(substr) = message_substring {
    assert!(
      message.contains(substr),
      "expected error message '{message}' to contain '{substr}'"
    );
  }
  json
}

pub async fn login_admin_token(app: &Router) -> String {
  let response = post_json(app, api_paths::auth::LOGIN, auth_login("admin", "admin")).await;
  let json = assert_api_success(response).await;
  json["data"]["accessToken"]
    .as_str()
    .expect("access token should exist")
    .to_string()
}

pub async fn setup_seeded_app() -> (Arc<DatabaseConnection>, Router) {
  let db = Arc::new(setup_db().await);
  let _ = seed_defaults(&db).await.unwrap();
  let app = build_router(Arc::new(test_api_state_with_default_restart_controls(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
  )));
  (db, app)
}

pub async fn setup_seeded_app_with_admin_token() -> (Arc<DatabaseConnection>, Router, String) {
  let (db, app) = setup_seeded_app().await;
  let token = login_admin_token(&app).await;
  (db, app, token)
}

pub async fn with_auth_token<T, Fut>(token: String, fut: Fut) -> T
where
  Fut: Future<Output = T>,
{
  AUTH_TOKEN.scope(token, fut).await
}
