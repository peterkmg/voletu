use std::time::Duration;

use serde_json::Value;
use uuid::Uuid;

use super::parse_doc_id;
use crate::common::integration::{
  api_get,
  await_sync_cycle,
  create_acceptance_via_api,
  get_acceptance_composite_json,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

async fn read_pull_watermark(client: &reqwest::Client, base_url: &str, token: &str) -> Uuid {
  let response: Value = api_get(client, &format!("{base_url}/sync/watermarks"), token).await;
  let watermarks = response
    .as_array()
    .expect("sync/watermarks should return an array");
  watermarks
    .iter()
    .find(|w| w["direction"].as_str() == Some("PULL"))
    .and_then(|w| w["lastAuditLogId"].as_str())
    .and_then(|s| Uuid::parse_str(s).ok())
    .unwrap_or_else(Uuid::nil)
}

#[tokio::test]
async fn watermark_advances_strictly_and_no_double_pull_on_second_wave() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r14-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r14-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-INC-1",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "10.0",
  )
  .await;

  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;

  let wave1_count = api_get(&client, &format!("{}/acceptance", pa.url), &pa.token)
    .await
    .as_array()
    .unwrap()
    .len();
  assert_eq!(wave1_count, 1, "wave 1 should sync exactly one acceptance");

  let watermark_after_wave1 = read_pull_watermark(&client, &pa.url, &pa.token).await;
  assert_ne!(
    watermark_after_wave1,
    Uuid::nil(),
    "PULL watermark must advance past nil after the first sync"
  );

  let acc2 = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-INC-2",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "20.0",
  )
  .await;
  let acc2_id = parse_doc_id(&acc2);

  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;

  let watermark_after_wave2 = read_pull_watermark(&client, &pa.url, &pa.token).await;
  assert!(
    watermark_after_wave2 > watermark_after_wave1,
    "PULL watermark must strictly advance between waves: wave1={watermark_after_wave1}, wave2={watermark_after_wave2}"
  );

  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc2_id)
      .await
      .is_some(),
    "wave 2 acceptance should be on PA"
  );
  let wave2_count = api_get(&client, &format!("{}/acceptance", pa.url), &pa.token)
    .await
    .as_array()
    .unwrap()
    .len();
  assert_eq!(
    wave2_count, 2,
    "should have exactly 2 acceptances after wave 2 (strict parity, not just `> wave1_count`)"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
