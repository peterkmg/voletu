use sea_orm::{entity::prelude::*, model};

use crate::entities::{base, database_instance};

#[voletu_core_macros::handle_uuid]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "node_base_assignments")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub node_id: Uuid,
  #[sea_orm(belongs_to, from = "node_id", to = "id")]
  pub node: HasOne<database_instance::Entity>,
  pub base_id: Uuid,
  #[sea_orm(belongs_to, from = "base_id", to = "id")]
  pub base: HasOne<base::Entity>,
}
