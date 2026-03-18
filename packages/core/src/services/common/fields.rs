use sea_orm::{
  prelude::{ChronoUtc, DateTimeUtc},
  ActiveValue,
  ActiveValue::Set,
  Value,
};
use uuid::Uuid;

pub fn set_if_some<T>(field: &mut ActiveValue<T>, value: Option<T>)
where
  T: Into<Value>,
{
  if let Some(value) = value {
    *field = Set(value);
  }
}

pub fn set_if_some_mapped<T, U>(field: &mut ActiveValue<U>, value: Option<T>, mapper: fn(T) -> U)
where
  U: Into<Value>,
{
  if let Some(value) = value {
    *field = Set(mapper(value));
  }
}

pub fn set_soft_deleted_fields(
  deleted_at: &mut ActiveValue<Option<DateTimeUtc>>,
  deleted_by: &mut ActiveValue<Option<Uuid>>,
  undo: bool,
  actor_id: Uuid,
) {
  let now = ChronoUtc::now();
  *deleted_at = Set(if undo { None } else { Some(now) });
  *deleted_by = Set(if undo { None } else { Some(actor_id) });
}
