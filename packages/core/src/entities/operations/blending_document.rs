use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::{CreateBlendingCompositeRequest, CreateBlendingRequest},
  entities::{blending_component, blending_result, company, product},
};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "blending_documents")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub document_number: String,
  pub date: DateTimeUtc,
  pub status: crate::enums::DocumentStatus,
  pub version: i32,
  pub executed_at: Option<DateTimeUtc>,
  pub executed_by: Option<Uuid>,
  pub reverted_at: Option<DateTimeUtc>,
  pub reverted_by: Option<Uuid>,
  pub contractor_id: Uuid,
  #[sea_orm(belongs_to, from = "contractor_id", to = "id")]
  pub contractor: HasOne<company::Entity>,
  pub target_product_id: Uuid,
  #[sea_orm(belongs_to, from = "target_product_id", to = "id")]
  pub target_product: HasOne<product::Entity>,
  #[sea_orm(has_many)]
  pub components: HasMany<blending_component::Entity>,
  #[sea_orm(has_many)]
  pub results: HasMany<blending_result::Entity>,
}

impl From<&CreateBlendingRequest> for ActiveModel {
  fn from(dto: &CreateBlendingRequest) -> Self {
    Self {
      document_number: Set(dto.document_number.clone()),
      date: Set(dto.date),
      status: Set(crate::enums::DocumentStatus::Draft),
      version: Set(1),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      contractor_id: Set(dto.contractor_id),
      target_product_id: Set(dto.target_product_id),
      ..Default::default()
    }
  }
}

impl From<&CreateBlendingCompositeRequest> for ActiveModel {
  fn from(dto: &CreateBlendingCompositeRequest) -> Self {
    Self {
      document_number: Set(dto.document_number.clone()),
      date: Set(dto.date),
      status: Set(crate::enums::DocumentStatus::Draft),
      version: Set(1),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      contractor_id: Set(dto.contractor_id),
      target_product_id: Set(dto.target_product_id),
      ..Default::default()
    }
  }
}
