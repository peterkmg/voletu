use std::sync::{Arc, Mutex};

use sea_orm::{Database, DatabaseConnection, EntityTrait};
use tokio::sync::oneshot;
use uuid::Uuid;
use voletu_core::{
  api::ApiState,
  config::{ApiConfig, DbConfig, DbParams, JwtConfig, NodeConfig},
  entities::local,
  worker::WorkerStatus,
};

pub async fn setup_db() -> DatabaseConnection {
  let db = Database::connect("sqlite::memory:").await.unwrap();
  db.get_schema_registry("voletu-core::entities::*")
    .sync(&db)
    .await
    .unwrap();
  db
}

#[allow(dead_code)]
pub fn test_config() -> ApiConfig {
  ApiConfig::new(
    NodeConfig::new(
      Uuid::now_v7(),
      "PERIPHERAL".to_string(),
      "test-secret".to_string(),
      None,
    ),
    JwtConfig::default(),
    DbConfig::new(DbParams::sqlite_memory(), "test-pass"),
  )
}

#[allow(dead_code)]
pub async fn test_config_for_db(db: &DatabaseConnection) -> ApiConfig {
  let node_id = local::Entity::find_by_id(1)
    .one(db)
    .await
    .unwrap()
    .map(|row| row.local_db_id)
    .unwrap_or_else(Uuid::now_v7);

  ApiConfig::new(
    NodeConfig::new(
      node_id,
      "PERIPHERAL".to_string(),
      "test-secret".to_string(),
      None,
    ),
    JwtConfig::default(),
    DbConfig::new(DbParams::sqlite_memory(), "test-pass"),
  )
}

#[allow(dead_code)]
pub fn test_api_state_with_default_restart_controls(
  db: Arc<DatabaseConnection>,
  cfg: Arc<ApiConfig>,
) -> ApiState {
  let (restart_tx, restart_rx) = oneshot::channel();
  drop(restart_rx);
  let restart_tx = Arc::new(Mutex::new(Some(restart_tx)));
  let worker_status = Arc::new(tokio::sync::RwLock::new(WorkerStatus::default()));
  ApiState::new(db, cfg, restart_tx, worker_status, true)
}
