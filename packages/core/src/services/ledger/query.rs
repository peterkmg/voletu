use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use super::LedgerService;
use crate::{
  api::ApiError,
  db::ops::list_all,
  dtos::LedgerEntryResponse,
  entities::inventory_ledger_entry,
};

impl LedgerService {
  pub async fn list(&self) -> Result<Vec<LedgerEntryResponse>, ApiError> {
    let rows = list_all::<inventory_ledger_entry::Entity>(self.db.as_ref()).await?;
    Ok(rows.into_iter().map(LedgerEntryResponse::from).collect())
  }

  pub async fn by_dimensions(
    &self,
    storage_id: Uuid,
    product_id: Uuid,
    contractor_id: Uuid,
  ) -> Result<Option<LedgerEntryResponse>, ApiError> {
    let row = inventory_ledger_entry::Entity::find()
      .filter(inventory_ledger_entry::Column::StorageId.eq(storage_id))
      .filter(inventory_ledger_entry::Column::ProductId.eq(product_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(contractor_id))
      .one(self.db.as_ref())
      .await?;

    Ok(row.map(LedgerEntryResponse::from))
  }
}
