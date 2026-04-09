use sea_orm::{entity::prelude::Decimal, ActiveModelTrait, ActiveValue::Set, ConnectionTrait};
use uuid::Uuid;

use super::{load_entry_by_dimensions_on, LedgerService};
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
    let existing = load_entry_by_dimensions_on(conn, storage_id, product_id, contractor_id).await?;

    match existing {
      Some(model) => {
        inventory_ledger_entry::ActiveModel {
          id: Set(model.id),
          current_amount: Set(model.current_amount + delta),
          ..Default::default()
        }
        .update(conn)
        .await?;
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
