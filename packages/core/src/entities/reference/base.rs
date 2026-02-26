use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{database_instance, user, warehouse};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "bases")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub common_name: String,
  pub long_name: Option<String>,
  #[sea_orm(has_many)]
  pub warehouses: HasMany<warehouse::Entity>,
  #[sea_orm(has_many)]
  pub users: HasMany<user::Entity>,
  #[sea_orm(has_many)]
  pub database_instances: HasMany<database_instance::Entity>,
}
