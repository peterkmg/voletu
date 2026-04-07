//! Verifies that calling the node initialization endpoint triggers a server restart
//! and that the new credentials work after the restart.
//!
//! Topology: single standalone node (no sync peers).
//!
//! Property: after initialization the old default credentials are rejected,
//! the new root credentials are accepted, and the sync status reports the
//! configured node type.

use std::time::Duration;

use reqwest::{Client, StatusCode};
use serde_json::Value;
use tokio::sync::oneshot;
use uuid::Uuid;
use voletu_core::{serve_api, DbConfig, DbParams, JwtConfig};

use crate::common::{
  integration::{wait_for_health, wait_for_login_token},
  payloads::{auth_login, node_initialize_replace_with_node_type},
};

#[tokio::test]
async fn initialize_endpoint_triggers_restart_and_reloads_node_configuration() {
  let tcp = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
  let port = tcp.local_addr().unwrap().port();
  drop(tcp);

  let db_path = std::env::temp_dir().join(format!("voletu-init-restart-{}.db", Uuid::now_v7()));
  let db_cfg = DbConfig::new(DbParams::sqlite(db_path.clone()), "integrationtestpass");
  let jwt_cfg = JwtConfig::default();

  let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
  let mut server_task = tokio::spawn(async move {
    serve_api(
      "127.0.0.1".to_string(),
      port.to_string(),
      db_cfg,
      jwt_cfg,
      shutdown_rx,
    )
    .await
  });

  let client = Client::new();
  let base_url = format!("http://127.0.0.1:{port}");

  wait_for_health(
    &client,
    &base_url,
    Duration::from_secs(10),
    &mut server_task,
  )
  .await;

  let admin_token =
    wait_for_login_token(&client, &base_url, "admin", "admin", Duration::from_secs(5)).await;

  let init_response = client
    .post(format!("{base_url}/node/initialize"))
    .header("idempotency-key", Uuid::now_v7().to_string())
    .bearer_auth(&admin_token)
    .json(
      &serde_json::from_str::<Value>(&node_initialize_replace_with_node_type(
        "root",
        "root-password",
        "Root User",
        "CENTRAL",
      ))
      .unwrap(),
    )
    .send()
    .await
    .unwrap();
  assert_eq!(init_response.status(), StatusCode::OK);

  let init_body: Value = init_response.json().await.unwrap();
  assert_eq!(init_body["data"]["message"], "Initialization completed");

  wait_for_health(
    &client,
    &base_url,
    Duration::from_secs(20),
    &mut server_task,
  )
  .await;

  let root_token = wait_for_login_token(
    &client,
    &base_url,
    "root",
    "root-password",
    Duration::from_secs(20),
  )
  .await;

  let old_admin_response = client
    .post(format!("{base_url}/auth/login"))
    .header("idempotency-key", Uuid::now_v7().to_string())
    .json(&serde_json::from_str::<Value>(&auth_login("admin", "admin")).unwrap())
    .send()
    .await
    .unwrap();
  assert_eq!(old_admin_response.status(), StatusCode::UNAUTHORIZED);

  let sync_status_response = client
    .get(format!("{base_url}/sync/status"))
    .bearer_auth(root_token)
    .send()
    .await
    .unwrap();
  assert_eq!(sync_status_response.status(), StatusCode::OK);
  let sync_status_body: Value = sync_status_response.json().await.unwrap();
  assert_eq!(sync_status_body["data"]["nodeType"], "CENTRAL");

  let _ = shutdown_tx.send(());

  let join_result = tokio::time::timeout(Duration::from_secs(10), server_task)
    .await
    .expect("server task should shut down in time")
    .expect("server task join should succeed");
  join_result.expect("serve_api should return Ok on shutdown");

  let _ = std::fs::remove_file(db_path);
}
