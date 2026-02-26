use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{product, storage};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ownership_transfers")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub date: DateTimeUtc,
  pub storage_id: Uuid,
  #[sea_orm(belongs_to, from = "storage_id", to = "id")]
  pub storage: HasOne<storage::Entity>,
  pub product_id: Uuid,
  #[sea_orm(belongs_to, from = "product_id", to = "id")]
  pub product: HasOne<product::Entity>,
  pub from_contractor_id: Uuid,
  pub to_contractor_id: Uuid,
  pub amount_transferred: Decimal,
}
