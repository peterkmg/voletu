use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::CreateOwnershipTransferRequest,
  entities::{enums, product, storage},
};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ownership_transfers")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub date: DateTimeUtc,
  pub status: enums::DocumentStatus,
  pub version: i32,
  pub executed_at: Option<DateTimeUtc>,
  pub executed_by: Option<Uuid>,
  pub reverted_at: Option<DateTimeUtc>,
  pub reverted_by: Option<Uuid>,
  pub storage_id: Uuid,
  #[sea_orm(belongs_to, from = "storage_id", to = "id")]
  pub storage: HasOne<storage::Entity>,
  pub product_id: Uuid,
  #[sea_orm(belongs_to, from = "product_id", to = "id")]
  pub product: HasOne<product::Entity>,
  pub from_contractor_id: Uuid,
  pub to_contractor_id: Uuid,
  pub amount_transferred: Decimal,
}

impl From<&CreateOwnershipTransferRequest> for ActiveModel {
  fn from(dto: &CreateOwnershipTransferRequest) -> Self {
    Self {
      date: Set(dto.date),
      status: Set(enums::DocumentStatus::Draft),
      version: Set(1),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      storage_id: Set(dto.storage_id),
      product_id: Set(dto.product_id),
      from_contractor_id: Set(dto.from_contractor_id),
      to_contractor_id: Set(dto.to_contractor_id),
      amount_transferred: Set(dto.amount_transferred),
      ..Default::default()
    }
  }
}
