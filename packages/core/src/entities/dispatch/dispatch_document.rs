use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{base, company, dispatch_item, dispatch_storage_measurement, enums, port};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "dispatch_documents")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub document_number: String,
  pub date: DateTimeUtc,
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
