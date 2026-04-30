use std::{collections::HashSet, time::Duration};

use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

use crate::common::integration::{
  add_base_assignment_via_api,
  api_get,
  assert_doc_count_at_least,
  assert_ledger_parity_for_base,
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
async fn central_accumulates_docs_from_both_peripherals_with_ledger_parity() {
  let client = Client::new();

  let central = setup_central_via_api(&client, &temp_db_path("bi-central")).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("bi-pa"), &central, &[]).await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("bi-pb"), &central, &[]).await;

  let seed_a: Value = dev_seed_via_api(&client, &pa.url, &pa.token).await;
  let bases_a = seed_a["bases"].as_u64().unwrap_or(0);
  assert!(
    bases_a >= 3,
    "PA seed must produce >=3 bases (got {bases_a})"
  );
  assert_seed_completeness(&client, &pa.url, &pa.token, "PA").await;

  let pa_bases = list_all_base_ids(&client, &pa.url, &pa.token).await;
  let b_a1 = pa_bases[0];
  let b_a2 = pa_bases[1];

  add_base_assignment_via_api(&client, &pa.url, &pa.token, b_a1).await;
  add_base_assignment_via_api(&client, &pa.url, &pa.token, b_a2).await;

  poll_until(
    || async {
      use crate::common::integration::{doc_count, DocType};
      for doc_type in DocType::all() {
        let pa_c = doc_count(&client, &pa.url, &pa.token, *doc_type).await;
        let c_c = doc_count(&client, &central.url, &central.token, *doc_type).await;
        if c_c < pa_c {
          return false;
        }
      }
      true
    },
    SYNC_DEADLINE,
    "central receives PA's full push",
  )
  .await;

  let seed_b: Value = dev_seed_via_api(&client, &pb.url, &pb.token).await;
  let bases_b = seed_b["bases"].as_u64().unwrap_or(0);
  assert!(
    bases_b >= 3,
    "PB seed must produce >=3 bases (got {bases_b})"
  );
  assert_seed_completeness(&client, &pb.url, &pb.token, "PB").await;

  let pb_bases = list_all_base_ids(&client, &pb.url, &pb.token).await;
  let pa_set: HashSet<Uuid> = pa_bases.iter().copied().collect();
  let b_b_own = pb_bases
    .into_iter()
    .find(|b| !pa_set.contains(b))
    .expect("PB should have at least one of its own seeded bases not in PA's set");

  add_base_assignment_via_api(&client, &pb.url, &pb.token, b_b_own).await;
  add_base_assignment_via_api(&client, &pb.url, &pb.token, b_a2).await;

  poll_until(
    || async {
      use crate::common::integration::{doc_count, DocType};
      for doc_type in DocType::all() {
        let pb_c = doc_count(&client, &pb.url, &pb.token, *doc_type).await;
        let c_c = doc_count(&client, &central.url, &central.token, *doc_type).await;
        if c_c < pb_c {
          return false;
        }
      }
      true
    },
    SYNC_DEADLINE,
    "central receives PB's full push",
  )
  .await;

  poll_until(
    || async {
      !get_all_ledger_balances(&client, &pa.url, &pa.token)
        .await
        .is_empty()
    },
    SYNC_DEADLINE,
    "PA has ledger after pull",
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

  assert_ledger_parity_for_base(
    &client,
    SyncNodeRef {
      url: &pa.url,
      token: &pa.token,
      label: "PA",
    },
    SyncNodeRef {
      url: &central.url,
      token: &central.token,
      label: "C",
    },
    b_a2,
  )
  .await;
  assert_ledger_parity_for_base(
    &client,
    SyncNodeRef {
      url: &pb.url,
      token: &pb.token,
      label: "PB",
    },
    SyncNodeRef {
      url: &central.url,
      token: &central.token,
      label: "C",
    },
    b_a2,
  )
  .await;

  central.shutdown().await;
  pa.shutdown().await;
  pb.shutdown().await;
}
