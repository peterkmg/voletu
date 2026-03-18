use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::{CreateOwnershipTransferItemRequest, OwnershipTransferItemCompositeRequest},
  entities::{ownership_transfer, product, storage},
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ownership_transfer_items")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub ownership_transfer_id: Uuid,
  #[sea_orm(belongs_to, from = "ownership_transfer_id", to = "id")]
  pub ownership_transfer: HasOne<ownership_transfer::Entity>,
  pub storage_id: Uuid,
  #[sea_orm(belongs_to, from = "storage_id", to = "id")]
  pub storage: HasOne<storage::Entity>,
  pub product_id: Uuid,
  #[sea_orm(belongs_to, from = "product_id", to = "id")]
  pub product: HasOne<product::Entity>,
  pub from_contractor_id: Uuid,
  pub to_contractor_id: Uuid,
  pub amount: Decimal,
}

impl From<&OwnershipTransferItemCompositeRequest> for ActiveModel {
  fn from(dto: &OwnershipTransferItemCompositeRequest) -> Self {
    Self {
      storage_id: Set(dto.storage_id),
      product_id: Set(dto.product_id),
      from_contractor_id: Set(dto.from_contractor_id),
      to_contractor_id: Set(dto.to_contractor_id),
      amount: Set(dto.amount),
      ..Default::default()
    }
  }
}

impl From<&CreateOwnershipTransferItemRequest> for ActiveModel {
  fn from(dto: &CreateOwnershipTransferItemRequest) -> Self {
    Self {
      ownership_transfer_id: Set(dto.ownership_transfer_id),
      ..Self::from(&dto.item)
    }
  }
}
