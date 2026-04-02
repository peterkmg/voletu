pub mod cargo_flow;
pub mod rail_receipt;
pub mod truck_dispatch;
pub mod truck_receipt;

use std::sync::Arc;

use sea_orm::DatabaseConnection;

pub struct FlowService {
  pub(crate) db: Arc<DatabaseConnection>,
}

impl FlowService {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }
}
