use sea_orm::{
  entity::prelude::*,
  model,
  ActiveValue::{NotSet, Set},
  ConnectionTrait,
};

use crate::{
  dtos::CreateProductRequest,
  entities::{company, product_group},
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields(before_save = product_before_save)]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "products")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub product_group_id: Uuid,
  #[sea_orm(belongs_to, from = "product_group_id", to = "id")]
  pub product_group: HasOne<product_group::Entity>,
  pub manufacturer_id: Option<Uuid>,
  #[sea_orm(belongs_to, from = "manufacturer_id", to = "id")]
  pub manufacturer: HasOne<company::Entity>,
  pub common_name: String,
  pub long_name: Option<String>,
  pub add_identification: Option<String>,
  pub is_component: bool,
}

pub async fn product_before_save<C: ConnectionTrait>(
  mut model: ActiveModel,
  _db: &C,
  insert: bool,
) -> Result<ActiveModel, DbErr> {
  if insert && matches!(model.is_component, NotSet) {
    model.is_component = Set(true);
  }
  Ok(model)
}

impl From<&CreateProductRequest> for ActiveModel {
  fn from(dto: &CreateProductRequest) -> Self {
    Self {
      product_group_id: Set(dto.product_group_id),
      manufacturer_id: Set(dto.manufacturer_id),
      common_name: Set(dto.common_name.clone()),
      long_name: Set(dto.long_name.clone()),
      add_identification: Set(dto.add_identification.clone()),
      is_component: Set(dto.is_component.unwrap_or(true)),
      ..Default::default()
    }
  }
}
