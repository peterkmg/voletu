use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{acceptance_item, dispatch_document, enums, rail_waybill, truck_waybill};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "acceptance_documents")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub document_number: String,
  pub date_accepted: DateTimeUtc,
  pub arrival_type: enums::ArrivalType,
  pub source_entity: Option<String>,
  pub truck_waybill_id: Option<Uuid>,
  #[sea_orm(belongs_to, from = "truck_waybill_id", to = "id")]
  pub truck_waybill: HasOne<truck_waybill::Entity>,
  pub rail_waybill_id: Option<Uuid>,
  #[sea_orm(belongs_to, from = "rail_waybill_id", to = "id")]
  pub rail_waybill: HasOne<rail_waybill::Entity>,
  pub transit_dispatch_id: Option<Uuid>,
  #[sea_orm(belongs_to, from = "transit_dispatch_id", to = "id")]
  pub transit_dispatch: HasOne<dispatch_document::Entity>,
  #[sea_orm(has_many)]
  pub items: HasMany<acceptance_item::Entity>,
}
