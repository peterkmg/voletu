use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::CreateAcceptanceItemRequest,
  entities::{acceptance_document, company, product, storage},
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "acceptance_items")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub acceptance_doc_id: Uuid,
  #[sea_orm(belongs_to, from = "acceptance_doc_id", to = "id")]
  pub acceptance_doc: HasOne<acceptance_document::Entity>,
  pub product_id: Uuid,
  #[sea_orm(belongs_to, from = "product_id", to = "id")]
  pub product: HasOne<product::Entity>,
  pub contractor_id: Uuid,
  #[sea_orm(belongs_to, from = "contractor_id", to = "id")]
  pub contractor: HasOne<company::Entity>,
  pub storage_id: Uuid,
  #[sea_orm(belongs_to, from = "storage_id", to = "id")]
  pub storage: HasOne<storage::Entity>,
  pub accepted_amount: Decimal,
}

impl From<&CreateAcceptanceItemRequest> for ActiveModel {
  fn from(dto: &CreateAcceptanceItemRequest) -> Self {
    Self {
      acceptance_doc_id: Set(dto.acceptance_doc_id),
      product_id: Set(dto.item.product_id),
      contractor_id: Set(dto.item.contractor_id),
      storage_id: Set(dto.item.storage_id),
      accepted_amount: Set(dto.item.accepted_amount),
      ..Default::default()
    }
  }
}
