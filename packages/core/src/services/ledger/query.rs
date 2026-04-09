use uuid::Uuid;

use super::{load_entry_by_dimensions_on, LedgerService};
use crate::{api::ApiError, dtos::LedgerEntryResponse, entities::inventory_ledger_entry};

impl LedgerService {
  pub async fn list(&self) -> Result<Vec<LedgerEntryResponse>, ApiError> {
    let rows: Vec<inventory_ledger_entry::ModelEx> = inventory_ledger_entry::Entity::load()
      .all(self.db.as_ref())
      .await?;
    Ok(rows.into_iter().map(LedgerEntryResponse::from).collect())
  }

  pub async fn by_dimensions(
    &self,
    storage_id: Uuid,
    product_id: Uuid,
    contractor_id: Uuid,
  ) -> Result<Option<LedgerEntryResponse>, ApiError> {
    let row =
      load_entry_by_dimensions_on(self.db.as_ref(), storage_id, product_id, contractor_id).await?;

    Ok(row.map(LedgerEntryResponse::from))
  }
}
