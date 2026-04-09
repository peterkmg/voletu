use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{config::ApiConfig, services::audit::AuditService};

mod auth_helpers;
mod auth_initialization;
mod auth_password;
mod auth_session;
pub mod database_instance;
pub mod local;
pub mod node_bases;
pub mod role;
pub mod token;
pub mod user;

pub struct SystemService {
  pub(super) db: Arc<DatabaseConnection>,
  pub(super) cfg: Arc<ApiConfig>,
  pub(super) audit: Arc<AuditService>,
}

impl SystemService {
  pub fn new(db: Arc<DatabaseConnection>, cfg: Arc<ApiConfig>) -> Self {
    let audit = Arc::new(AuditService::new(cfg.clone()));
    Self { db, cfg, audit }
  }
}
