//! **Peripheral seed full push**: PA seeds via POST /dev/seed with no base assignments,
//! and ALL data must push to Central. A second peripheral PB with a routing-filtered
//! subset must receive only its assigned bases via central distribution.
//!
//! **Topology:** Central + PA (seeds, then assigned [B1, B2]) + PB (assigned [B2, B3])
//! **Verifies:** Push from peripheral to central is complete, routing isolation on PB,
//! and ledger parity on the shared base B2.

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

/// Push+pull of a full seed dataset can span many batches. Keep generous.
const SYNC_DEADLINE: Duration = Duration::from_secs(300);

/// Query catalog bases on a node and return all base ids.
async fn list_all_base_ids(client: &Client, base_url: &str, token: &str) -> Vec<Uuid> {
  let response = api_get(client, &format!("{base_url}/catalog/bases"), token).await;
  response
    .as_array()
    .unwrap_or_else(|| panic!("expected array from /catalog/bases, got: {response}"))
    .iter()
    .filter_map(|b| b["id"].as_str().and_then(|s| Uuid::parse_str(s).ok()))
    .collect()
}

/// Wait until the per-doc-type counts on `target` catch up to `source` for every DocType.
/// Uses repeated API polling — independent of the sync worker's cycle timing.
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

  // 1. Central
  let central = setup_central_via_api(&client, &temp_db_path("pa-seed-central")).await;

  // 2. PA as peripheral with NO base assignments yet — the seed creates bases.
  let pa = setup_peripheral_via_api(&client, &temp_db_path("pa-seed-pa"), &central, &[]).await;

  // 3. Seed PA. dev_seed_via_api wraps api_post which already unwraps the `data`
  // envelope, so the SeedResult fields are at the top level.
  let seed_result: Value = dev_seed_via_api(&client, &pa.url, &pa.token).await;
  let bases_seeded = seed_result["bases"].as_u64().unwrap_or(0);
  assert!(
    bases_seeded >= 3,
    "seed must produce at least 3 bases for routing test (got {bases_seeded}): {seed_result}"
  );

  // 4. Verify PA has all document types locally (catches seed bugs early).
  assert_seed_completeness(&client, &pa.url, &pa.token, "PA").await;

  // 5. Extract bases created by seed.
  let all_bases = list_all_base_ids(&client, &pa.url, &pa.token).await;
  assert!(
    all_bases.len() >= 3,
    "PA should have >=3 bases after seed, got {all_bases:?}"
  );
  let b1 = all_bases[0];
  let b2 = all_bases[1];
  let b3 = all_bases[2];

  // 6. Assign [B1, B2] to PA so pull direction also works.
  add_base_assignment_via_api(&client, &pa.url, &pa.token, b1).await;
  add_base_assignment_via_api(&client, &pa.url, &pa.token, b2).await;

  // 7. Wait until central catches up with PA's full document set via push.
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

  // 8. Central must have at least as many docs of every type as PA (belt-and-braces).
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

  // 9. Start PB with [B2, B3] and wait until it pulls what it should.
  //    (We don't know exact counts up front, but PB's per-type counts should stabilize.)
  let pb =
    setup_peripheral_via_api(&client, &temp_db_path("pa-seed-pb"), &central, &[b2, b3]).await;
  // Give PB time to pull — wait until its ledger is non-empty (some B2 entries must exist
  // since both PA and PB share B2).
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

  // 10. Routing isolation: PB must not have ledger for B1.
  assert_no_ledger_for_base(&client, &pb.url, &pb.token, "PB", b1).await;

  // 11. Ledger parity on shared base B2 between PA and PB.
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

  // 12. Informational: note which doc types are absent on PB.
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
