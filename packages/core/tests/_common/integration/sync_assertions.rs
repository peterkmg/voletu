use std::collections::HashMap;

use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

use super::{
  api_client::api_get,
  verification::{get_all_ledger_balances, get_storages_for_base},
};

#[derive(Debug, Clone, Copy)]
pub struct SyncNodeRef<'a> {
  pub url: &'a str,
  pub token: &'a str,
  pub label: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocType {
  Acceptance,
  Dispatch,
  Blending,
  PhysicalTransfer,
  OwnershipTransfer,
  Reconciliation,
  TruckWaybill,
  RailWaybill,
}

impl DocType {
  pub fn all() -> &'static [DocType] {
    &[
      DocType::Acceptance,
      DocType::Dispatch,
      DocType::Blending,
      DocType::PhysicalTransfer,
      DocType::OwnershipTransfer,
      DocType::Reconciliation,
      DocType::TruckWaybill,
      DocType::RailWaybill,
    ]
  }

  pub fn list_endpoint(self) -> &'static str {
    match self {
      DocType::Acceptance => "/acceptance",
      DocType::Dispatch => "/dispatch",
      DocType::Blending => "/blending",
      DocType::PhysicalTransfer => "/physical-transfers",
      DocType::OwnershipTransfer => "/ownership-transfers",
      DocType::Reconciliation => "/reconciliations",
      DocType::TruckWaybill => "/transport/truck/waybills",
      DocType::RailWaybill => "/transport/rail/waybills",
    }
  }

  pub fn label(self) -> &'static str {
    match self {
      DocType::Acceptance => "acceptance",
      DocType::Dispatch => "dispatch",
      DocType::Blending => "blending",
      DocType::PhysicalTransfer => "physical_transfer",
      DocType::OwnershipTransfer => "ownership_transfer",
      DocType::Reconciliation => "reconciliation",
      DocType::TruckWaybill => "truck_waybill",
      DocType::RailWaybill => "rail_waybill",
    }
  }
}

pub async fn doc_count(client: &Client, base_url: &str, token: &str, doc_type: DocType) -> usize {
  let response = api_get(
    client,
    &format!("{base_url}{}", doc_type.list_endpoint()),
    token,
  )
  .await;
  response
    .as_array()
    .unwrap_or_else(|| {
      panic!(
        "expected array from {}, got: {response}",
        doc_type.list_endpoint()
      )
    })
    .len()
}

pub async fn doc_list(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_type: DocType,
) -> Vec<Value> {
  let response = api_get(
    client,
    &format!("{base_url}{}", doc_type.list_endpoint()),
    token,
  )
  .await;
  response
    .as_array()
    .unwrap_or_else(|| {
      panic!(
        "expected array from {}, got: {response}",
        doc_type.list_endpoint()
      )
    })
    .clone()
}

pub async fn doc_counts(client: &Client, base_url: &str, token: &str) -> HashMap<DocType, usize> {
  let mut out = HashMap::new();
  for doc_type in DocType::all() {
    out.insert(
      *doc_type,
      doc_count(client, base_url, token, *doc_type).await,
    );
  }
  out
}

pub async fn assert_seed_completeness(
  client: &Client,
  base_url: &str,
  token: &str,
  node_label: &str,
) {
  let counts = doc_counts(client, base_url, token).await;
  let mut missing: Vec<&'static str> = Vec::new();
  for doc_type in DocType::all() {
    let count = counts.get(doc_type).copied().unwrap_or(0);
    if count == 0 {
      missing.push(doc_type.label());
    }
  }
  assert!(
    missing.is_empty(),
    "node {node_label} missing seeded document types: {missing:?} (counts: {counts:?})"
  );

  let ledger = get_all_ledger_balances(client, base_url, token).await;
  assert!(
    !ledger.is_empty(),
    "node {node_label} has no ledger entries after seed"
  );
}

