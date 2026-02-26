use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{dispatch_document, storage};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dispatch_storage_measurements")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub dispatch_doc_id: Uuid,
  #[sea_orm(belongs_to, from = "dispatch_doc_id", to = "id")]
  pub dispatch_doc: HasOne<dispatch_document::Entity>,
  pub storage_id: Uuid,
  #[sea_orm(belongs_to, from = "storage_id", to = "id")]
  pub storage: HasOne<storage::Entity>,
  pub before_height: Option<Decimal>,
  pub before_volume: Option<Decimal>,
  pub before_density: Option<Decimal>,
  pub before_mass: Decimal,
  pub after_height: Option<Decimal>,
  pub after_volume: Option<Decimal>,
  pub after_density: Option<Decimal>,
  pub after_mass: Decimal,
}
