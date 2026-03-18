use sea_orm::{entity::prelude::*, model};
use uuid::Uuid;

use crate::entities::{base, refresh_token, role};

#[voletu_core_macros::handle_audit]
#[voletu_core_macros::handle_service_fields]
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
  #[serde(skip_serializing, skip_deserializing)]
  #[sea_orm(belongs_to, from = "role_id", to = "id")]
  pub role: HasOne<role::Entity>,
  #[serde(skip_serializing, skip_deserializing)]
  #[sea_orm(has_many)]
  pub refresh_tokens: HasMany<refresh_token::Entity>,
  pub home_base_id: Option<Uuid>,
  #[serde(skip_serializing, skip_deserializing)]
  #[sea_orm(belongs_to, from = "home_base_id", to = "id")]
  pub home_base: HasOne<base::Entity>,
}
