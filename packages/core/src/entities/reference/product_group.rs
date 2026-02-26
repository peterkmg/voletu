use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{product, product_type};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product_groups")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub product_type_id: Uuid,
  #[sea_orm(belongs_to, from = "product_type_id", to = "id")]
  pub product_type: HasOne<product_type::Entity>,
  #[sea_orm(unique)]
  pub common_name: String,
  pub long_name: Option<String>,
  #[sea_orm(has_many)]
  pub products: HasMany<product::Entity>,
}
