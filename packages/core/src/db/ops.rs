use std::{future::Future, pin::Pin};

use sea_orm::{
  DatabaseConnection,
  DatabaseTransaction,
  EntityTrait,
  PrimaryKeyTrait,
  TransactionTrait,
};

use crate::{api::ApiError, entities::local};

pub async fn with_transaction<T, F>(db: &DatabaseConnection, f: F) -> Result<T, ApiError>
where
  F: for<'a> FnOnce(
    &'a DatabaseTransaction,
  ) -> Pin<Box<dyn Future<Output = Result<T, ApiError>> + 'a>>,
{
  let txn = db.begin().await.map_err(ApiError::Database)?;
  let output = f(&txn).await?;
  txn.commit().await.map_err(ApiError::Database)?;
  Ok(output)
}

pub fn require_found<T>(value: Option<T>, message: impl Into<String>) -> Result<T, ApiError> {
  value.ok_or_else(|| ApiError::NotFound(message.into()))
}

pub async fn find_by_id_required<E>(
  conn: &impl sea_orm::ConnectionTrait,
  id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
  message: impl Into<String>,
) -> Result<E::Model, ApiError>
where
  E: EntityTrait,
{
  let model = E::find_by_id(id)
    .one(conn)
    .await
    .map_err(ApiError::Database)?;
  require_found(model, message)
}

pub async fn list_all<E>(conn: &impl sea_orm::ConnectionTrait) -> Result<Vec<E::Model>, ApiError>
where
  E: EntityTrait,
{
  E::find().all(conn).await.map_err(ApiError::Database)
}

pub async fn exists_by_id<E>(
  conn: &impl sea_orm::ConnectionTrait,
  id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
) -> Result<bool, ApiError>
where
  E: EntityTrait,
{
  let model = E::find_by_id(id)
    .one(conn)
    .await
    .map_err(ApiError::Database)?;
  Ok(model.is_some())
}

pub async fn load_local_bootstrap(
  conn: &impl sea_orm::ConnectionTrait,
) -> Result<local::Model, ApiError> {
  let local = local::Entity::find_by_id(1)
    .one(conn)
    .await
    .map_err(ApiError::Database)?;
  require_found(local, "Local bootstrap row is missing")
}
