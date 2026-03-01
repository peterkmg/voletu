use sea_orm::{
  entity::prelude::*,
  model,
  ActiveValue::{NotSet, Set},
  ConnectionTrait,
};

use crate::{
  dtos::CreateStorageRequest,
  entities::{product_type, warehouse},
};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps(before_save = storage_before_save)]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "storages")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub warehouse_id: Uuid,
  #[sea_orm(belongs_to, from = "warehouse_id", to = "id")]
  pub warehouse: HasOne<warehouse::Entity>,
  pub common_name: String,
  pub long_name: Option<String>,
  pub capacity: Option<Decimal>,
  pub is_type_specific: bool,
  pub product_type_id: Option<Uuid>,
  #[sea_orm(belongs_to, from = "product_type_id", to = "id")]
  pub product_type: HasOne<product_type::Entity>,
}

pub async fn storage_before_save<C: ConnectionTrait>(
  mut model: ActiveModel,
  _db: &C,
  insert: bool,
) -> Result<ActiveModel, DbErr> {
  if insert && matches!(model.is_type_specific, NotSet) {
    model.is_type_specific = Set(false);
  }
  Ok(model)
}

impl From<&CreateStorageRequest> for ActiveModel {
  fn from(dto: &CreateStorageRequest) -> Self {
    Self {
      warehouse_id: Set(dto.warehouse_id),
      common_name: Set(dto.common_name.clone()),
      long_name: Set(dto.long_name.clone()),
      capacity: Set(dto.capacity),
      is_type_specific: Set(dto.is_type_specific.unwrap_or(false)),
      product_type_id: Set(dto.product_type_id),
      ..Default::default()
    }
  }
}
