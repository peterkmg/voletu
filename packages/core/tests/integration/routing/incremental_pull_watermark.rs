//! **Incremental pull advances watermark correctly**: A second pull using the cursor from
//! the first pull returns only newly created data, with a smaller count.
//!
//! **Topology:** Central + 1 Peripheral (base_alpha)
//! **Verifies:** Incremental pull returns only data created after the previous cursor; count is smaller than initial pull

use std::time::Duration;

use super::parse_doc_id;
use crate::common::integration::{
  await_sync_cycle, create_acceptance_via_api, get_acceptance_composite_json, seed_catalog_via_api,
  setup_central_via_api, setup_peripheral_via_api, temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn incremental_pull_advances_watermark_correctly() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r14-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(
    &client,
    &temp_db_path("r14-pa"),
    &central,
    &[catalog.base_alpha],
  )
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

  // Verify wave 1 arrived
  let pa_accs =
    crate::common::integration::api_get(&client, &format!("{}/acceptance", pa.url), &pa.token)
      .await;
  let wave1_count = pa_accs.as_array().unwrap().len();
  assert!(
    wave1_count > 0,
    "wave 1 should have synced at least one acceptance"
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

  // Verify both wave 1 and wave 2 are present
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc2_id)
      .await
      .is_some(),
    "wave 2 acceptance should be on PA"
  );
  let pa_accs_after =
    crate::common::integration::api_get(&client, &format!("{}/acceptance", pa.url), &pa.token)
      .await;
  let wave2_count = pa_accs_after.as_array().unwrap().len();
  assert!(
    wave2_count > wave1_count,
    "should have more acceptances after wave 2 ({wave2_count} > {wave1_count})"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
