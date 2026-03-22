use sea_orm::DatabaseConnection;

use crate::services::audit::AuditService;

pub mod base;
pub mod company;
pub mod port;
pub mod product;
pub mod product_group;
pub mod product_type;
mod resolve;
pub mod storage;
pub mod warehouse;

pub struct CatalogService {
  pub(super) db: std::sync::Arc<DatabaseConnection>,
  pub(super) audit: std::sync::Arc<AuditService>,
}

impl CatalogService {
  pub fn new(db: std::sync::Arc<DatabaseConnection>, audit: std::sync::Arc<AuditService>) -> Self {
    Self { db, audit }
  }
}
