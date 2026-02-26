use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{product_group, storage};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product_types")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub common_name: String,
  pub long_name: Option<String>,
  #[sea_orm(has_many)]
  pub product_groups: HasMany<product_group::Entity>,
  #[sea_orm(has_many)]
  pub storages: HasMany<storage::Entity>,
}
