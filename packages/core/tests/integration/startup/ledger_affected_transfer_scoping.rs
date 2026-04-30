use std::time::Duration;

use super::parse_doc_id;
use crate::common::integration::{
  api_post,
  await_sync_cycle,
  get_all_ledger_balances,
  get_physical_transfer_composite_json,
  get_storages_for_base,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

#[tokio::test]
async fn targets_shared_then_local_scope_via_worker() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("s5-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("s5-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("s5-pb"), &central, &[
    catalog.base_beta
  ])
  .await;
  let alpha_storage_ids =
    get_storages_for_base(&client, &central.url, &central.token, catalog.base_alpha).await;
  let beta_storage_ids =
    get_storages_for_base(&client, &central.url, &central.token, catalog.base_beta).await;
  let summarize = |entries: &[serde_json::Value]| {
    let mut rows = entries
      .iter()
      .map(|entry| {
        (
          entry["storageId"].as_str().unwrap_or_default().to_string(),
          entry["productId"].as_str().unwrap_or_default().to_string(),
          entry["contractorId"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
          entry["quantity"].to_string(),
        )
      })
      .collect::<Vec<_>>();
    rows.sort();
    rows
  };
  let filter_by_storage = |entries: &[serde_json::Value], allowed: &[uuid::Uuid]| {
    entries
      .iter()
      .filter(|entry| {
        entry["storageId"]
          .as_str()
          .and_then(|id| uuid::Uuid::parse_str(id).ok())
          .map(|id| allowed.contains(&id))
          .unwrap_or(false)
      })
      .cloned()
      .collect::<Vec<_>>()
  };

  let transfer_1 = api_post(
    &client,
    &format!("{}/physical-transfers/save-and-execute", central.url),
    &central.token,
    serde_json::json!({
      "documentNumber": "PHYS-SHARED-W1",
      "date": "2026-01-15T10:00:00Z",
      "contractorId": catalog.contractor,
      "startCargoOps": "2026-01-15T08:00:00Z",
      "endCargoOps": "2026-01-15T16:00:00Z",
      "items": [{
        "productId": catalog.product,
        "fromStorageId": catalog.storage_alpha,
        "toStorageId": catalog.storage_beta,
        "amount": "120.0",
      }]
    }),
  )
  .await;
  let transfer_1_id = parse_doc_id(&transfer_1);

  await_sync_cycle(&client, &pa.url, &pa.token, Duration::from_secs(15)).await;
  await_sync_cycle(&client, &pb.url, &pb.token, Duration::from_secs(15)).await;

  let pa_t1 =
    get_physical_transfer_composite_json(&client, &pa.url, &pa.token, transfer_1_id).await;
  let pb_t1 =
    get_physical_transfer_composite_json(&client, &pb.url, &pb.token, transfer_1_id).await;
  assert!(pa_t1.is_some(), "PA should have wave-1 transfer");
  assert!(pb_t1.is_some(), "PB should have wave-1 transfer");

  let central_t1 =
    get_physical_transfer_composite_json(&client, &central.url, &central.token, transfer_1_id)
      .await
      .unwrap();
  assert_eq!(
    central_t1["items"][0]["amount"],
    pa_t1.unwrap()["items"][0]["amount"],
    "PA wave-1 amount should match Central"
  );
  assert_eq!(
    central_t1["items"][0]["amount"],
    pb_t1.unwrap()["items"][0]["amount"],
    "PB wave-1 amount should match Central"
  );

  let central_ledger_w1 = get_all_ledger_balances(&client, &central.url, &central.token).await;
  let pa_ledger_w1 = get_all_ledger_balances(&client, &pa.url, &pa.token).await;
  let pb_ledger_w1 = get_all_ledger_balances(&client, &pb.url, &pb.token).await;
  let central_alpha_w1 = filter_by_storage(&central_ledger_w1, &alpha_storage_ids);
  let central_beta_w1 = filter_by_storage(&central_ledger_w1, &beta_storage_ids);
  assert!(
    !central_ledger_w1.is_empty(),
    "Central should have ledger entries after wave 1"
  );
  assert_eq!(
    summarize(&pa_ledger_w1),
    summarize(&central_alpha_w1),
    "PA should mirror Central's alpha-scoped ledger rows after wave 1"
  );
  assert_eq!(
    summarize(&pb_ledger_w1),
    summarize(&central_beta_w1),
    "PB should mirror Central's beta-scoped ledger rows after wave 1"
  );

  let _acc_2 = api_post(
    &client,
    &format!("{}/acceptance/composite/save-and-execute", central.url),
    &central.token,
    serde_json::json!({
      "documentNumber": "ACC-ALPHA-W2",
      "dateAccepted": "2026-01-16T10:00:00Z",
      "arrivalType": "TRUCK",
      "sourceEntity": null,
      "contractorId": catalog.contractor,
      "truckWaybillId": null,
      "railWaybillId": null,
      "transitDispatchId": null,
      "items": [{
        "productId": catalog.product,
        "storageId": catalog.storage_alpha,
        "acceptedAmount": "45.0",
      }]
    }),
  )
  .await;

  await_sync_cycle(&client, &pa.url, &pa.token, Duration::from_secs(15)).await;
  await_sync_cycle(&client, &pb.url, &pb.token, Duration::from_secs(15)).await;

  let pa_ledger_w2 = get_all_ledger_balances(&client, &pa.url, &pa.token).await;
  let central_ledger_w2 = get_all_ledger_balances(&client, &central.url, &central.token).await;
  let central_alpha_w2 = filter_by_storage(&central_ledger_w2, &alpha_storage_ids);
  assert_eq!(
    summarize(&pa_ledger_w2),
    summarize(&central_alpha_w2),
    "PA should mirror Central's alpha-scoped ledger rows after wave 2"
  );

  let pb_ledger_w2 = get_all_ledger_balances(&client, &pb.url, &pb.token).await;
  let central_beta_w2 = filter_by_storage(&central_ledger_w2, &beta_storage_ids);
  assert_eq!(
    summarize(&pb_ledger_w2),
    summarize(&central_beta_w2),
    "PB should still mirror Central's beta-scoped ledger rows after wave 2"
  );
  assert_eq!(
    summarize(&pb_ledger_w2),
    summarize(&pb_ledger_w1),
    "PB beta-scoped ledger rows should remain unchanged after alpha-only wave 2"
  );

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
