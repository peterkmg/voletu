use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::CreateProductGroupRequest,
  entities::{product, product_type},
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "product_groups")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub product_type_id: Uuid,
  #[sea_orm(belongs_to, from = "product_type_id", to = "id")]
  pub product_type: HasOne<product_type::Entity>,
  #[sea_orm(unique)]
  pub common_name: String,
  pub long_name: Option<String>,
  #[sea_orm(has_many)]
  pub products: HasMany<product::Entity>,
}

impl From<&CreateProductGroupRequest> for ActiveModel {
  fn from(dto: &CreateProductGroupRequest) -> Self {
    Self {
      product_type_id: Set(dto.product_type_id),
      common_name: Set(dto.common_name.clone()),
      long_name: Set(dto.long_name.clone()),
      ..Default::default()
    }
  }
}
