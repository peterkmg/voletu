mod create;
mod query;

use std::sync::Arc;

use sea_orm::DatabaseConnection;
pub struct LedgerService {
  db: Arc<DatabaseConnection>,
}

impl LedgerService {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }
}
