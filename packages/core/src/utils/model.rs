use sea_orm::{
  ActiveValue,
  ActiveValue::{NotSet, Set, Unchanged},
  Value,
};
use uuid::Uuid;

#[macro_export]
macro_rules! audit_fields {
  () => {
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub deleted_by: Option<Uuid>,
    pub origin_db_id: Uuid,
  };
}

#[macro_export]
macro_rules! audit_fields_no_soft_delete {
  () => {
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub origin_db_id: Uuid,
  };
}

pub fn set_on_insert<T>(field: &mut ActiveValue<T>, insert: bool, value: T)
where
  T: Into<Value>,
{
  if insert && matches!(field, NotSet) {
    *field = Set(value);
  }
}

pub fn set<T>(field: &mut ActiveValue<T>, value: T)
where
  T: Into<Value>,
{
  *field = Set(value);
}

pub fn set_if_not_set_or_unchanged<T>(field: &mut ActiveValue<T>, value: T)
where
  T: Into<Value>,
{
  if matches!(field, NotSet | Unchanged(_)) {
    *field = Set(value);
  }
}

pub fn apply_uuid_on_insert(id: &mut ActiveValue<Uuid>, insert: bool) {
  set_on_insert(id, insert, Uuid::now_v7());
}
