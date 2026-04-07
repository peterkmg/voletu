#![allow(dead_code)]

use std::{
  collections::HashMap,
  sync::{Mutex, OnceLock},
  time::Duration,
};

use reqwest::{Client, StatusCode};
use sea_orm::{
  ActiveModelTrait,
  ActiveValue::Set,
  DatabaseConnection,
  EntityTrait,
  IntoActiveModel,
};
use serde_json::{json, Value};
use tokio::sync::oneshot;
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  db::init_database,
  entities::{
    base,
    company,
    database_instance,
    inventory_ledger_entry,
    local,
    node_base_assignment,
    ownership_transfer,
    ownership_transfer_item,
    product,
    product_group,
    product_type,
    storage,
    warehouse,
  },
  enums::{DocumentStatus, NodeType},
  serve_api,
  DbConfig,
  DbParams,
  JwtConfig,
};

const INITIAL_AUDIT_CURSOR: Uuid = Uuid::from_u128(1);

fn shared_db_keepalive() -> &'static Mutex<HashMap<String, DatabaseConnection>> {
  static REGISTRY: OnceLock<Mutex<HashMap<String, DatabaseConnection>>> = OnceLock::new();
  REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

async fn ensure_shared_memory_db_alive(db_name: &str) {
  let exists = {
    let registry = shared_db_keepalive()
      .lock()
      .expect("shared DB keepalive lock poisoned");
    registry.contains_key(db_name)
  };

  if exists {
    return;
  }

  let (db, _) = init_database(&db_cfg(db_name)).await.unwrap();

  let mut registry = shared_db_keepalive()
    .lock()
    .expect("shared DB keepalive lock poisoned");
  registry.entry(db_name.to_string()).or_insert(db);
}

#[derive(Clone, Copy)]
pub struct TransferFixtureRefs {
  pub contractor_a_id: Uuid,
  pub contractor_b_id: Uuid,
  pub product_type_id: Uuid,
  pub product_group_id: Uuid,
  pub product_id: Uuid,
  pub warehouse_id: Uuid,
  pub storage_id: Uuid,
}

pub fn db_cfg(db_name: &str) -> DbConfig {
  DbConfig::new(
    DbParams::sqlite_shared_memory(db_name.to_string()),
    "integrationtestpass",
  )
}

pub fn temp_db_path(prefix: &str) -> String {
  format!("{prefix}-{}", Uuid::now_v7())
}

pub fn reserve_port() -> u16 {
  let tcp = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
  let port = tcp.local_addr().unwrap().port();
  drop(tcp);
  port
}

pub async fn wait_for_health(
  client: &Client,
  base_url: &str,
  timeout: Duration,
  server_task: &mut tokio::task::JoinHandle<anyhow::Result<()>>,
) {
  let deadline = tokio::time::Instant::now() + timeout;
  loop {
    if server_task.is_finished() {
      let result = server_task
        .await
        .expect("server task join should succeed while waiting for health");
      panic!("server exited before becoming healthy: {result:?}");
    }

    if let Ok(response) = client.get(format!("{base_url}/health")).send().await {
      if response.status() == StatusCode::OK {
        return;
      }
    }

    assert!(
      tokio::time::Instant::now() < deadline,
      "server did not become healthy at {base_url} within {:?}",
      timeout
    );
    tokio::time::sleep(Duration::from_millis(100)).await;
  }
}

pub async fn wait_for_login_token(
  client: &Client,
  base_url: &str,
  username: &str,
  password: &str,
  timeout: Duration,
) -> String {
  let deadline = tokio::time::Instant::now() + timeout;
  let mut last_status: Option<StatusCode> = None;
  let mut last_body: Option<String> = None;
  loop {
    if let Ok(response) = client
      .post(format!("{base_url}/auth/login"))
      .header("idempotency-key", Uuid::now_v7().to_string())
      .json(&json!({"username": username, "password": password}))
      .send()
      .await
    {
      let status = response.status();
      let body_text = response.text().await.ok();

      last_status = Some(status);
      last_body = body_text.clone();

      if status == StatusCode::OK {
        let body: Value = serde_json::from_str(body_text.as_deref().unwrap_or("{}")).unwrap();
        return body["data"]["accessToken"].as_str().unwrap().to_string();
      }
    }

    assert!(
      tokio::time::Instant::now() < deadline,
      "login for '{username}' did not succeed within {:?}; last_status={:?}; last_body={:?}",
      timeout,
      last_status,
      last_body,
    );
    tokio::time::sleep(Duration::from_millis(120)).await;
  }
}

