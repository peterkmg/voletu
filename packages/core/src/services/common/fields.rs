use sea_orm::{ActiveValue, ActiveValue::Set, Value};

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
