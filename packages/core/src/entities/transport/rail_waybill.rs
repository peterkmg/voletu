use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::{CreateRailWaybillRequest, RailIntakeCompositeRequest},
  entities::{company, rail_wagon_manifest},
};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "rail_waybills")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub document_number: String,
  pub date: Date,
  pub sender_id: Uuid,
  #[sea_orm(belongs_to, from = "sender_id", to = "id")]
  pub sender: HasOne<company::Entity>,
  #[sea_orm(has_many)]
  pub wagon_manifests: HasMany<rail_wagon_manifest::Entity>,
}

impl From<&CreateRailWaybillRequest> for ActiveModel {
  fn from(dto: &CreateRailWaybillRequest) -> Self {
    Self {
      document_number: Set(dto.document_number.clone()),
      date: Set(dto.date),
      sender_id: Set(dto.sender_id),
      ..Default::default()
    }
  }
}

impl From<&RailIntakeCompositeRequest> for ActiveModel {
  fn from(dto: &RailIntakeCompositeRequest) -> Self {
    Self {
      document_number: Set(dto.document_number.clone()),
      date: Set(dto.date),
      sender_id: Set(dto.sender_id),
      ..Default::default()
    }
  }
}
