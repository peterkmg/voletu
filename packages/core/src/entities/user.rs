use sea_orm::prelude::async_trait::async_trait;
use sea_orm::ConnectionTrait;
use sea_orm::{entity::prelude::*, ActiveValue::Set};
use uuid::Uuid;

use crate::dtos::user::UserResponse;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub username: String,
  pub fullname: Option<String>,
  pub password_hash: String,
  pub role_id: Uuid,
  #[sea_orm(belongs_to, from = "role_id", to = "id")]
  pub role: HasOne<super::role::Entity>,
  pub created_at: DateTimeUtc,
  pub updated_at: DateTimeUtc,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(Uuid::now_v7()),
      created_at: Set(ChronoUtc::now()),
      updated_at: Set(ChronoUtc::now()),
      ..Default::default()
    }
  }

  async fn before_save<C: ConnectionTrait>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr> {
    if !insert {
      self.updated_at = Set(ChronoUtc::now());
    }
    Ok(self)
  }
}

impl ModelEx {
  pub fn to_user_response(&self) -> anyhow::Result<UserResponse> {
    let role = self
      .role
      .as_ref()
      .ok_or(anyhow::anyhow!("User role not found"))?;

    Ok(UserResponse {
      id: self.id,
      username: self.username.clone(),
      fullname: self.fullname.clone(),
      role: role.name.clone(),
    })
  }
}
