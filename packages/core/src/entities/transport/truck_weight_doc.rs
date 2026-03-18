use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{dtos::CreateTruckWeightDocRequest, entities::truck_waybill};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "truck_weight_docs")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub truck_waybill_id: Uuid,
  #[sea_orm(belongs_to, from = "truck_waybill_id", to = "id")]
  pub truck_waybill: HasOne<truck_waybill::Entity>,
  pub total_weight: Decimal,
}

impl From<&CreateTruckWeightDocRequest> for ActiveModel {
  fn from(dto: &CreateTruckWeightDocRequest) -> Self {
    Self {
      truck_waybill_id: Set(dto.truck_waybill_id),
      total_weight: Set(dto.weight_doc.total_weight),
      ..Default::default()
    }
  }
}
