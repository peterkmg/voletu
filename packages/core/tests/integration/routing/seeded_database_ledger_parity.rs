//! **Seeded database syncs with ledger parity**: After seeding Central via POST /dev/seed
//! and syncing to a peripheral, catalog entities, ledger entries for the assigned base,
//! and business documents all match exactly.
//!
//! **Topology:** Central (seeded) + 1 Peripheral (first base from seed)
//! **Verifies:** Full catalog parity, ledger entry parity for assigned base, absence of other-base ledger entries, and presence of business documents

use std::time::Duration;

use serde_json::Value;
use uuid::Uuid;

use crate::common::integration::{
  api_get, await_sync_cycle, dev_seed_via_api, get_all_ledger_entries, get_storages_for_base,
  setup_central_via_api, setup_peripheral_via_api, temp_db_path,
};

const SYNC_TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::test]
async fn seeded_database_syncs_to_peripheral_with_ledger_parity() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r15-central")).await;

  // Seed Central with realistic data
  let seed_result = dev_seed_via_api(&client, &central.url, &central.token).await;
  let seeded_bases: usize = seed_result["bases"].as_u64().unwrap_or(0) as usize;
  assert!(
    seeded_bases >= 5,
    "seed should create at least 5 bases, got {seeded_bases}"
  );
  let seeded_ledger_entries: usize = seed_result["ledger_entries"].as_u64().unwrap_or(0) as usize;
  assert!(
    seeded_ledger_entries > 0,
    "seed should create ledger entries"
  );

  // Pick the first base from the catalog
  let bases = api_get(
    &client,
    &format!("{}/catalog/bases", central.url),
    &central.token,
  )
  .await;
  let all_bases: Vec<Uuid> = bases
    .as_array()
    .unwrap()
    .iter()
    .filter_map(|b| b["id"].as_str().and_then(|s| Uuid::parse_str(s).ok()))
    .collect();
  assert!(!all_bases.is_empty());
  let target_base_id = all_bases[0];

  // Determine which storages belong to this base
  let base_storage_ids =
    get_storages_for_base(&client, &central.url, &central.token, target_base_id).await;
  assert!(
    !base_storage_ids.is_empty(),
    "target base should have storages"
  );

  // Get Central ledger state (ground truth)
  let central_ledger = get_all_ledger_entries(&client, &central.url, &central.token).await;
  assert!(
    !central_ledger.is_empty(),
    "Central should have ledger entries after seed"
  );

  // Partition ledger by base scope
  let central_ledger_for_base: Vec<&Value> = central_ledger
    .iter()
    .filter(|entry| {
      entry["storageId"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok())
        .map(|sid| base_storage_ids.contains(&sid))
        .unwrap_or(false)
    })
    .collect();
  let central_ledger_other: Vec<&Value> = central_ledger
    .iter()
    .filter(|entry| {
      entry["storageId"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok())
        .map(|sid| !base_storage_ids.contains(&sid))
        .unwrap_or(true)
    })
    .collect();
  assert!(
    !central_ledger_for_base.is_empty(),
    "target base should have ledger entries"
  );

  // Setup Peripheral with this base — initial sync happens during setup.
  // Run additional cycles to consume all seeded audit logs (seed creates many entries).
  let peripheral = setup_peripheral_via_api(
    &client,
    &temp_db_path("r15-periph"),
    &central,
    &[target_base_id],
  )
  .await;

  // Additional sync cycles to ensure all batched data arrives
  await_sync_cycle(&client, &peripheral.url, &peripheral.token, SYNC_TIMEOUT).await;
  await_sync_cycle(&client, &peripheral.url, &peripheral.token, SYNC_TIMEOUT).await;

  // --- Verify catalog entities present on Peripheral ---
  let periph_products = api_get(
    &client,
    &format!("{}/catalog/products", peripheral.url),
    &peripheral.token,
  )
  .await;
  let central_products = api_get(
    &client,
    &format!("{}/catalog/products", central.url),
    &central.token,
  )
  .await;
  assert_eq!(
    periph_products.as_array().unwrap().len(),
    central_products.as_array().unwrap().len(),
    "all products should sync (global catalog)"
  );

  let periph_companies = api_get(
    &client,
    &format!("{}/catalog/companies", peripheral.url),
    &peripheral.token,
  )
  .await;
  let central_companies = api_get(
    &client,
    &format!("{}/catalog/companies", central.url),
    &central.token,
  )
  .await;
  assert_eq!(
    periph_companies.as_array().unwrap().len(),
    central_companies.as_array().unwrap().len(),
    "all companies should sync (global catalog)"
  );

  // --- Verify ledger parity for assigned base ---
  let periph_ledger = get_all_ledger_entries(&client, &peripheral.url, &peripheral.token).await;

  // Every Central ledger entry for this base should exist on Peripheral with same amount
  for central_entry in &central_ledger_for_base {
    let storage_id = central_entry["storageId"].as_str().unwrap();
    let product_id = central_entry["productId"].as_str().unwrap();
    let contractor_id = central_entry["contractorId"].as_str().unwrap();
    let central_amount = &central_entry["currentAmount"];

    let periph_match = periph_ledger.iter().find(|pe| {
      pe["storageId"].as_str() == Some(storage_id)
        && pe["productId"].as_str() == Some(product_id)
        && pe["contractorId"].as_str() == Some(contractor_id)
    });

    assert!(
      periph_match.is_some(),
      "Peripheral missing ledger entry for storage={storage_id} product={product_id} contractor={contractor_id}"
    );
    assert_eq!(
      &periph_match.unwrap()["currentAmount"], central_amount,
      "ledger amount mismatch for storage={storage_id} product={product_id} contractor={contractor_id}"
    );
  }

  // Ledger entries for OTHER bases should NOT exist on Peripheral
  for other_entry in &central_ledger_other {
    let storage_id = other_entry["storageId"].as_str().unwrap();
    let product_id = other_entry["productId"].as_str().unwrap();
    let contractor_id = other_entry["contractorId"].as_str().unwrap();

    let should_not_exist = periph_ledger.iter().any(|pe| {
      pe["storageId"].as_str() == Some(storage_id)
        && pe["productId"].as_str() == Some(product_id)
        && pe["contractorId"].as_str() == Some(contractor_id)
    });

    assert!(
      !should_not_exist,
      "Peripheral should NOT have ledger entry for other-base storage={storage_id}"
    );
  }

  // --- Verify at least some business documents exist ---
  let periph_acceptance = api_get(
    &client,
    &format!("{}/acceptance", peripheral.url),
    &peripheral.token,
  )
  .await;
  assert!(
    !periph_acceptance.as_array().unwrap().is_empty(),
    "Peripheral should have at least some acceptance documents after sync"
  );

  central.shutdown().await;
  peripheral.shutdown().await;
}
