use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{product, rail_wagon_measurement, rail_wagon_weight, rail_waybill};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "rail_wagon_manifests")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub rail_waybill_id: Uuid,
  #[sea_orm(belongs_to, from = "rail_waybill_id", to = "id")]
  pub rail_waybill: HasOne<rail_waybill::Entity>,
  pub wagon_number: String,
  pub product_id: Uuid,
  #[sea_orm(belongs_to, from = "product_id", to = "id")]
  pub product: HasOne<product::Entity>,
  pub declared_volume: Decimal,
  pub declared_density: Decimal,
  pub declared_mass: Decimal,
  #[sea_orm(has_many)]
  pub measurements: HasMany<rail_wagon_measurement::Entity>,
  #[sea_orm(has_many)]
  pub weights: HasMany<rail_wagon_weight::Entity>,
}
