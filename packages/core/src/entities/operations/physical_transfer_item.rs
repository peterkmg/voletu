use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::{CreatePhysicalTransferItemRequest, PhysicalTransferItemCompositeRequest},
  entities::{company, physical_storage_transfer, product, storage},
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "physical_transfer_items")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub physical_transfer_id: Uuid,
  #[sea_orm(belongs_to, from = "physical_transfer_id", to = "id")]
  pub physical_transfer: HasOne<physical_storage_transfer::Entity>,
  pub contractor_id: Uuid,
  #[sea_orm(belongs_to, from = "contractor_id", to = "id")]
  pub contractor: HasOne<company::Entity>,
  pub product_id: Uuid,
  #[sea_orm(belongs_to, from = "product_id", to = "id")]
  pub product: HasOne<product::Entity>,
  pub from_storage_id: Uuid,
  #[sea_orm(belongs_to, from = "from_storage_id", to = "id")]
  pub from_storage: HasOne<storage::Entity>,
  pub to_storage_id: Uuid,
  pub amount: Decimal,
}

impl From<&PhysicalTransferItemCompositeRequest> for ActiveModel {
  fn from(dto: &PhysicalTransferItemCompositeRequest) -> Self {
    Self {
      contractor_id: Set(dto.contractor_id),
      product_id: Set(dto.product_id),
      from_storage_id: Set(dto.from_storage_id),
      to_storage_id: Set(dto.to_storage_id),
      amount: Set(dto.amount),
      ..Default::default()
    }
  }
}

impl From<&CreatePhysicalTransferItemRequest> for ActiveModel {
  fn from(dto: &CreatePhysicalTransferItemRequest) -> Self {
    Self {
      physical_transfer_id: Set(dto.physical_transfer_id),
      ..Self::from(&dto.item)
    }
  }
}
