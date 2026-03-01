use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::CreateDispatchMeasurementRequest,
  entities::{dispatch_document, storage},
};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dispatch_storage_measurements")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub dispatch_doc_id: Uuid,
  #[sea_orm(belongs_to, from = "dispatch_doc_id", to = "id")]
  pub dispatch_doc: HasOne<dispatch_document::Entity>,
  pub storage_id: Uuid,
  #[sea_orm(belongs_to, from = "storage_id", to = "id")]
  pub storage: HasOne<storage::Entity>,
  pub before_height: Option<Decimal>,
  pub before_volume: Option<Decimal>,
  pub before_density: Option<Decimal>,
  pub before_mass: Decimal,
  pub after_height: Option<Decimal>,
  pub after_volume: Option<Decimal>,
  pub after_density: Option<Decimal>,
  pub after_mass: Decimal,
}

impl From<&CreateDispatchMeasurementRequest> for ActiveModel {
  fn from(dto: &CreateDispatchMeasurementRequest) -> Self {
    Self {
      dispatch_doc_id: Set(dto.dispatch_doc_id),
      storage_id: Set(dto.measurement.storage_id),
      before_height: Set(dto.measurement.before_height),
      before_volume: Set(dto.measurement.before_volume),
      before_density: Set(dto.measurement.before_density),
      before_mass: Set(dto.measurement.before_mass),
      after_height: Set(dto.measurement.after_height),
      after_volume: Set(dto.measurement.after_volume),
      after_density: Set(dto.measurement.after_density),
      after_mass: Set(dto.measurement.after_mass),
      ..Default::default()
    }
  }
}
