use std::time::Duration;

use reqwest::{Client, StatusCode};
use serde_json::Value;

use super::api_get;

pub async fn await_sync_cycle(client: &Client, base_url: &str, token: &str, timeout: Duration) {
  let timeout_secs = timeout.as_secs().max(1);
  let response = client
    .get(format!(
      "{base_url}/sync/await-cycle?timeout={timeout_secs}"
    ))
    .bearer_auth(token)
    .timeout(timeout + Duration::from_secs(5))
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