pub async fn assert_doc_count_at_least(
  client: &Client,
  url_a: &str,
  token_a: &str,
  label_a: &str,
  url_b: &str,
  token_b: &str,
  label_b: &str,
) {
  let counts_a = doc_counts(client, url_a, token_a).await;
  let counts_b = doc_counts(client, url_b, token_b).await;
  let mut shortfalls: Vec<String> = Vec::new();
  for doc_type in DocType::all() {
    let a = counts_a.get(doc_type).copied().unwrap_or(0);
    let b = counts_b.get(doc_type).copied().unwrap_or(0);
    if b < a {
      shortfalls.push(format!(
        "{}: {label_a}={a} > {label_b}={b}",
        doc_type.label()
      ));
    }
  }
  assert!(
    shortfalls.is_empty(),
    "doc count parity failed ({label_b} missing data from {label_a}): {shortfalls:?}"
  );
}

pub async fn assert_doc_count_equal(
  client: &Client,
  url_a: &str,
  token_a: &str,
  label_a: &str,
  url_b: &str,
  token_b: &str,
  label_b: &str,
) {
  let counts_a = doc_counts(client, url_a, token_a).await;
  let counts_b = doc_counts(client, url_b, token_b).await;
  let mut mismatches: Vec<String> = Vec::new();
  for doc_type in DocType::all() {
    let a = counts_a.get(doc_type).copied().unwrap_or(0);
    let b = counts_b.get(doc_type).copied().unwrap_or(0);
    if a != b {
      mismatches.push(format!(
        "{}: {label_a}={a} vs {label_b}={b}",
        doc_type.label()
      ));
    }
  }
  assert!(
    mismatches.is_empty(),
    "doc count mismatch between {label_a} and {label_b}: {mismatches:?}"
  );
}

pub async fn assert_ledger_parity_for_base(
  client: &Client,
  node_a: SyncNodeRef<'_>,
  node_b: SyncNodeRef<'_>,
  base_id: Uuid,
) {
  let storages_a = get_storages_for_base(client, node_a.url, node_a.token, base_id).await;
  let storages_b = get_storages_for_base(client, node_b.url, node_b.token, base_id).await;

  let mut sa = storages_a.clone();
  let mut sb = storages_b.clone();
  sa.sort();
  sb.sort();
  assert_eq!(
    sa, sb,
    "{} and {} disagree on storages for base {base_id}",
    node_a.label, node_b.label
  );

  let entries_a = get_all_ledger_balances(client, node_a.url, node_a.token).await;
  let entries_b = get_all_ledger_balances(client, node_b.url, node_b.token).await;

  let storage_set: std::collections::HashSet<Uuid> = storages_a.into_iter().collect();

  let filter = |entries: &[Value]| -> Vec<(Uuid, Uuid, Uuid, String)> {
    entries
      .iter()
      .filter_map(|e| {
        let storage_id = e["storageId"]
          .as_str()
          .and_then(|s| Uuid::parse_str(s).ok())?;
        if !storage_set.contains(&storage_id) {
          return None;
        }
        let product_id = e["productId"]
          .as_str()
          .and_then(|s| Uuid::parse_str(s).ok())
          .unwrap_or_else(|| panic!("ledger entry missing productId: {e}"));
        let contractor_id = e["contractorId"]
          .as_str()
          .and_then(|s| Uuid::parse_str(s).ok())
          .unwrap_or_else(|| panic!("ledger entry missing contractorId: {e}"));

        let amount = e["currentAmount"].to_string();
        Some((storage_id, product_id, contractor_id, amount))
      })
      .collect()
  };

  let mut filtered_a = filter(&entries_a);
  let mut filtered_b = filter(&entries_b);
  filtered_a.sort();
  filtered_b.sort();

  assert_eq!(
    filtered_a, filtered_b,
    "ledger entries differ for base {base_id} between {} and {}",
    node_a.label, node_b.label
  );
}

pub async fn assert_no_ledger_for_base(
  client: &Client,
  base_url: &str,
  token: &str,
  node_label: &str,
  forbidden_base_id: Uuid,
) {
  let storages = get_storages_for_base(client, base_url, token, forbidden_base_id).await;
  let storage_set: std::collections::HashSet<Uuid> = storages.into_iter().collect();
  let entries = get_all_ledger_balances(client, base_url, token).await;

  let violations: Vec<Value> = entries
    .into_iter()
    .filter(|e| {
      e["storageId"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok())
        .map(|id| storage_set.contains(&id))
        .unwrap_or(false)
    })
    .collect();

  assert!(
    violations.is_empty(),
    "node {node_label} has {} ledger entries for forbidden base {forbidden_base_id}: {violations:?}",
    violations.len()
  );
}
