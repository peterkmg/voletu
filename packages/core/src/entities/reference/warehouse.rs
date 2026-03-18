use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::CreateWarehouseRequest,
  entities::{base, storage},
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "warehouses")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub base_id: Uuid,
  #[sea_orm(belongs_to, from = "base_id", to = "id")]
  pub base: HasOne<base::Entity>,
  pub common_name: String,
  pub long_name: Option<String>,
  #[sea_orm(has_many)]
  pub storages: HasMany<storage::Entity>,
}

impl From<&CreateWarehouseRequest> for ActiveModel {
  fn from(dto: &CreateWarehouseRequest) -> Self {
    Self {
      base_id: Set(dto.base_id),
      common_name: Set(dto.common_name.clone()),
      long_name: Set(dto.long_name.clone()),
      ..Default::default()
    }
  }
}
