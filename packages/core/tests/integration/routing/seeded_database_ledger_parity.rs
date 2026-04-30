use std::time::Duration;

use uuid::Uuid;

use crate::common::integration::{
  api_get,
  assert_doc_count_equal,
  assert_ledger_parity_for_base,
  dev_seed_via_api,
  doc_count,
  get_all_ledger_balances,
  get_storages_for_base,
  poll_until,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
  DocType,
  SyncNodeRef,
};

const SYNC_DEADLINE: Duration = Duration::from_secs(300);

#[tokio::test]
async fn peripheral_matches_central_catalog_ledger_and_documents_after_dev_seed() {
  let client = reqwest::Client::new();
  let central = setup_central_via_api(&client, &temp_db_path("r15-central")).await;

  let seed_result = dev_seed_via_api(&client, &central.url, &central.token).await;
  let seeded_bases: usize = seed_result["bases"].as_u64().unwrap_or(0) as usize;
  assert!(
    seeded_bases >= 2,
    "seed should create at least 2 bases for routing coverage, got {seeded_bases}"
  );
  let seeded_ledger_entries: usize = seed_result["ledgerEntries"].as_u64().unwrap_or(0) as usize;
  assert!(
    seeded_ledger_entries > 0,
    "seed should create ledger entries"
  );

  let bases_response = api_get(
    &client,
    &format!("{}/catalog/bases", central.url),
    &central.token,
  )
  .await;
  let all_bases: Vec<Uuid> = bases_response
    .as_array()
    .unwrap()
    .iter()
    .filter_map(|b| b["id"].as_str().and_then(|s| Uuid::parse_str(s).ok()))
    .collect();
  assert_eq!(
    all_bases.len(),
    seeded_bases,
    "base count from /catalog/bases should match SeedResult.bases"
  );

  let peripheral =
    setup_peripheral_via_api(&client, &temp_db_path("r15-periph"), &central, &all_bases).await;

  poll_until(
    || async {
      for doc_type in DocType::all() {
        let central_count = doc_count(&client, &central.url, &central.token, *doc_type).await;
        let periph_count = doc_count(&client, &peripheral.url, &peripheral.token, *doc_type).await;
        if periph_count < central_count {
          return false;
        }
      }

      let central_ledger_len = get_all_ledger_balances(&client, &central.url, &central.token)
        .await
        .len();
      let periph_ledger_len = get_all_ledger_balances(&client, &peripheral.url, &peripheral.token)
        .await
        .len();
      periph_ledger_len >= central_ledger_len
    },
    SYNC_DEADLINE,
    "peripheral catches up to central (per-doc-type + ledger counts)",
  )
  .await;

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

  assert_doc_count_equal(
    &client,
    &central.url,
    &central.token,
    "C",
    &peripheral.url,
    &peripheral.token,
    "P",
  )
  .await;

  for base_id in &all_bases {
    let storages = get_storages_for_base(&client, &central.url, &central.token, *base_id).await;
    if storages.is_empty() {
      continue;
    }
    assert_ledger_parity_for_base(
      &client,
      SyncNodeRef {
        url: &central.url,
        token: &central.token,
        label: "C",
      },
      SyncNodeRef {
        url: &peripheral.url,
        token: &peripheral.token,
        label: "P",
      },
      *base_id,
    )
    .await;
  }

  let central_ledger = get_all_ledger_balances(&client, &central.url, &central.token).await;
  let periph_ledger = get_all_ledger_balances(&client, &peripheral.url, &peripheral.token).await;
  assert_eq!(
    periph_ledger.len(),
    central_ledger.len(),
    "peripheral ledger row count must match central when all bases are assigned"
  );

  central.shutdown().await;
  peripheral.shutdown().await;
}
