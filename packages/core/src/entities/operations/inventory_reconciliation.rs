use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::CreateInventoryReconciliationRequest,
  entities::{company, inventory_adjustment, warehouse},
  enums,
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "inventory_reconciliations")]
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
  pub contractor_id: Uuid,
  #[sea_orm(belongs_to, from = "contractor_id", to = "id")]
  pub contractor: HasOne<company::Entity>,
  pub warehouse_id: Uuid,
  #[sea_orm(belongs_to, from = "warehouse_id", to = "id")]
  pub warehouse: HasOne<warehouse::Entity>,
  #[sea_orm(has_many)]
  pub adjustments: HasMany<inventory_adjustment::Entity>,
}

impl From<&CreateInventoryReconciliationRequest> for ActiveModel {
  fn from(dto: &CreateInventoryReconciliationRequest) -> Self {
    Self {
      document_number: Set(dto.document_number.clone()),
      date: Set(dto.date),
      status: Set(enums::DocumentStatus::Draft),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      contractor_id: Set(dto.contractor_id),
      warehouse_id: Set(dto.warehouse_id),
      ..Default::default()
    }
  }
}
