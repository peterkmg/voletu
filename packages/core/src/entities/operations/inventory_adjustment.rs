use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{company, enums, inventory_reconciliation, product, storage};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "inventory_adjustments")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub reconciliation_id: Uuid,
  #[sea_orm(belongs_to, from = "reconciliation_id", to = "id")]
  pub reconciliation: HasOne<inventory_reconciliation::Entity>,
  pub storage_id: Uuid,
  #[sea_orm(belongs_to, from = "storage_id", to = "id")]
  pub storage: HasOne<storage::Entity>,
  pub product_id: Uuid,
  #[sea_orm(belongs_to, from = "product_id", to = "id")]
  pub product: HasOne<product::Entity>,
  pub contractor_id: Uuid,
  #[sea_orm(belongs_to, from = "contractor_id", to = "id")]
  pub contractor: HasOne<company::Entity>,
  pub adjustment_type: enums::AdjustmentType,
  pub amount: Decimal,
  pub reason: Option<String>,
}
