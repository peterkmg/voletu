use sea_orm::{entity::prelude::*, model, ActiveValue::Set};

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

impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(1), // singleton row — there is exactly one record, always with id = 1
      ..Default::default()
    }
  }
}
