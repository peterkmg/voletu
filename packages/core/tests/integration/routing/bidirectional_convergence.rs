//! **Bidirectional convergence**: both peripherals seed independently. All
//! three nodes must converge to the correct routed subsets, with the overlap
//! base receiving data from BOTH peripherals.
//!
//! **Topology:** Central + PA (seeds, then assigned [b_a1, b_a2]) + PB (seeds,
//! then assigned [b_b_own, b_a2] for overlap).
//! **Verifies:** Central holds all docs from both peripherals. Ledger parity
//! on the overlap base b_a2 holds across all three nodes.

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
  get_all_ledger_entries,
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

  // Setup: central + PA with no bases + PB with no bases.
  let central = setup_central_via_api(&client, &temp_db_path("bi-central")).await;
  let pa = setup_peripheral_via_api(&client, &temp_db_path("bi-pa"), &central, &[]).await;
  let pb = setup_peripheral_via_api(&client, &temp_db_path("bi-pb"), &central, &[]).await;

  // 1. Seed PA and let it push to central.
  let seed_a: Value = dev_seed_via_api(&client, &pa.url, &pa.token).await;
  let bases_a = seed_a["bases"].as_u64().unwrap_or(0);
  assert!(
    bases_a >= 3,
    "PA seed must produce >=3 bases (got {bases_a})"
  );
  assert_seed_completeness(&client, &pa.url, &pa.token, "PA").await;

  let pa_bases = list_all_base_ids(&client, &pa.url, &pa.token).await;
  let b_a1 = pa_bases[0];
  let b_a2 = pa_bases[1]; // overlap base

  add_base_assignment_via_api(&client, &pa.url, &pa.token, b_a1).await;
  add_base_assignment_via_api(&client, &pa.url, &pa.token, b_a2).await;

  // Wait for central to have as many docs as PA (PA's full push completed).
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

  // 2. Seed PB (fresh data, but PB has already pulled PA's bases as catalog).
  let seed_b: Value = dev_seed_via_api(&client, &pb.url, &pb.token).await;
  let bases_b = seed_b["bases"].as_u64().unwrap_or(0);
  assert!(
    bases_b >= 3,
    "PB seed must produce >=3 bases (got {bases_b})"
  );
  assert_seed_completeness(&client, &pb.url, &pb.token, "PB").await;

  // PB's base list contains PA's bases (synced from central) + PB's own seeded bases.
  let pb_bases = list_all_base_ids(&client, &pb.url, &pb.token).await;
  let pa_set: HashSet<Uuid> = pa_bases.iter().copied().collect();
  let b_b_own = pb_bases
    .into_iter()
    .find(|b| !pa_set.contains(b))
    .expect("PB should have at least one of its own seeded bases not in PA's set");

  // Assign PB to its own base + overlap with PA's b_a2.
  add_base_assignment_via_api(&client, &pb.url, &pb.token, b_b_own).await;
  add_base_assignment_via_api(&client, &pb.url, &pb.token, b_a2).await;

  // 3. Wait for central to catch up to PB.
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

  // 4. Wait for PA to pull PB's overlap-base data.
  poll_until(
    || async {
      // PA should see ledger entries for b_a2 from both PA's own data and PB's overlap.
      !get_all_ledger_entries(&client, &pa.url, &pa.token)
        .await
        .is_empty()
    },
    SYNC_DEADLINE,
    "PA has ledger after pull",
  )
  .await;

  // 5. Final assertions: central has >= both peripherals (it's the superset).
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

  // 6. Ledger parity on overlap base b_a2: PA, PB, and central agree.
  //    All three nodes should see identical ledger entries for storages under b_a2.
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
