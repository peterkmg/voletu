use std::time::Duration;

use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use tokio::sync::oneshot;
use uuid::Uuid;
use voletu_core::{
  serve_api,
  serve_api_with_sync_config,
  DbConfig,
  DbParams,
  JwtConfig,
  SyncConfig,
};

use super::api_client::ensure_shared_memory_db_alive;

pub fn db_cfg(db_name: &str) -> DbConfig {
  DbConfig::new(
    DbParams::sqlite_shared_memory(db_name.to_string()),
    "integrationtestpass",
  )
}

pub fn temp_db_path(prefix: &str) -> String {
  format!("{prefix}-{}", Uuid::now_v7())
}

pub fn reserve_port() -> u16 {
  let tcp = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
  let port = tcp.local_addr().unwrap().port();
  drop(tcp);
  port
}

pub fn test_sync_config() -> SyncConfig {
  SyncConfig {
    tick_interval: Duration::from_millis(200),
    probe_timeout: Duration::from_secs(3),
    request_timeout: Duration::from_secs(5),
    sync_batch_limit: 1000,
  }
}

pub async fn spawn_server(
  db_name: &str,
  port: u16,
) -> (
  oneshot::Sender<()>,
  tokio::task::JoinHandle<anyhow::Result<()>>,
) {
  spawn_server_with_sync_config(db_name, port, None).await
}

pub async fn spawn_server_with_sync_config(
  db_name: &str,
  port: u16,
  sync_config: Option<SyncConfig>,
) -> (
  oneshot::Sender<()>,
  tokio::task::JoinHandle<anyhow::Result<()>>,
) {
  ensure_shared_memory_db_alive(db_name).await;
  let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
  let db_cfg = db_cfg(db_name);
  let jwt_cfg = JwtConfig::default();
  let server_task = tokio::spawn(async move {
    serve_api_with_sync_config(
      "127.0.0.1".to_string(),
      port.to_string(),
      db_cfg,
      jwt_cfg,
      sync_config,
      shutdown_rx,
    )
    .await
  });
  (shutdown_tx, server_task)
}

pub async fn wait_for_health(
  client: &Client,
  base_url: &str,
  timeout: Duration,
  server_task: &mut tokio::task::JoinHandle<anyhow::Result<()>>,
) {
  let deadline = tokio::time::Instant::now() + timeout;
  loop {
    if server_task.is_finished() {
      let result = server_task
        .await
        .expect("server task join should succeed while waiting for health");
      panic!("server exited before becoming healthy: {result:?}");
    }

    if let Ok(response) = client.get(format!("{base_url}/health")).send().await {
      if response.status() == StatusCode::OK {
        return;
      }
    }

    assert!(
      tokio::time::Instant::now() < deadline,
      "server did not become healthy at {base_url} within {:?}",
      timeout
    );
    tokio::time::sleep(Duration::from_millis(100)).await;
  }
}

pub async fn wait_for_login_token(
  client: &Client,
  base_url: &str,
  username: &str,
  password: &str,
  timeout: Duration,
) -> String {
  let deadline = tokio::time::Instant::now() + timeout;
  let mut last_status: Option<StatusCode> = None;
  let mut last_body: Option<String> = None;
  loop {
    if let Ok(response) = client
      .post(format!("{base_url}/auth/login"))
      .header("idempotency-key", Uuid::now_v7().to_string())
      .json(&json!({"username": username, "password": password}))
      .send()
      .await
    {
      let status = response.status();
      let body_text = response.text().await.ok();

      last_status = Some(status);
      last_body = body_text.clone();

      if status == StatusCode::OK {
        let body: Value = serde_json::from_str(body_text.as_deref().unwrap_or("{}")).unwrap();
        return body["data"]["accessToken"].as_str().unwrap().to_string();
      }
    }

    assert!(
      tokio::time::Instant::now() < deadline,
      "login for '{username}' did not succeed within {:?}; last_status={:?}; last_body={:?}",
      timeout,
      last_status,
      last_body,
    );
    tokio::time::sleep(Duration::from_millis(120)).await;
  }
}

pub async fn shutdown_server(
  shutdown_tx: oneshot::Sender<()>,
  server_task: tokio::task::JoinHandle<anyhow::Result<()>>,
) {
  let _ = shutdown_tx.send(());
  let join_result = tokio::time::timeout(Duration::from_secs(10), server_task)
    .await
    .expect("server task should shut down in time")
    .expect("server task join should succeed");
  join_result.expect("serve_api should return Ok on shutdown");
}

pub struct NodeHandle {
  pub url: String,
  pub token: String,
  pub shutdown_tx: oneshot::Sender<()>,
  pub task: tokio::task::JoinHandle<anyhow::Result<()>>,
}

impl NodeHandle {
  pub async fn shutdown(self) {
    shutdown_server(self.shutdown_tx, self.task).await;
  }
}
