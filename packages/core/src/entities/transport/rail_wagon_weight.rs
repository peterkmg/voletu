use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{dtos::CreateRailWagonWeightRequest, entities::rail_wagon_manifest};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "rail_wagon_weights")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub wagon_manifest_id: Uuid,
  #[sea_orm(belongs_to, from = "wagon_manifest_id", to = "id")]
  pub wagon_manifest: HasOne<rail_wagon_manifest::Entity>,
  pub gross_weight: Decimal,
  pub tare_weight: Decimal,
  pub net_product_weight: Decimal,
}

impl From<&CreateRailWagonWeightRequest> for ActiveModel {
  fn from(dto: &CreateRailWagonWeightRequest) -> Self {
    Self {
      wagon_manifest_id: Set(dto.wagon_manifest_id),
      gross_weight: Set(dto.gross_weight),
      tare_weight: Set(dto.tare_weight),
      net_product_weight: Set(dto.net_product_weight),
      ..Default::default()
    }
  }
}
