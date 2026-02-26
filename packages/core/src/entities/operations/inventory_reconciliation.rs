use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{inventory_adjustment, warehouse};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "inventory_reconciliations")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub document_number: String,
  pub date: DateTimeUtc,
  pub warehouse_id: Uuid,
  #[sea_orm(belongs_to, from = "warehouse_id", to = "id")]
  pub warehouse: HasOne<warehouse::Entity>,
  #[sea_orm(has_many)]
  pub adjustments: HasMany<inventory_adjustment::Entity>,
}
