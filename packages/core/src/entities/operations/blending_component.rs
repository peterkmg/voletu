use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::CreateBlendingComponentRequest,
  entities::{blending_document, product, storage},
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "blending_components")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub blending_doc_id: Uuid,
  #[sea_orm(belongs_to, from = "blending_doc_id", to = "id")]
  pub blending_doc: HasOne<blending_document::Entity>,
  pub storage_id: Uuid,
  #[sea_orm(belongs_to, from = "storage_id", to = "id")]
  pub storage: HasOne<storage::Entity>,
  pub source_product_id: Uuid,
  #[sea_orm(belongs_to, from = "source_product_id", to = "id")]
  pub source_product: HasOne<product::Entity>,
  pub amount_used: Decimal,
}

impl From<&CreateBlendingComponentRequest> for ActiveModel {
  fn from(dto: &CreateBlendingComponentRequest) -> Self {
    Self {
      blending_doc_id: Set(dto.blending_doc_id),
      storage_id: Set(dto.component.storage_id),
      source_product_id: Set(dto.component.source_product_id),
      amount_used: Set(dto.component.amount_used),
      ..Default::default()
    }
  }
}
