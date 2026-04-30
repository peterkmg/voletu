use std::time::Duration;

use super::parse_doc_id;
use crate::common::integration::{
  await_sync_cycle,
  create_acceptance_via_api,
  execute_document_via_api,
  get_acceptance_composite_json,
  revert_document_via_api,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn status_transitions_replicate_to_peripheral_after_each_change() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r18-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r18-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  let acc = create_acceptance_via_api(
    &client,
    &central.url,
    &central.token,
    "ACC-LIFE-001",
    catalog.contractor,
    catalog.product,
    catalog.storage_alpha,
    "300.0",
  )
  .await;
  let acc_id = parse_doc_id(&acc);

  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;
  let pa_acc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
    .await
    .unwrap();
  assert_eq!(pa_acc["status"], "DRAFT");

  execute_document_via_api(
    &client,
    &central.url,
    &central.token,
    "/acceptance/execute/{id}",
    acc_id,
  )
  .await;
  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;
  let pa_acc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
    .await
    .unwrap();
  assert_eq!(pa_acc["status"], "EXECUTED");

  revert_document_via_api(
    &client,
    &central.url,
    &central.token,
    "/acceptance/revert/{id}",
    acc_id,
  )
  .await;
  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;
  let pa_acc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
    .await
    .unwrap();
  assert_eq!(pa_acc["status"], "DRAFT");

  execute_document_via_api(
    &client,
    &central.url,
    &central.token,
    "/acceptance/execute/{id}",
    acc_id,
  )
  .await;
  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;
  let pa_acc = get_acceptance_composite_json(&client, &pa.url, &pa.token, acc_id)
    .await
    .unwrap();
  assert_eq!(pa_acc["status"], "EXECUTED");

  central.shutdown().await;
  pa.shutdown().await;
}
