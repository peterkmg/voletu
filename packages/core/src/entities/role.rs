use sea_orm::{entity::prelude::*, ActiveValue::Set};
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "roles")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub name: String,
  #[sea_orm(has_many)]
  pub users: HasMany<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(Uuid::now_v7()),
      ..Default::default()
    }
  }
}
