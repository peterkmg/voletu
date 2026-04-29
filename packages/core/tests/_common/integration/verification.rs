use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use uuid::Uuid;

use super::{api_get, api_post};

/// Query audit logs, optionally filtered by table name and/or record ID (client-side filter).
pub async fn query_audit_logs(
  client: &Client,
  base_url: &str,
  token: &str,
  table_name: Option<&str>,
  record_id: Option<Uuid>,
) -> Vec<Value> {
  let data = api_get(client, &format!("{base_url}/audit-logs"), token).await;
  let all = data.as_array().cloned().unwrap_or_default();
  all
    .into_iter()
    .filter(|log| {
      if let Some(tn) = table_name {
        if log["tableName"] != tn {
          return false;
        }
      }
      if let Some(rid) = record_id {
        if log["recordId"] != rid.to_string() {
          return false;
        }
      }
      true
    })
    .collect()
}

/// Find audit logs for a specific record and assert target_base_ids contains the expected base.
pub fn assert_audit_log_targets(
  logs: &[Value],
  table_name: &str,
  record_id: Uuid,
  expected_base_id: Uuid,
) {
  let matching: Vec<&Value> = logs
    .iter()
    .filter(|l| l["tableName"] == table_name && l["recordId"] == record_id.to_string())
    .collect();

  assert!(
    !matching.is_empty(),
    "expected audit log for {table_name}/{record_id}, found none"
  );

  let base_str = expected_base_id.to_string();
  for log in &matching {
    let target = log["targetBaseIds"].as_str().unwrap_or("");
    assert!(
      target.contains(&base_str),
      "audit log for {table_name}/{record_id} has target_base_ids='{}', expected to contain '{}'",
      target,
      base_str,
    );
  }
}

pub async fn has_audit_record(
  client: &Client,
  node_url: &str,
  token: &str,
  table_name: &str,
  record_id: Uuid,
) -> bool {
  let logs = api_get(client, &format!("{node_url}/audit-logs"), token).await;
  logs
    .as_array()
    .unwrap()
    .iter()
    .any(|log| log["tableName"] == table_name && log["recordId"] == record_id.to_string())
}

/// Check that a catalog entity exists on a node by querying the list endpoint.
pub async fn has_catalog_entity(
  client: &Client,
  base_url: &str,
  token: &str,
  endpoint: &str,
  entity_id: Uuid,
) -> bool {
  let data = api_get(client, &format!("{base_url}{endpoint}"), token).await;
  data
    .as_array()
    .unwrap()
    .iter()
    .any(|e| e["id"] == entity_id.to_string())
}

/// Retrieve an acceptance composite document by ID.
pub async fn get_acceptance_composite_json(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_id: Uuid,
) -> Option<Value> {
  let response = client
    .get(format!("{base_url}/acceptance/composite/{doc_id}"))
    .bearer_auth(token)
    .send()
    .await
    .unwrap();
  if response.status() == StatusCode::NOT_FOUND {
    return None;
  }
  assert_eq!(response.status(), StatusCode::OK);
  let body: Value = response.json().await.unwrap();
  assert_eq!(body["success"], Value::Bool(true));
  Some(body["data"].clone())
}

/// Retrieve a physical transfer composite document by ID.
pub async fn get_physical_transfer_composite_json(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_id: Uuid,
) -> Option<Value> {
  let response = client
    .get(format!("{base_url}/physical-transfers/composite/{doc_id}"))
    .bearer_auth(token)
    .send()
    .await
    .unwrap();
  if response.status() == StatusCode::NOT_FOUND {
    return None;
  }
  assert_eq!(response.status(), StatusCode::OK);
  let body: Value = response.json().await.unwrap();
  assert_eq!(body["success"], Value::Bool(true));
  Some(body["data"].clone())
}

/// Retrieve a generic composite document by ID. Returns None if 404.
pub async fn get_composite_json(
  client: &Client,
  base_url: &str,
  token: &str,
  path_template: &str,
  doc_id: Uuid,
) -> Option<Value> {
  let url = format!(
    "{base_url}{}",
    path_template.replace("{id}", &doc_id.to_string())
  );
  let response = client.get(&url).bearer_auth(token).send().await.unwrap();
  if response.status() == StatusCode::NOT_FOUND {
    return None;
  }
  assert_eq!(response.status(), StatusCode::OK);
  let body: Value = response.json().await.unwrap();
  assert_eq!(body["success"], Value::Bool(true));
  Some(body["data"].clone())
}

pub async fn get_ownership_transfer_json(
  client: &Client,
  node_url: &str,
  token: &str,
  transfer_id: Uuid,
) -> Option<Value> {
  let transfers = api_get(client, &format!("{node_url}/ownership-transfers"), token).await;
  transfers
    .as_array()
    .unwrap()
    .iter()
    .find(|item| item["id"] == transfer_id.to_string())
    .cloned()
}

pub async fn get_ledger_balance_json(
  client: &Client,
  node_url: &str,
  token: &str,
  storage_id: Uuid,
  product_id: Uuid,
  contractor_id: Uuid,
) -> Option<Value> {
  let entry = api_post(
    client,
    &format!("{node_url}/ledger/query"),
    token,
    json!({
      "storageId": storage_id,
      "productId": product_id,
      "contractorId": contractor_id,
    }),
  )
  .await;

  if entry.is_null() {
    None
  } else {
    Some(entry)
  }
}

/// Get all ledger balances from a node via GET /ledger.
pub async fn get_all_ledger_balances(client: &Client, base_url: &str, token: &str) -> Vec<Value> {
  let data = api_get(client, &format!("{base_url}/ledger"), token).await;
  data.as_array().cloned().unwrap_or_default()
}

/// Get all storages belonging to a specific base via warehouse->base chain.
pub async fn get_storages_for_base(
  client: &Client,
  base_url: &str,
  token: &str,
  base_id: Uuid,
) -> Vec<Uuid> {
  let warehouses = api_get(client, &format!("{base_url}/catalog/warehouses"), token).await;
  let base_warehouse_ids: Vec<Uuid> = warehouses
    .as_array()
    .unwrap()
    .iter()
    .filter(|w| w["baseId"].as_str() == Some(&base_id.to_string()))
    .filter_map(|w| w["id"].as_str().and_then(|s| Uuid::parse_str(s).ok()))
    .collect();

  let storages = api_get(client, &format!("{base_url}/catalog/storages"), token).await;
  storages
    .as_array()
    .unwrap()
    .iter()
    .filter(|s| {
      s["warehouseId"]
        .as_str()
        .and_then(|wid| Uuid::parse_str(wid).ok())
        .map(|wid| base_warehouse_ids.contains(&wid))
        .unwrap_or(false)
    })
    .filter_map(|s| s["id"].as_str().and_then(|id| Uuid::parse_str(id).ok()))
    .collect()
}
