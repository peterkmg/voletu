use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::product;

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "companies")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub common_name: String,
  pub legal_name: Option<String>,
  pub is_contractor: bool,
  pub is_exporter: bool,
  pub is_manufacturer: bool,
  pub is_sender: bool,
  #[sea_orm(has_many)]
  pub products: HasMany<product::Entity>,
}
