use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::{CreateTruckWaybillRequest, TruckWaybillCompositeRequest},
  entities::{company, truck_waybill_item, truck_weight_doc},
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "truck_waybills")]
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
  pub items: HasMany<truck_waybill_item::Entity>,
  #[sea_orm(has_many)]
  pub weight_docs: HasMany<truck_weight_doc::Entity>,
}

impl From<&CreateTruckWaybillRequest> for ActiveModel {
  fn from(dto: &CreateTruckWaybillRequest) -> Self {
    Self {
      document_number: Set(dto.document_number.clone()),
      date: Set(dto.date),
      sender_id: Set(dto.sender_id),
      ..Default::default()
    }
  }
}

impl From<&TruckWaybillCompositeRequest> for ActiveModel {
  fn from(dto: &TruckWaybillCompositeRequest) -> Self {
    Self {
      document_number: Set(dto.document_number.clone()),
      date: Set(dto.date),
      sender_id: Set(dto.sender_id),
      ..Default::default()
    }
  }
}
