use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::CreateAcceptanceRequest,
  entities::{acceptance_item, company, dispatch_document, rail_waybill, truck_waybill},
  enums,
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "acceptance_documents")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub document_number: String,
  pub date_accepted: DateTimeUtc,
  pub status: enums::DocumentStatus,
  pub version: i32,
  pub executed_at: Option<DateTimeUtc>,
  pub executed_by: Option<Uuid>,
  pub reverted_at: Option<DateTimeUtc>,
  pub reverted_by: Option<Uuid>,
  pub arrival_type: enums::ArrivalType,
  pub source_entity: Option<String>,
  pub contractor_id: Option<Uuid>,
  #[sea_orm(belongs_to, from = "contractor_id", to = "id")]
  pub contractor: HasOne<company::Entity>,
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

impl From<&CreateAcceptanceRequest> for ActiveModel {
  fn from(dto: &CreateAcceptanceRequest) -> Self {
    Self {
      document_number: Set(dto.document_number.clone()),
      date_accepted: Set(dto.date_accepted),
      status: Set(enums::DocumentStatus::Draft),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      arrival_type: Set(dto.arrival_type),
      source_entity: Set(dto.source_entity.clone()),
      contractor_id: Set(dto.contractor_id),
      truck_waybill_id: Set(dto.truck_waybill_id),
      rail_waybill_id: Set(dto.rail_waybill_id),
      transit_dispatch_id: Set(dto.transit_dispatch_id),
      ..Default::default()
    }
  }
}
