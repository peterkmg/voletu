use sea_orm::{
  entity::prelude::Decimal,
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  ConnectionTrait,
  QueryFilter,
};
use uuid::Uuid;

use super::LedgerService;
use crate::{
  api::ApiError,
  entities::inventory_ledger_entry,
  enums::{LedgerEntrySourceEvent, LedgerEntrySourceKind},
};

pub struct LedgerDelta {
  pub storage_id: Uuid,
  pub product_id: Uuid,
  pub contractor_id: Uuid,
  pub quantity_delta: Decimal,
  pub source_kind: LedgerEntrySourceKind,
  pub source_id: Uuid,
  pub source_event: LedgerEntrySourceEvent,
  pub reverses_entry_id: Option<Uuid>,
}

impl LedgerService {
  pub async fn append_delta_on<C: ConnectionTrait>(
    &self,
    conn: &C,
    delta: LedgerDelta,
  ) -> Result<inventory_ledger_entry::Model, ApiError> {
    let entry = inventory_ledger_entry::ActiveModel {
      id: Set(Uuid::now_v7()),
      storage_id: Set(delta.storage_id),
      product_id: Set(delta.product_id),
      contractor_id: Set(delta.contractor_id),
      quantity_delta: Set(delta.quantity_delta),
      source_kind: Set(delta.source_kind),
      source_id: Set(delta.source_id),
      source_event: Set(delta.source_event),
      reverses_entry_id: Set(delta.reverses_entry_id),
      ..Default::default()
    }
    .insert(conn)
    .await?;

    Ok(entry)
  }

  pub async fn append_reversal_deltas_on<C: ConnectionTrait>(
    &self,
    conn: &C,
    source_kind: LedgerEntrySourceKind,
    source_id: Uuid,
  ) -> Result<(), ApiError> {
    let original_rows: Vec<inventory_ledger_entry::ModelEx> =
      inventory_ledger_entry::Entity::load()
        .filter(inventory_ledger_entry::Column::SourceKind.eq(source_kind))
        .filter(inventory_ledger_entry::Column::SourceId.eq(source_id))
        .filter(inventory_ledger_entry::Column::SourceEvent.eq(LedgerEntrySourceEvent::Execution))
        .all(conn)
        .await?;

    for row in original_rows {
      let existing_reversal = inventory_ledger_entry::Entity::load()
        .filter(inventory_ledger_entry::Column::ReversesEntryId.eq(Some(row.id)))
        .one(conn)
        .await?;

      if existing_reversal.is_some() {
        continue;
      }

      self
        .append_delta_on(conn, LedgerDelta {
          storage_id: row.storage_id,
          product_id: row.product_id,
          contractor_id: row.contractor_id,
          quantity_delta: -row.quantity_delta,
          source_kind: row.source_kind,
          source_id: row.source_id,
          source_event: LedgerEntrySourceEvent::Reversion,
          reverses_entry_id: Some(row.id),
        })
        .await?;
    }

    Ok(())
  }
}
