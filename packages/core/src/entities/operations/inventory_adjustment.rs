use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::CreateInventoryAdjustmentRequest,
  entities::{inventory_reconciliation, product, storage},
  enums,
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inventory_adjustments")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub reconciliation_id: Uuid,
  #[sea_orm(belongs_to, from = "reconciliation_id", to = "id")]
  pub reconciliation: HasOne<inventory_reconciliation::Entity>,
  pub storage_id: Uuid,
  #[sea_orm(belongs_to, from = "storage_id", to = "id")]
  pub storage: HasOne<storage::Entity>,
  pub product_id: Uuid,
  #[sea_orm(belongs_to, from = "product_id", to = "id")]
  pub product: HasOne<product::Entity>,
  pub adjustment_type: enums::AdjustmentType,
  pub amount: Decimal,
  pub reason: Option<String>,
}

impl From<&CreateInventoryAdjustmentRequest> for ActiveModel {
  fn from(dto: &CreateInventoryAdjustmentRequest) -> Self {
    Self {
      reconciliation_id: Set(dto.reconciliation_id),
      storage_id: Set(dto.storage_id),
      product_id: Set(dto.product_id),
      adjustment_type: Set(dto.adjustment_type),
      amount: Set(dto.amount),
      reason: Set(dto.reason.clone()),
      ..Default::default()
    }
  }
}
