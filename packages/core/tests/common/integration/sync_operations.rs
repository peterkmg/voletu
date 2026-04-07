use reqwest::Client;
use serde_json::json;
use uuid::Uuid;

use super::{api_get, api_post};

pub const INITIAL_AUDIT_CURSOR: Uuid = Uuid::from_u128(1);

pub async fn push_outbound_to_central(
  client: &Client,
  source_url: &str,
  source_token: &str,
  central_url: &str,
  central_token: &str,
  after_audit_log_id: Uuid,
) -> usize {
  let logs = api_get(
    client,
    &format!(
      "{source_url}/sync/outbound?afterAuditLogId={}&limit=1000",
      after_audit_log_id
    ),
    source_token,
  )
  .await;
  let logs = logs.as_array().unwrap().clone();

  if !logs.is_empty() {
    let _ = api_post(
      client,
      &format!("{central_url}/sync/push"),
      central_token,
      json!({ "logs": logs }),
    )
    .await;
  }

  logs.len()
}

pub async fn pull_from_central_to_target(
  client: &Client,
  central_url: &str,
  central_token: &str,
  target_url: &str,
  target_token: &str,
  base_ids: &[Uuid],
) -> (usize, Uuid) {
  pull_from_central_to_target_after(
    client,
    central_url,
    central_token,
    target_url,
    target_token,
    base_ids,
    INITIAL_AUDIT_CURSOR,
  )
  .await
}

pub async fn pull_from_central_to_target_after(
  client: &Client,
  central_url: &str,
  central_token: &str,
  target_url: &str,
  target_token: &str,
  base_ids: &[Uuid],
  last_audit_log_id: Uuid,
) -> (usize, Uuid) {
  let base_ids_param = base_ids
    .iter()
    .map(|id| id.to_string())
    .collect::<Vec<_>>()
    .join(",");
  let data = api_get(
    client,
    &format!(
      "{central_url}/sync/pull?lastAuditLogId={last_audit_log_id}&baseIds={base_ids_param}&limit=1000"
    ),
    central_token,
  )
  .await;

  let logs = data["logs"].as_array().unwrap().clone();
  let highest_evaluated = Uuid::parse_str(data["highestEvaluatedId"].as_str().unwrap()).unwrap();

  if !logs.is_empty() {
    let _ = api_post(
      client,
      &format!("{target_url}/sync/push"),
      target_token,
      json!({ "logs": logs }),
    )
    .await;
  }

  (logs.len(), highest_evaluated)
}

pub async fn get_highest_audit_log_id(client: &Client, node_url: &str, token: &str) -> Uuid {
  let status = api_get(client, &format!("{node_url}/sync/status"), token).await;
  Uuid::parse_str(status["highestAuditLogId"].as_str().unwrap()).unwrap()
}
