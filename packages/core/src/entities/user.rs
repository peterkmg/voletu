use sea_orm::prelude::async_trait::async_trait;
use sea_orm::ConnectionTrait;
use sea_orm::{
  entity::prelude::*,
  model,
  ActiveValue::{NotSet, Set},
};
use uuid::Uuid;

use crate::dtos::user::UserResponse;

#[model]
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
  async fn before_save<C: ConnectionTrait>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr> {
    let now = ChronoUtc::now();

    if insert {
      if matches!(self.id, NotSet) {
        self.id = Set(Uuid::now_v7());
      }
      if matches!(self.created_at, NotSet) {
        self.created_at = Set(now);
      }
    }

    self.updated_at = Set(now);
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
      role: role.common_name.as_str().into(),
    })
  }
}
