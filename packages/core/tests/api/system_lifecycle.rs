use std::sync::{Arc, Mutex};

use axum::http::StatusCode;
use tokio::sync::oneshot;
use uuid::Uuid;
use voletu_core::{
  api::{router::build_router, ApiState},
  db::seed_defaults,
  endpoints::paths as api_paths,
  utils::jwt::generate_secret,
};

use crate::common::{
  http::{
    assert_api_error,
    assert_api_success,
    login_admin_token,
    post_empty,
    post_json,
    with_auth_token,
  },
  payloads::{auth_login, user_create},
  setup_db,
  test_config_for_db,
};

#[tokio::test]
async fn restart_endpoint_requires_admin_role() {
  let db = Arc::new(setup_db().await);
  let _ = seed_defaults(&db).await.unwrap();

  let (restart_tx, restart_rx) = oneshot::channel::<()>();
  tokio::spawn(async move {
    let _ = restart_rx.await;
  });

  let app = build_router(Arc::new(ApiState::new(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
    Arc::new(Mutex::new(Some(restart_tx))),
    Arc::new(tokio::sync::RwLock::new(
      voletu_core::worker::WorkerStatus::default(),
    )),
    true,
  )));

  let admin_token = login_admin_token(&app).await;
  with_auth_token(admin_token, async {
    let create_operator = post_json(
      &app,
      api_paths::users::ROOT,
      user_create(
        "restart-operator",
        "operator-pass",
        "Restart Operator",
        "operator",
      ),
    )
    .await;
    let _ = assert_api_success(create_operator).await;
  })
  .await;

  let operator_login = post_json(
    &app,
    api_paths::auth::LOGIN,
    auth_login("restart-operator", "operator-pass"),
  )
  .await;
  let operator_login_json = assert_api_success(operator_login).await;
  let operator_token = operator_login_json["data"]["accessToken"]
    .as_str()
    .unwrap()
    .to_string();

  with_auth_token(operator_token, async {
    let restart = post_empty(&app, api_paths::node::RESTART).await;
    let _ = assert_api_error(
      restart,
      StatusCode::FORBIDDEN,
      "FORBIDDEN",
      Some("Only admin"),
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn restart_endpoint_accepts_admin_and_rejects_repeated_trigger() {
  let db = Arc::new(setup_db().await);
  let _ = seed_defaults(&db).await.unwrap();

  let (restart_tx, restart_rx) = oneshot::channel::<()>();
  tokio::spawn(async move {
    let _ = restart_rx.await;
  });

  let app = build_router(Arc::new(ApiState::new(
    db.clone(),
    Arc::new(test_config_for_db(&db).await),
    Arc::new(Mutex::new(Some(restart_tx))),
    Arc::new(tokio::sync::RwLock::new(
      voletu_core::worker::WorkerStatus::default(),
    )),
    true,
  )));

  let admin_token = login_admin_token(&app).await;
  with_auth_token(admin_token, async {
    let first_restart = post_empty(&app, api_paths::node::RESTART).await;
    let first_restart_json = assert_api_success(first_restart).await;
    assert_eq!(
      first_restart_json["data"]["message"],
      "API restart initiated"
    );

    let second_restart = post_empty(&app, api_paths::node::RESTART).await;
    let _ = assert_api_error(
      second_restart,
      StatusCode::CONFLICT,
      "CONFLICT",
      Some("already in progress"),
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn restart_endpoint_rejects_access_tokens_signed_with_a_different_secret() {
  let db = Arc::new(setup_db().await);
  let _ = seed_defaults(&db).await.unwrap();

  let mut original_cfg = test_config_for_db(&db).await;
  original_cfg.node.jwt_secret = generate_secret();

  let original_app = build_router(Arc::new(ApiState::new(
    db.clone(),
    Arc::new(original_cfg.clone()),
    Arc::new(Mutex::new(None)),
    Arc::new(tokio::sync::RwLock::new(
      voletu_core::worker::WorkerStatus::default(),
    )),
    true,
  )));
  let stale_token = login_admin_token(&original_app).await;

  let mut other_cfg = original_cfg;
  other_cfg.node.jwt_secret = generate_secret();
  other_cfg.node = voletu_core::config::NodeConfig::new(
    Uuid::now_v7(),
    "PERIPHERAL".to_string(),
    other_cfg.node.jwt_secret.clone(),
    None,
  );

  let protected_app = build_router(Arc::new(ApiState::new(
    db,
    Arc::new(other_cfg),
    Arc::new(Mutex::new(None)),
    Arc::new(tokio::sync::RwLock::new(
      voletu_core::worker::WorkerStatus::default(),
    )),
    true,
  )));

  with_auth_token(stale_token, async {
    let restart = post_empty(&protected_app, api_paths::node::RESTART).await;
    let _ = assert_api_error(
      restart,
      StatusCode::UNAUTHORIZED,
      "UNAUTHORIZED",
      Some("Invalid access token"),
    )
    .await;
  })
  .await;
}
