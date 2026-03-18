use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{dtos::CreateRailWagonMeasurementRequest, entities::rail_wagon_manifest};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
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

impl From<&CreateRailWagonMeasurementRequest> for ActiveModel {
  fn from(dto: &CreateRailWagonMeasurementRequest) -> Self {
    Self {
      wagon_manifest_id: Set(dto.wagon_manifest_id),
      measured_height: Set(dto.measured_height),
      lab_density: Set(dto.lab_density),
      calculated_mass: Set(dto.calculated_mass),
      ..Default::default()
    }
  }
}
