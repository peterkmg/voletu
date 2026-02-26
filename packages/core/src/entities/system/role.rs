use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{enums, user};

#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "roles")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub common_name: enums::RoleType,
  #[sea_orm(has_many)]
  pub users: HasMany<user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
