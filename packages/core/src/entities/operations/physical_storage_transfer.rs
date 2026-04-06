use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{dtos::CreatePhysicalTransferRequest, entities::{company, physical_transfer_item}, enums};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "physical_storage_transfers")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub document_number: String,
  pub date: DateTimeUtc,
  pub status: enums::DocumentStatus,
  pub version: i32,
  pub executed_at: Option<DateTimeUtc>,
  pub executed_by: Option<Uuid>,
  pub reverted_at: Option<DateTimeUtc>,
  pub reverted_by: Option<Uuid>,
  pub contractor_id: Option<Uuid>,
  #[sea_orm(belongs_to, from = "contractor_id", to = "id")]
  pub contractor: HasOne<company::Entity>,
  pub start_cargo_ops: DateTimeUtc,
  pub end_cargo_ops: DateTimeUtc,
  #[sea_orm(has_many)]
  pub items: HasMany<physical_transfer_item::Entity>,
}

impl From<&CreatePhysicalTransferRequest> for ActiveModel {
  fn from(dto: &CreatePhysicalTransferRequest) -> Self {
    Self {
      document_number: Set(dto.document_number.clone()),
      date: Set(dto.date),
      status: Set(enums::DocumentStatus::Draft),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      contractor_id: Set(dto.contractor_id),
      start_cargo_ops: Set(dto.start_cargo_ops),
      end_cargo_ops: Set(dto.end_cargo_ops),
      ..Default::default()
    }
  }
}
