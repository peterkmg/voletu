use std::time::Duration;

use reqwest::{Client, StatusCode};
use serde_json::Value;

use super::api_get;

/// Wait for the sync worker to complete a cycle via the `/sync/await-cycle` endpoint.
///
/// Returns immediately if the worker is OnlineIdle with a completed sync (nothing to do).
/// Otherwise blocks until the next cycle completes (server-side Notify, zero polling).
pub async fn await_sync_cycle(client: &Client, base_url: &str, token: &str, timeout: Duration) {
  let timeout_secs = timeout.as_secs().max(1);
  let response = client
    .get(format!(
      "{base_url}/sync/await-cycle?timeout={timeout_secs}"
    ))
    .bearer_auth(token)
    .timeout(timeout + Duration::from_secs(5)) // HTTP timeout > server timeout
    .send()
    .await
    .expect("await-cycle request failed");

  assert_eq!(response.status(), StatusCode::OK);
  let body: Value = response.json().await.unwrap();
  assert_eq!(body["success"], Value::Bool(true));
  assert_eq!(
    body["data"]["completed"],
    Value::Bool(true),
    "sync cycle did not complete within {timeout_secs}s; worker_state={}",
    body["data"]["workerState"]
  );
}

/// Wait for the sync worker to reach an active state (OnlineIdle or Syncing).
/// Uses polling on `GET /node/status` — needed during setup before the worker is running.
pub async fn wait_for_worker_online(
  client: &Client,
  base_url: &str,
  token: &str,
  timeout: Duration,
) {
  let deadline = tokio::time::Instant::now() + timeout;
  loop {
    let status = api_get(client, &format!("{base_url}/node/status"), token).await;
    let state = status["workerState"].as_str().unwrap_or("");
    if state == "OnlineIdle" || state == "Syncing" {
      return;
    }

    assert!(
      tokio::time::Instant::now() < deadline,
      "worker did not come online within {timeout:?}; last state={state}"
    );
    tokio::time::sleep(Duration::from_millis(100)).await;
  }
}

/// Poll an async predicate until it returns true or the timeout is reached.
/// Use for cross-node verification where we need to confirm data appeared.
pub async fn poll_until<F, Fut>(predicate: F, timeout: Duration, label: &str)
where
  F: Fn() -> Fut,
  Fut: std::future::Future<Output = bool>,
{
  let deadline = tokio::time::Instant::now() + timeout;
  loop {
    if predicate().await {
      return;
    }
    assert!(
      tokio::time::Instant::now() < deadline,
      "poll_until timed out: {label}"
    );
    tokio::time::sleep(Duration::from_millis(100)).await;
  }
}
