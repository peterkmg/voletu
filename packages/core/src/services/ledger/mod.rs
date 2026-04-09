mod create;
mod query;

use std::sync::Arc;

use sea_orm::{ColumnTrait, ConnectionTrait, DatabaseConnection, QueryFilter};
use uuid::Uuid;

use crate::{api::ApiError, entities::inventory_ledger_entry};

pub struct LedgerService {
  db: Arc<DatabaseConnection>,
}

impl LedgerService {
  pub fn new(db: Arc<DatabaseConnection>) -> Self {
    Self { db }
  }
}

pub(crate) async fn load_entry_by_dimensions_on<C: ConnectionTrait>(
  conn: &C,
  storage_id: Uuid,
  product_id: Uuid,
  contractor_id: Uuid,
) -> Result<Option<inventory_ledger_entry::ModelEx>, ApiError> {
  inventory_ledger_entry::Entity::load()
    .filter(inventory_ledger_entry::Column::StorageId.eq(storage_id))
    .filter(inventory_ledger_entry::Column::ProductId.eq(product_id))
    .filter(inventory_ledger_entry::Column::ContractorId.eq(contractor_id))
    .one(conn)
    .await
    .map_err(Into::into)
}
