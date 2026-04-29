use sea_orm::{entity::prelude::*, model};

use crate::{
  entities::{company, product, storage},
  enums,
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inventory_ledger_entries")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub storage_id: Uuid,
  #[sea_orm(belongs_to, from = "storage_id", to = "id")]
  pub storage: HasOne<storage::Entity>,
  pub product_id: Uuid,
  #[sea_orm(belongs_to, from = "product_id", to = "id")]
  pub product: HasOne<product::Entity>,
  pub contractor_id: Uuid,
  #[sea_orm(belongs_to, from = "contractor_id", to = "id")]
  pub contractor: HasOne<company::Entity>,
  pub quantity_delta: Decimal,
  pub source_kind: enums::LedgerEntrySourceKind,
  pub source_id: Uuid,
  pub source_event: enums::LedgerEntrySourceEvent,
  #[sea_orm(unique)]
  pub reverses_entry_id: Option<Uuid>,
}
