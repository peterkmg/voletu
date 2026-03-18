use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::config::ApiConfig;

pub struct SyncService {
  pub(super) db: Arc<DatabaseConnection>,
  pub(super) cfg: Arc<ApiConfig>,
}

impl SyncService {
  pub fn new(db: Arc<DatabaseConnection>, cfg: Arc<ApiConfig>) -> Self {
    Self { db, cfg }
  }
}
