use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{acceptance_item, storage};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "acceptance_storage_allocations")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub acceptance_item_id: Uuid,
  #[sea_orm(belongs_to, from = "acceptance_item_id", to = "id")]
  pub acceptance_item: HasOne<acceptance_item::Entity>,
  pub storage_id: Uuid,
  #[sea_orm(belongs_to, from = "storage_id", to = "id")]
  pub storage: HasOne<storage::Entity>,
  pub allocated_amount: Decimal,
}
