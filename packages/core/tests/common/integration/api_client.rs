use std::{
  collections::HashMap,
  sync::{Mutex, OnceLock},
};

use reqwest::{Client, StatusCode};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use uuid::Uuid;
use voletu_core::db::init_database;

use super::server::db_cfg;

fn shared_db_keepalive() -> &'static Mutex<HashMap<String, DatabaseConnection>> {
  static REGISTRY: OnceLock<Mutex<HashMap<String, DatabaseConnection>>> = OnceLock::new();
  REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(super) async fn ensure_shared_memory_db_alive(db_name: &str) {
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
