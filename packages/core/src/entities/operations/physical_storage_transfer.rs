use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::CreatePhysicalTransferRequest,
  entities::{company, enums, product},
};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
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
  pub start_cargo_ops: DateTimeUtc,
  pub end_cargo_ops: DateTimeUtc,
  pub contractor_id: Uuid,
  #[sea_orm(belongs_to, from = "contractor_id", to = "id")]
  pub contractor: HasOne<company::Entity>,
  pub product_id: Uuid,
  #[sea_orm(belongs_to, from = "product_id", to = "id")]
  pub product: HasOne<product::Entity>,
  pub from_storage_id: Uuid,
  pub to_storage_id: Uuid,
  pub amount_transferred: Decimal,
}

impl From<&CreatePhysicalTransferRequest> for ActiveModel {
  fn from(dto: &CreatePhysicalTransferRequest) -> Self {
    Self {
      document_number: Set(dto.document_number.clone()),
      date: Set(dto.date),
      status: Set(enums::DocumentStatus::Draft),
      version: Set(1),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      start_cargo_ops: Set(dto.start_cargo_ops),
      end_cargo_ops: Set(dto.end_cargo_ops),
      contractor_id: Set(dto.contractor_id),
      product_id: Set(dto.product_id),
      from_storage_id: Set(dto.from_storage_id),
      to_storage_id: Set(dto.to_storage_id),
      amount_transferred: Set(dto.amount_transferred),
      ..Default::default()
    }
  }
}
