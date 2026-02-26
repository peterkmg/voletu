use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{blending_document, storage};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "blending_results")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub blending_doc_id: Uuid,
  #[sea_orm(belongs_to, from = "blending_doc_id", to = "id")]
  pub blending_doc: HasOne<blending_document::Entity>,
  pub storage_id: Uuid,
  #[sea_orm(belongs_to, from = "storage_id", to = "id")]
  pub storage: HasOne<storage::Entity>,
  pub produced_amount: Decimal,
}
