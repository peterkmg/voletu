use sea_orm::{
  entity::prelude::*,
  model,
  ActiveValue::{NotSet, Set},
  ConnectionTrait,
};

use crate::entities::user;

#[voletu_core_macros::handle_service_fields(before_save = refresh_token_before_save)]
#[model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "refresh_tokens")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub user_id: Uuid,
  #[sea_orm(belongs_to, from = "user_id", to = "id")]
  pub user: HasOne<user::Entity>,
  #[sea_orm(unique)]
  pub token_hash: String,
  pub expires_at: DateTimeUtc,
  pub is_revoked: bool,
  pub device_info: Option<String>,
  pub created_at: DateTimeUtc,
  pub updated_at: DateTimeUtc,
}

pub async fn refresh_token_before_save<C: ConnectionTrait>(
  mut model: ActiveModel,
  _db: &C,
  insert: bool,
) -> Result<ActiveModel, DbErr> {
  if insert && matches!(model.is_revoked, NotSet) {
    model.is_revoked = Set(false);
  }
  Ok(model)
}
