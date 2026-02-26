use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{company, truck_waybill_item, truck_weight_doc};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "truck_waybills")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub document_number: String,
  pub date: Date,
  pub sender_id: Uuid,
  #[sea_orm(belongs_to, from = "sender_id", to = "id")]
  pub sender: HasOne<company::Entity>,
  #[sea_orm(has_many)]
  pub items: HasMany<truck_waybill_item::Entity>,
  #[sea_orm(has_many)]
  pub weight_docs: HasMany<truck_weight_doc::Entity>,
}
