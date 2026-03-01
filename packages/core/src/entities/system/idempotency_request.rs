use sea_orm::{entity::prelude::*, model, ActiveValue::Set, ConnectionTrait};

#[voletu_core_macros::handle_uuid(before_save = idempotency_request_before_save)]
#[model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "idempotency_requests")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[sea_orm(unique)]
  pub request_key: String,
  pub created_at: DateTimeUtc,
}

pub async fn idempotency_request_before_save<C: ConnectionTrait>(
  mut model: ActiveModel,
  _db: &C,
  insert: bool,
) -> Result<ActiveModel, DbErr> {
  if insert {
    model.created_at = Set(ChronoUtc::now());
  }
  Ok(model)
}
