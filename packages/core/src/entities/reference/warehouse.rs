use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{base, storage};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "warehouses")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub base_id: Uuid,
  #[sea_orm(belongs_to, from = "base_id", to = "id")]
  pub base: HasOne<base::Entity>,
  pub common_name: String,
  pub long_name: Option<String>,
  #[sea_orm(has_many)]
  pub storages: HasMany<storage::Entity>,
}
