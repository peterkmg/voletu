use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{blending_component, blending_result, company, product};

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
