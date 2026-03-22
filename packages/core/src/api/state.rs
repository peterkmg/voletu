use std::{
  collections::HashMap,
  sync::{
    atomic::AtomicBool,
    Arc, Mutex,
  },
  time::Instant,
};

use sea_orm::DatabaseConnection;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{api::ApiServices, config::ApiConfig, worker::WorkerStatus};

pub struct ApiState {
  pub db: Arc<DatabaseConnection>,
  pub cfg: Arc<ApiConfig>,
  pub restart_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
  pub idempotency_cache: Arc<Mutex<HashMap<Uuid, Instant>>>,
  pub is_initialized: AtomicBool,
  pub worker_status: Arc<tokio::sync::RwLock<WorkerStatus>>,
  pub svc: ApiServices,
}

impl ApiState {
  pub fn new(
    db: Arc<DatabaseConnection>,
    cfg: Arc<ApiConfig>,
    restart_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    worker_status: Arc<tokio::sync::RwLock<WorkerStatus>>,
    is_initialized: bool,
  ) -> Self {
    let is_initialized = AtomicBool::new(is_initialized);
    let svc = ApiServices::new(db.clone(), cfg.clone());
    Self {
      db,
      cfg,
      restart_tx,
      idempotency_cache: Arc::new(Mutex::new(HashMap::new())),
      is_initialized,
      worker_status,
      svc,
    }
  }
}
