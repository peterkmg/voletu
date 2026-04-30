use std::time::Duration;

use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

use crate::common::integration::{
  api_get,
  assert_doc_count_at_least,
  assert_ledger_parity_for_base,
  assert_no_ledger_for_base,
  assert_seed_completeness,
  dev_seed_via_api,
  get_all_ledger_balances,
  poll_until,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
  SyncNodeRef,
};

const SYNC_DEADLINE: Duration = Duration::from_secs(300);

async fn list_all_base_ids(client: &Client, base_url: &str, token: &str) -> Vec<Uuid> {
  let response = api_get(client, &format!("{base_url}/catalog/bases"), token).await;
  response
    .as_array()
    .unwrap_or_else(|| panic!("expected array from /catalog/bases, got: {response}"))
    .iter()
    .filter_map(|b| b["id"].as_str().and_then(|s| Uuid::parse_str(s).ok()))
    .collect()
}

#[tokio::test]
async fn isolates_base_scopes_and_maintains_ledger_parity() {
  let client = Client::new();

  let central = setup_central_via_api(&client, &temp_db_path("c-seed-central")).await;

  let seed_result: Value = dev_seed_via_api(&client, &central.url, &central.token).await;
  let bases_seeded = seed_result["bases"].as_u64().unwrap_or(0);
  assert!(
    bases_seeded >= 3,
    "seed must produce at least 3 bases (got {bases_seeded}): {seed_result}"
  );

  assert_seed_completeness(&client, &central.url, &central.token, "C").await;

  let all_bases = list_all_base_ids(&client, &central.url, &central.token).await;
  assert!(
    all_bases.len() >= 3,
    "central should have >=3 bases, got {all_bases:?}"
  );
  let b1 = all_bases[0];
  let b2 = all_bases[1];
  let b3 = all_bases[2];

  let pa = setup_peripheral_via_api(&client, &temp_db_path("c-seed-pa"), &central, &[b1, b2]).await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("c-seed-pb"), &central, &[b2, b3]).await;

  poll_until(
    || async {
      !get_all_ledger_balances(&client, &pa.url, &pa.token)
        .await
        .is_empty()
    },
    SYNC_DEADLINE,
    "PA pulls some ledger entries from central",
  )
  .await;

  poll_until(
    || async {
      !get_all_ledger_balances(&client, &pb.url, &pb.token)
        .await
        .is_empty()
    },
    SYNC_DEADLINE,
    "PB pulls some ledger entries from central",
  )
  .await;

  assert_doc_count_at_least(
    &client,
    &pa.url,
    &pa.token,
    "PA",
    &central.url,
    &central.token,
    "C",
  )
  .await;
  assert_doc_count_at_least(
    &client,
    &pb.url,
    &pb.token,
    "PB",
    &central.url,
    &central.token,
    "C",
  )
  .await;

  assert_no_ledger_for_base(&client, &pa.url, &pa.token, "PA", b3).await;
  assert_no_ledger_for_base(&client, &pb.url, &pb.token, "PB", b1).await;

  assert_ledger_parity_for_base(
    &client,
    SyncNodeRef {
      url: &central.url,
      token: &central.token,
      label: "C",
    },
    SyncNodeRef {
      url: &pa.url,
      token: &pa.token,
      label: "PA",
    },
    b2,
  )
  .await;
  assert_ledger_parity_for_base(
    &client,
    SyncNodeRef {
      url: &central.url,
      token: &central.token,
      label: "C",
    },
    SyncNodeRef {
      url: &pb.url,
      token: &pb.token,
      label: "PB",
    },
    b2,
  )
  .await;

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
