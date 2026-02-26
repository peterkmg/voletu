use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{company, product};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "physical_storage_transfers")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub document_number: String,
  pub date: DateTimeUtc,
  pub start_cargo_ops: DateTimeUtc,
  pub end_cargo_ops: DateTimeUtc,
  pub contractor_id: Uuid,
  #[sea_orm(belongs_to, from = "contractor_id", to = "id")]
  pub contractor: HasOne<company::Entity>,
  pub product_id: Uuid,
  #[sea_orm(belongs_to, from = "product_id", to = "id")]
  pub product: HasOne<product::Entity>,
  pub from_storage_id: Uuid,
  pub to_storage_id: Uuid,
  pub amount_transferred: Decimal,
}
