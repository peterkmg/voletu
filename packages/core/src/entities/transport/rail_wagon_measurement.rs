use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::rail_wagon_manifest;

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "rail_wagon_measurements")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub wagon_manifest_id: Uuid,
  #[sea_orm(belongs_to, from = "wagon_manifest_id", to = "id")]
  pub wagon_manifest: HasOne<rail_wagon_manifest::Entity>,
  pub measured_height: Decimal,
  pub lab_density: Option<Decimal>,
  pub calculated_mass: Decimal,
}
