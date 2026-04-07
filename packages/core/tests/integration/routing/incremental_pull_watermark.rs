//! **Incremental pull advances watermark correctly**: A second pull using the cursor from
//! the first pull returns only newly created data, with a smaller count.
//!
//! **Topology:** Central + 1 Peripheral (base_alpha)
//! **Verifies:** Incremental pull returns only data created after the previous cursor; count is smaller than initial pull

use crate::common::integration::{
  create_acceptance_via_api,
  get_acceptance_composite_json,
  pull_from_central_to_target,
  pull_from_central_to_target_after,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

use super::parse_doc_id;

#[tokio::test]
async fn incremental_pull_advances_watermark_correctly() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r14-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r14-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  let _acc1 = create_acceptance_via_api(
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

  // First pull — catalog + first doc
  let (pull1_count, pull1_cursor) = pull_from_central_to_target(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
  )
  .await;
  assert!(pull1_count > 0);

  // Create second doc
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

  // Incremental pull from cursor — only new data
  let (pull2_count, _) = pull_from_central_to_target_after(
    &client,
    &central.url,
    &central.token,
    &pa.url,
    &pa.token,
    &[catalog.base_alpha],
    pull1_cursor,
  )
  .await;
  assert!(pull2_count > 0, "incremental pull should return new data");
  assert!(
    pull2_count < pull1_count,
    "incremental should be smaller than initial"
  );
  assert!(
    get_acceptance_composite_json(&client, &pa.url, &pa.token, acc2_id)
      .await
      .is_some()
  );

  central.shutdown().await;
  pa.shutdown().await;
}