pub async fn prepare_node_db(
  db_name: &str,
  common_name: &str,
  node_type: NodeType,
  base_id: Option<Uuid>,
) -> Uuid {
  ensure_shared_memory_db_alive(db_name).await;
  let (db, node_cfg) = init_database(&db_cfg(db_name)).await.unwrap();
  let local_row = local::Entity::find_by_id(1)
    .one(&db)
    .await
    .unwrap()
    .unwrap();
  let local_db_id = local_row.local_db_id;

  if let Some(base_id) = base_id {
    with_audit_context(Uuid::now_v7(), node_cfg.db_id, || async {
      if base::Entity::find_by_id(base_id)
        .one(&db)
        .await
        .unwrap()
        .is_none()
      {
        base::ActiveModel {
          id: Set(base_id),
          common_name: Set(format!("Base-{base_id}")),
          long_name: Set(None),
          ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();
      }
    })
    .await;
  }

  with_audit_context(Uuid::now_v7(), node_cfg.db_id, || async {
    let model = database_instance::Entity::find_by_id(local_db_id)
      .one(&db)
      .await
      .unwrap()
      .unwrap();
    let mut am = model.into_active_model();
    am.common_name = Set(common_name.to_string());
    am.node_type = Set(node_type);
    am.base_id = Set(base_id);
    am.update(&db).await.unwrap();

    let mut local_am = local_row.into_active_model();
    local_am.is_initialized = Set(true);
    local_am.update(&db).await.unwrap();

    // Create node_base_assignment for multi-base pull filtering
    if let Some(base_id) = base_id {
      node_base_assignment::ActiveModel {
        node_id: Set(local_db_id),
        base_id: Set(base_id),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }
  })
  .await;

  local_db_id
}

pub async fn register_remote_node_on_central(
  central_db_name: &str,
  node_id: Uuid,
  common_name: &str,
  base_id: Uuid,
) {
  ensure_shared_memory_db_alive(central_db_name).await;
  let (db, central_node_cfg) = init_database(&db_cfg(central_db_name)).await.unwrap();

  with_audit_context(Uuid::now_v7(), central_node_cfg.db_id, || async {
    if base::Entity::find_by_id(base_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      base::ActiveModel {
        id: Set(base_id),
        common_name: Set(format!("Base-{base_id}")),
        long_name: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if database_instance::Entity::find_by_id(node_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      database_instance::ActiveModel {
        id: Set(node_id),
        common_name: Set(common_name.to_string()),
        node_type: Set(NodeType::Peripheral),
        base_id: Set(Some(base_id)),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }
  })
  .await;
}

pub async fn spawn_server(
  db_name: &str,
  port: u16,
) -> (
  oneshot::Sender<()>,
  tokio::task::JoinHandle<anyhow::Result<()>>,
) {
  ensure_shared_memory_db_alive(db_name).await;
  let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
  let db_cfg = db_cfg(db_name);
  let jwt_cfg = JwtConfig::default();
  let server_task = tokio::spawn(async move {
    serve_api(
      "127.0.0.1".to_string(),
      port.to_string(),
      db_cfg,
      jwt_cfg,
      shutdown_rx,
    )
    .await
  });
  (shutdown_tx, server_task)
}

pub async fn api_get(client: &Client, url: &str, token: &str) -> Value {
  let response = client.get(url).bearer_auth(token).send().await.unwrap();
  assert_eq!(response.status(), StatusCode::OK);
  let body: Value = response.json().await.unwrap();
  assert_eq!(body["success"], Value::Bool(true));
  body["data"].clone()
}

pub async fn api_post(client: &Client, url: &str, token: &str, payload: Value) -> Value {
  let response = client
    .post(url)
    .bearer_auth(token)
    .header("idempotency-key", Uuid::now_v7().to_string())
    .json(&payload)
    .send()
    .await
    .unwrap();
  let status = response.status();
  let body: Value = response.json().await.unwrap();
  assert_eq!(
    status,
    StatusCode::OK,
    "POST {url} returned {status}; body: {body}"
  );
  assert_eq!(body["success"], Value::Bool(true));
  body["data"].clone()
}

pub async fn ensure_shared_transfer_refs(
  db_name: &str,
  local_node_id: Uuid,
  base_1: Uuid,
  base_2: Uuid,
) -> TransferFixtureRefs {
  ensure_shared_memory_db_alive(db_name).await;
  let refs = TransferFixtureRefs {
    contractor_a_id: Uuid::parse_str("00000000-0000-0000-0000-00000000aa01").unwrap(),
    contractor_b_id: Uuid::parse_str("00000000-0000-0000-0000-00000000aa02").unwrap(),
    product_type_id: Uuid::parse_str("00000000-0000-0000-0000-00000000bb01").unwrap(),
    product_group_id: Uuid::parse_str("00000000-0000-0000-0000-00000000bb02").unwrap(),
    product_id: Uuid::parse_str("00000000-0000-0000-0000-00000000bb03").unwrap(),
    warehouse_id: Uuid::parse_str("00000000-0000-0000-0000-00000000cc01").unwrap(),
    storage_id: Uuid::parse_str("00000000-0000-0000-0000-00000000cc02").unwrap(),
  };

  let (db, _) = init_database(&db_cfg(db_name)).await.unwrap();
  with_audit_context(Uuid::now_v7(), local_node_id, || async {
    for base_id in [base_1, base_2] {
      if base::Entity::find_by_id(base_id)
        .one(&db)
        .await
        .unwrap()
        .is_none()
      {
        base::ActiveModel {
          id: Set(base_id),
          common_name: Set(format!("Base-{base_id}")),
          long_name: Set(None),
          ..Default::default()
        }
        .insert(&db)
        .await
        .unwrap();
      }
    }

    if company::Entity::find_by_id(refs.contractor_a_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      company::ActiveModel {
        id: Set(refs.contractor_a_id),
        common_name: Set("Transfer-Contractor-A".to_string()),
        legal_name: Set(None),
        is_contractor: Set(true),
        is_exporter: Set(false),
        is_manufacturer: Set(false),
        is_sender: Set(false),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if company::Entity::find_by_id(refs.contractor_b_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      company::ActiveModel {
        id: Set(refs.contractor_b_id),
        common_name: Set("Transfer-Contractor-B".to_string()),
        legal_name: Set(None),
        is_contractor: Set(true),
        is_exporter: Set(false),
        is_manufacturer: Set(false),
        is_sender: Set(false),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if product_type::Entity::find_by_id(refs.product_type_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      product_type::ActiveModel {
        id: Set(refs.product_type_id),
        common_name: Set("Transfer-PT".to_string()),
        long_name: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if product_group::Entity::find_by_id(refs.product_group_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      product_group::ActiveModel {
        id: Set(refs.product_group_id),
        product_type_id: Set(refs.product_type_id),
        common_name: Set("Transfer-PG".to_string()),
        long_name: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if product::Entity::find_by_id(refs.product_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      product::ActiveModel {
        id: Set(refs.product_id),
        product_group_id: Set(refs.product_group_id),
        manufacturer_id: Set(None),
        common_name: Set("Transfer-Product".to_string()),
        long_name: Set(None),
        add_identification: Set(None),
        is_component: Set(true),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if warehouse::Entity::find_by_id(refs.warehouse_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      warehouse::ActiveModel {
        id: Set(refs.warehouse_id),
        base_id: Set(base_1),
        common_name: Set("Transfer-WH".to_string()),
        long_name: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }

    if storage::Entity::find_by_id(refs.storage_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      storage::ActiveModel {
        id: Set(refs.storage_id),
        warehouse_id: Set(refs.warehouse_id),
        common_name: Set("Transfer-Storage".to_string()),
        long_name: Set(None),
        capacity: Set(None),
        is_type_specific: Set(false),
        product_type_id: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }
  })
  .await;

  refs
}

pub async fn create_local_transfer_and_ledger(
  db_name: &str,
  local_node_id: Uuid,
  refs: TransferFixtureRefs,
  transfer_id: Uuid,
  ledger_entry_id: Uuid,
  amount: i64,
) -> (Value, Uuid, Value, Value) {
  ensure_shared_memory_db_alive(db_name).await;
  let (db, _) = init_database(&db_cfg(db_name)).await.unwrap();

  let (transfer, transfer_item, ledger_entry) =
    with_audit_context(Uuid::now_v7(), local_node_id, || async {
      let transfer = ownership_transfer::ActiveModel {
        id: Set(transfer_id),
        date: Set(chrono::Utc::now()),
        status: Set(DocumentStatus::Executed),
        version: Set(1),
        executed_at: Set(Some(chrono::Utc::now())),
        executed_by: Set(Some(local_node_id)),
        reverted_at: Set(None),
        reverted_by: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();

      let transfer_item = ownership_transfer_item::ActiveModel {
        ownership_transfer_id: Set(transfer.id),
        storage_id: Set(refs.storage_id),
        product_id: Set(refs.product_id),
        from_contractor_id: Set(refs.contractor_a_id),
        to_contractor_id: Set(refs.contractor_b_id),
        amount: Set(amount.into()),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();

      let ledger_entry = inventory_ledger_entry::ActiveModel {
        id: Set(ledger_entry_id),
        storage_id: Set(refs.storage_id),
        product_id: Set(refs.product_id),
        contractor_id: Set(refs.contractor_b_id),
        current_amount: Set(amount.into()),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();

      (transfer, transfer_item, ledger_entry)
    })
    .await;

  (
    serde_json::to_value(&transfer).unwrap(),
    transfer_item.id,
    serde_json::to_value(&transfer_item).unwrap(),
    serde_json::to_value(&ledger_entry).unwrap(),
  )
}

pub async fn inject_targeted_idempotency_log(
  client: &Client,
  node_url: &str,
  token: &str,
  origin_db_id: Uuid,
  record_id: Uuid,
  request_key: &str,
  target_base_ids: &str,
) {
  let snapshot = json!({
    "id": record_id,
    "request_key": request_key,
    "created_at": "2026-01-10T00:00:00Z"
  });

  let _ = api_post(
    client,
    &format!("{node_url}/sync/push"),
    token,
    json!({
      "logs": [{
        "id": Uuid::now_v7(),
        "tableName": "audit_logs",
        "recordId": record_id,
        "action": "INSERT",
        "oldValuesJson": null,
        "newValuesJson": serde_json::to_string(&snapshot).unwrap(),
        "targetBaseIds": target_base_ids,
        "userRoleWeight": 10,
        "userId": origin_db_id,
        "timestamp": "2026-01-10T00:00:01Z",
        "originDbId": origin_db_id,
      }]
    }),
  )
  .await;
}

pub async fn inject_targeted_sync_log(
  client: &Client,
  node_url: &str,
  token: &str,
  origin_db_id: Uuid,
  table_name: &str,
  record_id: Uuid,
  snapshot: &Value,
  target_base_ids: &str,
) {
  let _ = api_post(
    client,
    &format!("{node_url}/sync/push"),
    token,
    json!({
      "logs": [{
        "id": Uuid::now_v7(),
        "tableName": table_name,
        "recordId": record_id,
        "action": "UPDATE",
        "oldValuesJson": null,
        "newValuesJson": serde_json::to_string(snapshot).unwrap(),
        "targetBaseIds": target_base_ids,
        "userRoleWeight": 10,
        "userId": origin_db_id,
        "timestamp": "2026-01-10T00:00:01Z",
        "originDbId": origin_db_id,
      }]
    }),
  )
  .await;
}

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

pub async fn get_ledger_entry_json(
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

pub async fn shutdown_server(
  shutdown_tx: oneshot::Sender<()>,
  server_task: tokio::task::JoinHandle<anyhow::Result<()>>,
) {
  let _ = shutdown_tx.send(());
  let join_result = tokio::time::timeout(Duration::from_secs(10), server_task)
    .await
    .expect("server task should shut down in time")
    .expect("server task join should succeed");
  join_result.expect("serve_api should return Ok on shutdown");
}

/// Add a base assignment to a peripheral node's DB.
/// Creates the base entity if it doesn't exist, then inserts a node_base_assignment row.
pub async fn add_base_assignment(db_name: &str, node_id: Uuid, base_id: Uuid, base_name: &str) {
  ensure_shared_memory_db_alive(db_name).await;
  let (db, _) = init_database(&db_cfg(db_name)).await.unwrap();
  with_audit_context(Uuid::now_v7(), node_id, || async {
    if base::Entity::find_by_id(base_id)
      .one(&db)
      .await
      .unwrap()
      .is_none()
    {
      base::ActiveModel {
        id: Set(base_id),
        common_name: Set(base_name.to_string()),
        long_name: Set(None),
        ..Default::default()
      }
      .insert(&db)
      .await
      .unwrap();
    }
    node_base_assignment::ActiveModel {
      node_id: Set(node_id),
      base_id: Set(base_id),
      ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();
  })
  .await;
}

// ---------------------------------------------------------------------------
// Routing integration test helpers — catalog + document creation via HTTP API
// ---------------------------------------------------------------------------

/// IDs returned after seeding a two-base catalog via HTTP API.
#[derive(Clone, Copy, Debug)]
pub struct RoutingCatalog {
  pub base_alpha: Uuid,
  pub base_beta: Uuid,
  pub warehouse_alpha: Uuid,
  pub warehouse_beta: Uuid,
  pub storage_alpha: Uuid,
  pub storage_beta: Uuid,
  pub product_type: Uuid,
  pub product_group: Uuid,
  pub product: Uuid,
  pub product_b: Uuid,
  pub contractor: Uuid,
  pub contractor_b: Uuid,
}

/// Seed catalog entities on a running node via HTTP API.
/// Creates two bases (alpha/beta), each with a warehouse + storage, plus products and companies.
pub async fn seed_catalog_via_api(client: &Client, base_url: &str, token: &str) -> RoutingCatalog {
  let base_alpha = api_post(
    client,
    &format!("{base_url}/catalog/bases"),
    token,
    json!({
      "commonName": "Base Alpha",
      "longName": null,
    }),
  )
  .await;
  let base_alpha_id = parse_uuid(&base_alpha, "id");

  let base_beta = api_post(
    client,
    &format!("{base_url}/catalog/bases"),
    token,
    json!({
      "commonName": "Base Beta",
      "longName": null,
    }),
  )
  .await;
  let base_beta_id = parse_uuid(&base_beta, "id");

  let wh_alpha = api_post(
    client,
    &format!("{base_url}/catalog/warehouses"),
    token,
    json!({
      "baseId": base_alpha_id,
      "commonName": "Warehouse Alpha",
      "longName": null,
    }),
  )
  .await;
  let warehouse_alpha_id = parse_uuid(&wh_alpha, "id");

  let wh_beta = api_post(
    client,
    &format!("{base_url}/catalog/warehouses"),
    token,
    json!({
      "baseId": base_beta_id,
      "commonName": "Warehouse Beta",
      "longName": null,
    }),
  )
  .await;
  let warehouse_beta_id = parse_uuid(&wh_beta, "id");

  let st_alpha = api_post(
    client,
    &format!("{base_url}/catalog/storages"),
    token,
    json!({
      "warehouseId": warehouse_alpha_id,
      "commonName": "Tank Alpha",
      "longName": null,
      "capacity": null,
      "isTypeSpecific": false,
      "productTypeId": null,
    }),
  )
  .await;
  let storage_alpha_id = parse_uuid(&st_alpha, "id");

  let st_beta = api_post(
    client,
    &format!("{base_url}/catalog/storages"),
    token,
    json!({
      "warehouseId": warehouse_beta_id,
      "commonName": "Tank Beta",
      "longName": null,
      "capacity": null,
      "isTypeSpecific": false,
      "productTypeId": null,
    }),
  )
  .await;
  let storage_beta_id = parse_uuid(&st_beta, "id");

  let pt = api_post(
    client,
    &format!("{base_url}/catalog/product-types"),
    token,
    json!({
      "commonName": "Test Fuel",
      "longName": null,
    }),
  )
  .await;
  let product_type_id = parse_uuid(&pt, "id");

  let pg = api_post(
    client,
    &format!("{base_url}/catalog/product-groups"),
    token,
    json!({
      "productTypeId": product_type_id,
      "commonName": "Test Diesel",
      "longName": null,
    }),
  )
  .await;
  let product_group_id = parse_uuid(&pg, "id");

  let prod = api_post(
    client,
    &format!("{base_url}/catalog/products"),
    token,
    json!({
      "productGroupId": product_group_id,
      "manufacturerId": null,
      "commonName": "Product A",
      "longName": null,
      "addIdentification": null,
      "isComponent": true,
    }),
  )
  .await;
  let product_id = parse_uuid(&prod, "id");

  let prod_b = api_post(
    client,
    &format!("{base_url}/catalog/products"),
    token,
    json!({
      "productGroupId": product_group_id,
      "manufacturerId": null,
      "commonName": "Product B",
      "longName": null,
      "addIdentification": null,
      "isComponent": false,
    }),
  )
  .await;
  let product_b_id = parse_uuid(&prod_b, "id");

  let company = api_post(
    client,
    &format!("{base_url}/catalog/companies"),
    token,
    json!({
      "commonName": "Test Contractor",
      "legalName": null,
      "isContractor": true,
      "isExporter": false,
      "isManufacturer": false,
      "isSender": false,
    }),
  )
  .await;
  let contractor_id = parse_uuid(&company, "id");

  let company_b = api_post(
    client,
    &format!("{base_url}/catalog/companies"),
    token,
    json!({
      "commonName": "Test Contractor B",
      "legalName": null,
      "isContractor": true,
      "isExporter": false,
      "isManufacturer": false,
      "isSender": false,
    }),
  )
  .await;
  let contractor_b_id = parse_uuid(&company_b, "id");

  RoutingCatalog {
    base_alpha: base_alpha_id,
    base_beta: base_beta_id,
    warehouse_alpha: warehouse_alpha_id,
    warehouse_beta: warehouse_beta_id,
    storage_alpha: storage_alpha_id,
    storage_beta: storage_beta_id,
    product_type: product_type_id,
    product_group: product_group_id,
    product: product_id,
    product_b: product_b_id,
    contractor: contractor_id,
    contractor_b: contractor_b_id,
  }
}

/// Create an acceptance document via composite HTTP endpoint. Returns the response JSON.
pub async fn create_acceptance_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_number: &str,
  contractor_id: Uuid,
  product_id: Uuid,
  storage_id: Uuid,
  amount: &str,
) -> Value {
  api_post(
    client,
    &format!("{base_url}/acceptance/composite/save"),
    token,
    json!({
      "documentNumber": doc_number,
      "dateAccepted": "2026-01-15T10:00:00Z",
      "arrivalType": "TRUCK",
      "sourceEntity": null,
      "contractorId": contractor_id,
      "truckWaybillId": null,
      "railWaybillId": null,
      "transitDispatchId": null,
      "items": [{
        "productId": product_id,
        "storageId": storage_id,
        "acceptedAmount": amount,
      }]
    }),
  )
  .await
}

/// Create a physical transfer via composite HTTP endpoint. Returns the response JSON.
pub async fn create_physical_transfer_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_number: &str,
  contractor_id: Uuid,
  product_id: Uuid,
  from_storage_id: Uuid,
  to_storage_id: Uuid,
  amount: &str,
) -> Value {
  api_post(
    client,
    &format!("{base_url}/physical-transfers/save"),
    token,
    json!({
      "documentNumber": doc_number,
      "date": "2026-01-15T10:00:00Z",
      "contractorId": contractor_id,
      "startCargoOps": "2026-01-15T08:00:00Z",
      "endCargoOps": "2026-01-15T16:00:00Z",
      "items": [{
        "productId": product_id,
        "fromStorageId": from_storage_id,
        "toStorageId": to_storage_id,
        "amount": amount,
      }]
    }),
  )
  .await
}

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

/// Parse a UUID from a JSON value field.
fn parse_uuid(json: &Value, field: &str) -> Uuid {
  Uuid::parse_str(
    json[field]
      .as_str()
      .unwrap_or_else(|| panic!("missing {field} in response: {json}")),
  )
  .unwrap()
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

/// Create a dispatch composite document via HTTP API.
pub async fn create_dispatch_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_number: &str,
  contractor_id: Uuid,
  product_id: Uuid,
  storage_id: Uuid,
  amount: &str,
  destination_base_id: Option<Uuid>,
) -> Value {
  api_post(
    client,
    &format!("{base_url}/dispatch/composite/save"),
    token,
    json!({
      "documentNumber": doc_number,
      "date": "2026-01-15T10:00:00Z",
      "dispatchPurpose": "EXTERNAL",
      "dispatchMethod": "TRUCK",
      "contractorId": contractor_id,
      "destinationBaseId": destination_base_id,
      "receiverEntity": null,
      "startCargoOps": null,
      "endCargoOps": null,
      "bunkerType": null,
      "exporterId": null,
      "portId": null,
      "items": [{
        "productId": product_id,
        "storageId": storage_id,
        "dispatchedAmount": amount,
      }],
      "storageMeasurements": null,
    }),
  )
  .await
}

/// Create a blending composite document via HTTP API.
pub async fn create_blending_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_number: &str,
  contractor_id: Uuid,
  target_product_id: Uuid,
  component_storage_id: Uuid,
  source_product_id: Uuid,
  component_amount: &str,
  result_storage_id: Uuid,
  result_amount: &str,
) -> Value {
  api_post(
    client,
    &format!("{base_url}/blending/composite/save"),
    token,
    json!({
      "documentNumber": doc_number,
      "date": "2026-01-15T10:00:00Z",
      "contractorId": contractor_id,
      "targetProductId": target_product_id,
      "components": [{
        "storageId": component_storage_id,
        "sourceProductId": source_product_id,
        "amountUsed": component_amount,
      }],
      "results": [{
        "storageId": result_storage_id,
        "producedAmount": result_amount,
      }]
    }),
  )
  .await
}

/// Create an ownership transfer composite document via HTTP API.
pub async fn create_ownership_transfer_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  storage_id: Uuid,
  product_id: Uuid,
  from_contractor_id: Uuid,
  to_contractor_id: Uuid,
  amount: &str,
) -> Value {
  api_post(
    client,
    &format!("{base_url}/ownership-transfers/save"),
    token,
    json!({
      "date": "2026-01-15T10:00:00Z",
      "items": [{
        "storageId": storage_id,
        "productId": product_id,
        "fromContractorId": from_contractor_id,
        "toContractorId": to_contractor_id,
        "amount": amount,
      }]
    }),
  )
  .await
}

/// Create a reconciliation document via HTTP API.
pub async fn create_reconciliation_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_number: &str,
  contractor_id: Uuid,
  warehouse_id: Uuid,
) -> Value {
  api_post(
    client,
    &format!("{base_url}/reconciliations/save"),
    token,
    json!({
      "documentNumber": doc_number,
      "date": "2026-01-15T10:00:00Z",
      "contractorId": contractor_id,
      "warehouseId": warehouse_id,
    }),
  )
  .await
}

/// Execute a document via HTTP API (POST to execute/{id}).
pub async fn execute_document_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  execute_path: &str,
  doc_id: Uuid,
) -> Value {
  let url = format!(
    "{base_url}{}",
    execute_path.replace("{id}", &doc_id.to_string())
  );
  let response = client
    .post(&url)
    .bearer_auth(token)
    .header("idempotency-key", Uuid::now_v7().to_string())
    .send()
    .await
    .unwrap();
  let status = response.status();
  let body: Value = response.json().await.unwrap();
  assert_eq!(
    status,
    StatusCode::OK,
    "POST {url} returned {status}; body: {body}"
  );
  assert_eq!(body["success"], Value::Bool(true));
  body["data"].clone()
}

/// Add a base assignment to a running node via HTTP API (POST /node/bases).
pub async fn add_base_assignment_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  base_id: Uuid,
) {
  api_post(
    client,
    &format!("{base_url}/node/bases"),
    token,
    json!({ "baseId": base_id }),
  )
  .await;
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

// ===========================================================================
// API-only node setup helpers — no direct DB access for business logic
// ===========================================================================

/// Holds the running state of a node after API-only setup.
pub struct NodeHandle {
  pub url: String,
  pub token: String,
  pub shutdown_tx: oneshot::Sender<()>,
  pub task: tokio::task::JoinHandle<anyhow::Result<()>>,
}

impl NodeHandle {
  pub async fn shutdown(self) {
    shutdown_server(self.shutdown_tx, self.task).await;
  }
}

/// Spawn a server and initialize it as Central via API only.
pub async fn setup_central_via_api(client: &Client, db_name: &str) -> NodeHandle {
  let port = reserve_port();
  let (shutdown_tx, mut task) = spawn_server(db_name, port).await;
  let url = format!("http://127.0.0.1:{port}");
  wait_for_health(client, &url, Duration::from_secs(10), &mut task).await;

  let bootstrap_token =
    wait_for_login_token(client, &url, "admin", "admin", Duration::from_secs(5)).await;

  api_post(
    client,
    &format!("{url}/node/initialize"),
    &bootstrap_token,
    json!({
      "nodeType": "CENTRAL",
      "nodeName": "Central",
      "centralApiUrl": null,
      "newUsername": "root",
      "newPassword": "rootpass",
      "fullname": "Root Admin",
    }),
  )
  .await;

  wait_for_health(client, &url, Duration::from_secs(20), &mut task).await;
  let token = wait_for_login_token(client, &url, "root", "rootpass", Duration::from_secs(10)).await;

  NodeHandle {
    url,
    token,
    shutdown_tx,
    task,
  }
}

/// Spawn a server and initialize it as a Peripheral via API only.
/// Pulls catalog from Central, then adds base assignments.
pub async fn setup_peripheral_via_api(
  client: &Client,
  db_name: &str,
  central: &NodeHandle,
  base_ids: &[Uuid],
) -> NodeHandle {
  let port = reserve_port();
  let (shutdown_tx, mut task) = spawn_server(db_name, port).await;
  let url = format!("http://127.0.0.1:{port}");
  wait_for_health(client, &url, Duration::from_secs(10), &mut task).await;

  let bootstrap_token =
    wait_for_login_token(client, &url, "admin", "admin", Duration::from_secs(5)).await;

  api_post(
    client,
    &format!("{url}/node/initialize"),
    &bootstrap_token,
    json!({
      "nodeType": "PERIPHERAL",
      "nodeName": null,
      "centralApiUrl": central.url,
      "newUsername": "root",
      "newPassword": "rootpass",
      "fullname": "Root Admin",
    }),
  )
  .await;

  wait_for_health(client, &url, Duration::from_secs(20), &mut task).await;
  let token = wait_for_login_token(client, &url, "root", "rootpass", Duration::from_secs(10)).await;

  // Pull catalog from Central so base entities exist on this peripheral.
  // Loop until all catalog audit logs are consumed (large seeds may exceed single batch).
  let mut cursor = INITIAL_AUDIT_CURSOR;
  loop {
    let (pulled, new_cursor) = pull_from_central_to_target_after(
      client,
      &central.url,
      &central.token,
      &url,
      &token,
      &[],
      cursor,
    )
    .await;
    if pulled == 0 || new_cursor == cursor {
      break;
    }
    cursor = new_cursor;
  }

  // Add base assignments via API
  for base_id in base_ids {
    add_base_assignment_via_api(client, &url, &token, *base_id).await;
  }

  NodeHandle {
    url,
    token,
    shutdown_tx,
    task,
  }
}

/// Soft-delete an entity via HTTP DELETE /{path}/{id}.
pub async fn soft_delete_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  path: &str,
  id: Uuid,
) {
  let url = format!("{base_url}{}", path.replace("{id}", &id.to_string()));
  let response = client
    .delete(&url)
    .bearer_auth(token)
    .header("idempotency-key", Uuid::now_v7().to_string())
    .send()
    .await
    .unwrap();
  let status = response.status();
  let body: Value = response.json().await.unwrap();
  assert_eq!(
    status,
    StatusCode::OK,
    "DELETE {url} returned {status}; body: {body}"
  );
}

/// Hard-delete an entity via HTTP DELETE /{path}/{id}/hard.
pub async fn hard_delete_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  path: &str,
  id: Uuid,
) {
  let url = format!("{base_url}{}", path.replace("{id}", &id.to_string()));
  let response = client
    .delete(&url)
    .bearer_auth(token)
    .header("idempotency-key", Uuid::now_v7().to_string())
    .send()
    .await
    .unwrap();
  let status = response.status();
  let body: Value = response.json().await.unwrap();
  assert_eq!(
    status,
    StatusCode::OK,
    "DELETE {url} returned {status}; body: {body}"
  );
}

/// Revert a document via HTTP POST /{revert_path}/{id}.
pub async fn revert_document_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  revert_path: &str,
  doc_id: Uuid,
) {
  execute_document_via_api(client, base_url, token, revert_path, doc_id).await;
}

/// Call POST /dev/seed on a running node. Returns the seed result.
pub async fn dev_seed_via_api(client: &Client, base_url: &str, token: &str) -> Value {
  api_post(client, &format!("{base_url}/dev/seed"), token, json!({})).await
}

/// Get all ledger entries from a node via GET /ledger.
pub async fn get_all_ledger_entries(client: &Client, base_url: &str, token: &str) -> Vec<Value> {
  let data = api_get(client, &format!("{base_url}/ledger"), token).await;
  data.as_array().cloned().unwrap_or_default()
}

/// Get all storages belonging to a specific base via warehouse→base chain.
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
