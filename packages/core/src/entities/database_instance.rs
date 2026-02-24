use sea_orm::{entity::prelude::*, model, ActiveValue::Set};
use uuid::Uuid;

#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "database_instances")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub common_name: String,
  #[sea_orm(column_name = "type")]
  pub node_type: String,
  #[sea_orm(unique)]
  pub base_id: Option<Uuid>,
}

impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(Uuid::now_v7()),
      ..Default::default()
    }
  }
}
