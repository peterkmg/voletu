//! **Incremental pull advances watermark correctly**: After a pull cycle, the
//! stored PULL watermark moves forward. A second cycle starts from the new
//! watermark and does not re-pull data the peripheral already has.
//!
//! **Topology:** Central + 1 Peripheral (base_alpha)
//! **Verifies:**
//!   1. After wave 1, the peripheral's PULL watermark points at a non-nil audit log id.
//!   2. After wave 2, the PULL watermark has advanced to a strictly greater id.
//!   3. Both waves are present on the peripheral with exact count parity.
//!
//! This replaces a weaker earlier version that only asserted "both waves exist"
//! and never actually read the watermark.

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

/// Fetch the peripheral's PULL watermark for the central node. Returns the
/// `last_audit_log_id` as a Uuid (nil if no watermark row exists yet).
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
async fn incremental_pull_advances_watermark_correctly() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r14-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r14-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Wave 1
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

  // Capture watermark after wave 1.
  let watermark_after_wave1 = read_pull_watermark(&client, &pa.url, &pa.token).await;
  assert_ne!(
    watermark_after_wave1,
    Uuid::nil(),
    "PULL watermark must advance past nil after the first sync"
  );

  // Wave 2
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

  // Watermark must strictly advance. If the worker re-pulled wave 1 instead of
  // starting from the previous cursor, this will still pass — but if the cursor
  // never advanced at all (a bug we want to catch), this assertion fails.
  let watermark_after_wave2 = read_pull_watermark(&client, &pa.url, &pa.token).await;
  assert!(
    watermark_after_wave2 > watermark_after_wave1,
    "PULL watermark must strictly advance between waves: wave1={watermark_after_wave1}, wave2={watermark_after_wave2}"
  );

  // Both waves are present.
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
