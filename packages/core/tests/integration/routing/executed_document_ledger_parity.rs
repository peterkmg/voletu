//! **Executed document syncs with ledger parity**: An acceptance document created and
//! executed on Central produces a ledger entry that matches exactly on the peripheral after sync.
//!
//! **Topology:** Central + 1 Peripheral (base_alpha)
//! **Verifies:** Ledger entry (storage + product + contractor + amount) on peripheral matches Central after syncing an executed document

use std::time::Duration;

use crate::common::integration::{
  api_post,
  await_sync_cycle,
  get_all_ledger_entries,
  seed_catalog_via_api,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn executed_document_syncs_with_ledger_parity() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r21-central")).await;
  let catalog = seed_catalog_via_api(&client, &central.url, &central.token).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("r21-pa"), &central, &[
    catalog.base_alpha
  ])
  .await;

  // Create and execute acceptance (creates ledger entry)
  api_post(&client, &format!("{}/acceptance/composite/save-and-execute", central.url), &central.token,
    serde_json::json!({
      "documentNumber": "ACC-LED-001", "dateAccepted": "2026-01-15T10:00:00Z", "arrivalType": "TRUCK",
      "sourceEntity": null, "contractorId": catalog.contractor, "truckWaybillId": null, "railWaybillId": null, "transitDispatchId": null,
      "items": [{"productId": catalog.product, "storageId": catalog.storage_alpha, "acceptedAmount": "1234.56"}]
    })).await;

  // Check ledger on Central
  let central_ledger = get_all_ledger_entries(&client, &central.url, &central.token).await;
  let central_entry = central_ledger.iter().find(|e| {
    e["storageId"].as_str() == Some(&catalog.storage_alpha.to_string())
      && e["productId"].as_str() == Some(&catalog.product.to_string())
      && e["contractorId"].as_str() == Some(&catalog.contractor.to_string())
  });
  assert!(central_entry.is_some(), "Central should have ledger entry");
  let expected_amount = &central_entry.unwrap()["currentAmount"];

  // Sync to PA
  await_sync_cycle(&client, &pa.url, &pa.token, SYNC_TIMEOUT).await;

  // Verify ledger on PA matches
  let pa_ledger = get_all_ledger_entries(&client, &pa.url, &pa.token).await;
  let pa_entry = pa_ledger.iter().find(|e| {
    e["storageId"].as_str() == Some(&catalog.storage_alpha.to_string())
      && e["productId"].as_str() == Some(&catalog.product.to_string())
      && e["contractorId"].as_str() == Some(&catalog.contractor.to_string())
  });
  assert!(pa_entry.is_some(), "PA should have ledger entry after sync");
  assert_eq!(
    &pa_entry.unwrap()["currentAmount"],
    expected_amount,
    "ledger amount should match"
  );

  central.shutdown().await;
  pa.shutdown().await;
}
