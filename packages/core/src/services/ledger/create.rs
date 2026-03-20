use sea_orm::{
  entity::prelude::Decimal,
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  ConnectionTrait,
  EntityTrait,
  QueryFilter,
};
use uuid::Uuid;

use super::LedgerService;
use crate::{api::ApiError, entities::inventory_ledger_entry};

impl LedgerService {
  pub async fn apply_delta(
    &self,
    storage_id: Uuid,
    product_id: Uuid,
    contractor_id: Uuid,
    delta: Decimal,
  ) -> Result<(), ApiError> {
    self
      .apply_delta_on(
        self.db.as_ref(),
        storage_id,
        product_id,
        contractor_id,
        delta,
      )
      .await
  }

  pub async fn apply_delta_on<C: ConnectionTrait>(
    &self,
    conn: &C,
    storage_id: Uuid,
    product_id: Uuid,
    contractor_id: Uuid,
    delta: Decimal,
  ) -> Result<(), ApiError> {
    let existing = inventory_ledger_entry::Entity::find()
      .filter(inventory_ledger_entry::Column::StorageId.eq(storage_id))
      .filter(inventory_ledger_entry::Column::ProductId.eq(product_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(contractor_id))
      .one(conn)
      .await?;

    match existing {
      Some(model) => {
        let mut am: inventory_ledger_entry::ActiveModel = model.into();
        am.current_amount = Set(model.current_amount + delta);
        am.update(conn).await?;
      }
      None => {
        inventory_ledger_entry::ActiveModel {
          id: Set(Uuid::now_v7()),
          storage_id: Set(storage_id),
          product_id: Set(product_id),
          contractor_id: Set(contractor_id),
          current_amount: Set(delta),
          ..Default::default()
        }
        .insert(conn)
        .await?;
      }
    }
    Ok(())
  }
}
