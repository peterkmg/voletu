use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::truck_waybill;

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "truck_weight_docs")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub truck_waybill_id: Uuid,
  #[sea_orm(belongs_to, from = "truck_waybill_id", to = "id")]
  pub truck_waybill: HasOne<truck_waybill::Entity>,
  pub total_weight: Decimal,
}
