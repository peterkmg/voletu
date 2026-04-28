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
use uuid::Uuid;

use crate::common::{
  integration::{
    spawn_server_with_sync_config,
    temp_db_path,
    test_sync_config,
    wait_for_health,
    wait_for_login_token,
  },
  payloads::{auth_login, node_initialize_replace_with_node_type},
};

#[tokio::test]
async fn old_credentials_rejected_and_new_configuration_loaded_after_init() {
  let client = Client::new();
  let db_name = temp_db_path("init-restart");
  let port = crate::common::integration::reserve_port();

  let (shutdown_tx, mut server_task) =
    spawn_server_with_sync_config(&db_name, port, Some(test_sync_config())).await;

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
}
