use std::time::Duration;

use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::common::integration::{
  api_get,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
  wait_for_worker_online,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(20);

#[tokio::test]
async fn rejects_invalid_urls_and_persists_valid_url_to_db() {
  let client = reqwest::Client::new();

  let central_a = setup_central_via_api(&client, &temp_db_path("url-change-central-a")).await;
  let catalog_a = seed_catalog_via_api(&client, &central_a.url, &central_a.token).await;
  let peripheral = setup_peripheral_via_api(
    &client,
    &temp_db_path("url-change-peripheral"),
    &central_a,
    &[catalog_a.base_alpha],
  )
  .await;
  wait_for_worker_online(&client, &peripheral.url, &peripheral.token, SYNC_TIMEOUT).await;

  let central_b = setup_central_via_api(&client, &temp_db_path("url-change-central-b")).await;

  let patch_url = format!("{}/node/central-api-url", peripheral.url);

  {
    let resp = client
      .patch(&patch_url)
      .bearer_auth(&peripheral.token)
      .header("idempotency-key", uuid::Uuid::now_v7().to_string())
      .json(&json!({ "url": "not-a-url" }))
      .send()
      .await
      .unwrap();
    assert_eq!(
      resp.status(),
      StatusCode::BAD_REQUEST,
      "malformed URL must yield 400; got {}",
      resp.status()
    );
  }

  {
    let resp = client
      .patch(&patch_url)
      .bearer_auth(&peripheral.token)
      .header("idempotency-key", uuid::Uuid::now_v7().to_string())
      .json(&json!({ "url": "http://127.0.0.1:1" }))
      .send()
      .await
      .unwrap();
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let status = api_get(
      &client,
      &format!("{}/node/status", peripheral.url),
      &peripheral.token,
    )
    .await;
    assert_eq!(
      status["centralApiUrl"].as_str().unwrap_or_default(),
      central_a.url,
      "failed probe must not persist the URL change"
    );
  }

  {
    let resp = client
      .patch(&patch_url)
      .bearer_auth(&peripheral.token)
      .header("idempotency-key", uuid::Uuid::now_v7().to_string())
      .json(&json!({ "url": central_b.url.clone() }))
      .send()
      .await
      .unwrap();
    assert_eq!(
      resp.status(),
      StatusCode::OK,
      "happy path must yield 200; body: {:?}",
      resp.text().await.ok()
    );
    let body: Value = resp.json().await.unwrap();
    assert_eq!(body["success"], Value::Bool(true));
    assert_eq!(
      body["data"]["centralApiUrl"].as_str().unwrap_or_default(),
      central_b.url,
      "response must reflect the newly-persisted URL"
    );
  }

  let status = api_get(
    &client,
    &format!("{}/node/status", peripheral.url),
    &peripheral.token,
  )
  .await;
  assert_eq!(
    status["centralApiUrl"].as_str().unwrap_or_default(),
    central_b.url,
    "status endpoint must reflect the new URL without a restart"
  );

  central_a.shutdown().await;
  central_b.shutdown().await;
  peripheral.shutdown().await;
}
