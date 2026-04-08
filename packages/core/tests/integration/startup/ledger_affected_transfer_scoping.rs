//! Verifies that ledger-affected documents propagate with correct scope
//! narrowing across a three-node topology using the real sync worker.
//!
//! Wave 1 creates a cross-base physical transfer (routes to both bases) and
//! executes it, so all three nodes converge on the document and ledger entries.
//! Wave 2 creates a single-base acceptance (routes to base_alpha only) and
//! executes it, so only Central and PA receive it while PB's ledger stays at
//! the prior value.
//!
//! Topology: Central + 2 Peripherals (PA on base_alpha, PB on base_beta).
//!
//! Property: shared-scope executed documents and their ledger entries reach all
//! nodes; local-scope executed documents and their ledger entries reach only
//! Central and the relevant Peripheral.

use std::time::Duration;

use crate::common::integration::{
  api_post, await_sync_cycle, get_all_ledger_entries, get_physical_transfer_composite_json,
  seed_catalog_via_api, setup_central_via_api, setup_peripheral_via_api, temp_db_path,
};

use super::parse_doc_id;

#[tokio::test]
async fn ledger_affected_transfer_targets_shared_then_local_scope_via_worker() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("s5-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("s5-pa"), &central, &[
    catalog.base_alpha,
  ])
  .await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("s5-pb"), &central, &[
    catalog.base_beta,
  ])
  .await;

  // ── Wave 1: cross-base physical transfer (routes to BOTH bases) ──

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

  // Let both peripherals sync wave 1
  await_sync_cycle(&client, &pa.url, &pa.token, Duration::from_secs(15)).await;
  await_sync_cycle(&client, &pb.url, &pb.token, Duration::from_secs(15)).await;

  // Both peripherals should have the transfer
  let pa_t1 =
    get_physical_transfer_composite_json(&client, &pa.url, &pa.token, transfer_1_id).await;
  let pb_t1 =
    get_physical_transfer_composite_json(&client, &pb.url, &pb.token, transfer_1_id).await;
  assert!(pa_t1.is_some(), "PA should have wave-1 transfer");
  assert!(pb_t1.is_some(), "PB should have wave-1 transfer");

  // Field parity on the transfer
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

  // Ledger: both peripherals have entries (physical transfer creates ledger rows on execute)
  let central_ledger_w1 = get_all_ledger_entries(&client, &central.url, &central.token).await;
  let pa_ledger_w1 = get_all_ledger_entries(&client, &pa.url, &pa.token).await;
  let pb_ledger_w1 = get_all_ledger_entries(&client, &pb.url, &pb.token).await;
  assert!(
    !central_ledger_w1.is_empty(),
    "Central should have ledger entries after wave 1"
  );
  assert_eq!(
    pa_ledger_w1.len(),
    central_ledger_w1.len(),
    "PA ledger count should match Central after wave 1"
  );
  assert_eq!(
    pb_ledger_w1.len(),
    central_ledger_w1.len(),
    "PB ledger count should match Central after wave 1"
  );

  // ── Wave 2: single-base acceptance (routes to base_alpha ONLY) ──

  let _acc_2 = api_post(
    &client,
    &format!(
      "{}/acceptance/composite/save-and-execute",
      central.url
    ),
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

  // Let both peripherals sync wave 2
  await_sync_cycle(&client, &pa.url, &pa.token, Duration::from_secs(15)).await;
  await_sync_cycle(&client, &pb.url, &pb.token, Duration::from_secs(15)).await;

  // PA should have gained a new ledger entry (the acceptance for storage_alpha)
  let pa_ledger_w2 = get_all_ledger_entries(&client, &pa.url, &pa.token).await;
  let central_ledger_w2 = get_all_ledger_entries(&client, &central.url, &central.token).await;
  assert_eq!(
    pa_ledger_w2.len(),
    central_ledger_w2.len(),
    "PA ledger count should still match Central after wave 2"
  );

  // PB should NOT have the new ledger entry — its count stays at wave 1 level
  let pb_ledger_w2 = get_all_ledger_entries(&client, &pb.url, &pb.token).await;
  assert_eq!(
    pb_ledger_w2.len(),
    pb_ledger_w1.len(),
    "PB ledger count should remain unchanged after wave 2 (alpha-only acceptance)"
  );

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
