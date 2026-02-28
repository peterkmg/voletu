use anyhow::anyhow;
use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::{
  dtos::UserResponse,
  entities::{base, refresh_token, role},
};

#[voletu_core_macros::with_audit_fields]
#[voletu_core_macros::handle_uuid_timestamps]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub username: String,
  pub fullname: Option<String>,
  pub password_hash: String,
  pub role_id: Uuid,
  #[sea_orm(belongs_to, from = "role_id", to = "id")]
  pub role: HasOne<role::Entity>,
  #[sea_orm(has_many)]
  pub refresh_tokens: HasMany<refresh_token::Entity>,
  pub home_base_id: Option<Uuid>,
  #[sea_orm(belongs_to, from = "home_base_id", to = "id")]
  pub home_base: HasOne<base::Entity>,
}

impl ModelEx {
  pub fn to_user_response(&self) -> anyhow::Result<UserResponse> {
    let role = self.role.as_ref().ok_or(anyhow!("User role not found"))?;

    Ok(UserResponse {
      id: self.id,
      username: self.username.clone(),
      fullname: self.fullname.clone(),
      role: role.common_name.as_str().into(),
    })
  }
}
