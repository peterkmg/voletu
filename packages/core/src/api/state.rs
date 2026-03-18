use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
  time::Instant,
};

use sea_orm::DatabaseConnection;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{api::ApiServices, config::ApiConfig};

pub struct ApiState {
  pub db: Arc<DatabaseConnection>,
  pub cfg: Arc<ApiConfig>,
  pub restart_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
  pub idempotency_cache: Arc<Mutex<HashMap<Uuid, Instant>>>,
  pub svc: ApiServices,
}

impl ApiState {
  pub fn new(
    db: Arc<DatabaseConnection>,
    cfg: Arc<ApiConfig>,
    restart_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
  ) -> Self {
    let svc = ApiServices::new(db.clone(), cfg.clone());
    Self {
      db,
      cfg,
      restart_tx,
      idempotency_cache: Arc::new(Mutex::new(HashMap::new())),
      svc,
    }
  }
}
