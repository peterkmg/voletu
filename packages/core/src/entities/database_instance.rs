use sea_orm::prelude::async_trait::async_trait;
use sea_orm::ConnectionTrait;
use sea_orm::{
  entity::prelude::*,
  model,
  ActiveValue::{NotSet, Set},
};
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

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  async fn before_save<C: ConnectionTrait>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr> {
    if insert && matches!(self.id, NotSet) {
      self.id = Set(Uuid::now_v7());
    }
    Ok(self)
  }
}
