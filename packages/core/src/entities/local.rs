use sea_orm::prelude::async_trait::async_trait;
use sea_orm::ConnectionTrait;
use sea_orm::{
  entity::prelude::*,
  model,
  ActiveValue::{NotSet, Set},
};

#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "local")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: i32, // always 1, enforced in code
  pub is_initialized: bool, // database_instance has been renamed and admin password has been changed from defaults
  #[sea_orm(unique)]
  pub local_db_id: Uuid,
  #[sea_orm(belongs_to, from = "local_db_id", to = "id")]
  pub database_instance: HasOne<super::database_instance::Entity>,
  pub jwt_secret: String, // rotating this value invalidates all existing tokens.
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  async fn before_save<C: ConnectionTrait>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr> {
    if insert && matches!(self.id, NotSet) {
      self.id = Set(1);
    }
    Ok(self)
  }
}
