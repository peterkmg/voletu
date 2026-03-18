use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

use crate::{
  dtos::CreateDispatchRequest,
  entities::{base, company, dispatch_item, dispatch_storage_measurement, port},
  enums,
};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "dispatch_documents")]
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
  pub dispatch_purpose: enums::DispatchPurpose,
  pub dispatch_method: enums::DispatchMethod,
  pub contractor_id: Uuid,
  #[sea_orm(belongs_to, from = "contractor_id", to = "id")]
  pub contractor: HasOne<company::Entity>,
  pub destination_base_id: Option<Uuid>,
  #[sea_orm(belongs_to, from = "destination_base_id", to = "id")]
  pub destination_base: HasOne<base::Entity>,
  pub receiver_entity: Option<String>,
  pub start_cargo_ops: Option<DateTimeUtc>,
  pub end_cargo_ops: Option<DateTimeUtc>,
  pub bunker_type: Option<enums::BunkerType>,
  pub exporter_id: Option<Uuid>,
  pub port_id: Option<Uuid>,
  #[sea_orm(belongs_to, from = "port_id", to = "id")]
  pub port: HasOne<port::Entity>,
  #[sea_orm(has_many)]
  pub items: HasMany<dispatch_item::Entity>,
  #[sea_orm(has_many)]
  pub storage_measurements: HasMany<dispatch_storage_measurement::Entity>,
}

impl From<&CreateDispatchRequest> for ActiveModel {
  fn from(dto: &CreateDispatchRequest) -> Self {
    Self {
      document_number: Set(dto.document_number.clone()),
      date: Set(dto.date),
      status: Set(enums::DocumentStatus::Draft),
      executed_at: Set(None),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      dispatch_purpose: Set(dto.dispatch_purpose),
      dispatch_method: Set(dto.dispatch_method),
      contractor_id: Set(dto.contractor_id),
      destination_base_id: Set(dto.destination_base_id),
      receiver_entity: Set(dto.receiver_entity.clone()),
      start_cargo_ops: Set(dto.start_cargo_ops),
      end_cargo_ops: Set(dto.end_cargo_ops),
      bunker_type: Set(dto.bunker_type),
      exporter_id: Set(dto.exporter_id),
      port_id: Set(dto.port_id),
      ..Default::default()
    }
  }
}
