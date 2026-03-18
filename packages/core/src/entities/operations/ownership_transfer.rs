use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{dtos::CreateOwnershipTransferRequest, entities::ownership_transfer_item, enums};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
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
  #[sea_orm(has_many)]
  pub items: HasMany<ownership_transfer_item::Entity>,
}

impl From<&CreateOwnershipTransferRequest> for ActiveModel {
  fn from(dto: &CreateOwnershipTransferRequest) -> Self {
    Self {
      date: Set(dto.date),
      status: Set(enums::DocumentStatus::Draft),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      ..Default::default()
    }
  }
}
