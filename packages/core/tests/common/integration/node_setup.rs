use std::time::Duration;

use reqwest::Client;
use serde_json::json;
use uuid::Uuid;

use super::{
  api_post,
  server::{
    reserve_port, spawn_server_with_sync_config, test_sync_config, wait_for_health,
    wait_for_login_token, NodeHandle,
  },
  wait::{await_sync_cycle, wait_for_worker_online},
};

/// Add a base assignment to a running node via HTTP API (POST /node/bases).
pub async fn add_base_assignment_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  base_id: Uuid,
) {
  api_post(
    client,
    &format!("{base_url}/node/bases"),
    token,
    json!({ "baseId": base_id }),
  )
  .await;
}

/// Spawn a server and initialize it as Central via API only.
/// Central nodes do not run the sync worker (no `centralApiUrl`).
pub async fn setup_central_via_api(client: &Client, db_name: &str) -> NodeHandle {
  let port = reserve_port();
  let (shutdown_tx, mut task) =
    spawn_server_with_sync_config(db_name, port, Some(test_sync_config())).await;
  let url = format!("http://127.0.0.1:{port}");
  wait_for_health(client, &url, Duration::from_secs(10), &mut task).await;

  let bootstrap_token =
    wait_for_login_token(client, &url, "admin", "admin", Duration::from_secs(5)).await;

  api_post(
    client,
    &format!("{url}/node/initialize"),
    &bootstrap_token,
    json!({
      "nodeType": "CENTRAL",
      "nodeName": "Central",
      "centralApiUrl": null,
      "newUsername": "root",
      "newPassword": "rootpass",
      "fullname": "Root Admin",
    }),
  )
  .await;

  wait_for_health(client, &url, Duration::from_secs(20), &mut task).await;
  let token = wait_for_login_token(client, &url, "root", "rootpass", Duration::from_secs(10)).await;

  NodeHandle {
    url,
    token,
    shutdown_tx,
    task,
  }
}

/// Spawn a server and initialize it as a Peripheral via API only.
/// The real sync worker starts automatically after initialization (central_api_url is set).
/// Waits for the worker to come online and complete the initial catalog sync.
pub async fn setup_peripheral_via_api(
  client: &Client,
  db_name: &str,
  central: &NodeHandle,
  base_ids: &[Uuid],
) -> NodeHandle {
  let port = reserve_port();
  let (shutdown_tx, mut task) =
    spawn_server_with_sync_config(db_name, port, Some(test_sync_config())).await;
  let url = format!("http://127.0.0.1:{port}");
  wait_for_health(client, &url, Duration::from_secs(10), &mut task).await;

  let bootstrap_token =
    wait_for_login_token(client, &url, "admin", "admin", Duration::from_secs(5)).await;

  api_post(
    client,
    &format!("{url}/node/initialize"),
    &bootstrap_token,
    json!({
      "nodeType": "PERIPHERAL",
      "nodeName": null,
      "centralApiUrl": central.url,
      "newUsername": "root",
      "newPassword": "rootpass",
      "fullname": "Root Admin",
    }),
  )
  .await;

  wait_for_health(client, &url, Duration::from_secs(20), &mut task).await;
  let token = wait_for_login_token(client, &url, "root", "rootpass", Duration::from_secs(10)).await;

  // Wait for the REAL sync worker to come online and complete initial catalog sync.
  // The worker transitions to OnlineIdle only AFTER a successful sync_once cycle,
  // so by the time this returns, catalog is already synced.
  // Catalog must sync BEFORE adding base assignments (bases must exist locally).
  wait_for_worker_online(client, &url, &token, Duration::from_secs(15)).await;

  // Add base assignments via API (bases now exist from catalog sync)
  for base_id in base_ids {
    add_base_assignment_via_api(client, &url, &token, *base_id).await;
  }

  NodeHandle {
    url,
    token,
    shutdown_tx,
    task,
  }
}
