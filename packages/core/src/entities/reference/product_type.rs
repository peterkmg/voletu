use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::CreateProductTypeRequest,
  entities::{product_group, storage},
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product_types")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub common_name: String,
  pub long_name: Option<String>,
  #[sea_orm(has_many)]
  pub product_groups: HasMany<product_group::Entity>,
  #[sea_orm(has_many)]
  pub storages: HasMany<storage::Entity>,
}

impl From<&CreateProductTypeRequest> for ActiveModel {
  fn from(dto: &CreateProductTypeRequest) -> Self {
    Self {
      common_name: Set(dto.common_name.clone()),
      long_name: Set(dto.long_name.clone()),
      ..Default::default()
    }
  }
}
