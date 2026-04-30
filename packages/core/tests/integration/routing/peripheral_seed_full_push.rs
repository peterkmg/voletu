use std::time::Duration;

use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

use crate::common::integration::{
  add_base_assignment_via_api,
  api_get,
  assert_doc_count_at_least,
  assert_ledger_parity_for_base,
  assert_no_ledger_for_base,
  assert_seed_completeness,
  dev_seed_via_api,
  doc_count,
  doc_counts,
  poll_until,
  setup_central_via_api,
  setup_peripheral_via_api,
  temp_db_path,
  DocType,
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

async fn wait_for_doc_count_parity(
  client: &Client,
  source_url: &str,
  source_token: &str,
  source_label: &str,
  target_url: &str,
  target_token: &str,
  target_label: &str,
) {
  let label = format!("{source_label} -> {target_label} doc count parity");
  poll_until(
    || async {
      for doc_type in DocType::all() {
        let src = doc_count(client, source_url, source_token, *doc_type).await;
        let tgt = doc_count(client, target_url, target_token, *doc_type).await;
        if tgt < src {
          return false;
        }
      }
      true
    },
    SYNC_DEADLINE,
    &label,
  )
  .await;
}

#[tokio::test]
async fn isolates_unshared_base_and_maintains_ledger_parity_on_shared() {
  let client = Client::new();

  let central = setup_central_via_api(&client, &temp_db_path("pa-seed-central")).await;

  let pa = setup_peripheral_via_api(&client, &temp_db_path("pa-seed-pa"), &central, &[]).await;

  let seed_result: Value = dev_seed_via_api(&client, &pa.url, &pa.token).await;
  let bases_seeded = seed_result["bases"].as_u64().unwrap_or(0);
  assert!(
    bases_seeded >= 3,
    "seed must produce at least 3 bases for routing test (got {bases_seeded}): {seed_result}"
  );

  assert_seed_completeness(&client, &pa.url, &pa.token, "PA").await;

  let all_bases = list_all_base_ids(&client, &pa.url, &pa.token).await;
  assert!(
    all_bases.len() >= 3,
    "PA should have >=3 bases after seed, got {all_bases:?}"
  );
  let b1 = all_bases[0];
  let b2 = all_bases[1];
  let b3 = all_bases[2];

  add_base_assignment_via_api(&client, &pa.url, &pa.token, b1).await;
  add_base_assignment_via_api(&client, &pa.url, &pa.token, b2).await;

  wait_for_doc_count_parity(
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
    &pa.url,
    &pa.token,
    "PA",
    &central.url,
    &central.token,
    "C",
  )
  .await;

  let pb =
    setup_peripheral_via_api(&client, &temp_db_path("pa-seed-pb"), &central, &[b2, b3]).await;

  poll_until(
    || async {
      let entries =
        crate::common::integration::get_all_ledger_balances(&client, &pb.url, &pb.token).await;
      !entries.is_empty()
    },
    SYNC_DEADLINE,
    "PB pulls some ledger entries from central",
  )
  .await;

  assert_no_ledger_for_base(&client, &pb.url, &pb.token, "PB", b1).await;

  assert_ledger_parity_for_base(
    &client,
    SyncNodeRef {
      url: &pa.url,
      token: &pa.token,
      label: "PA",
    },
    SyncNodeRef {
      url: &pb.url,
      token: &pb.token,
      label: "PB",
    },
    b2,
  )
  .await;

  let pb_counts = doc_counts(&client, &pb.url, &pb.token).await;
  let zero_types: Vec<&'static str> = pb_counts
    .iter()
    .filter(|(_, &c)| c == 0)
    .map(|(t, _)| t.label())
    .collect();
  eprintln!("PB doc types with zero count (informational): {zero_types:?}");

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
