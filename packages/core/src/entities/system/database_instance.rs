use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::{entities::base, enums};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "database_instances")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub common_name: String,
  #[sea_orm(column_name = "type")]
  pub node_type: enums::NodeType,
  #[sea_orm(unique)]
  pub base_id: Option<Uuid>,
  #[sea_orm(belongs_to, from = "base_id", to = "id")]
  pub base: HasOne<base::Entity>,
}
