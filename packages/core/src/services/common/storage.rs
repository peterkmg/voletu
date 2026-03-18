use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  entities::{product, product_group, storage},
};

pub async fn ensure_storage_accepts_product(
  conn: &impl ConnectionTrait,
  stor_id: Uuid,
  prod_id: Uuid,
) -> Result<(), ApiError> {
  let stor = storage::Entity::load()
    .filter_by_id(stor_id)
    .filter(storage::Column::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Storage '{}' not found", stor_id)))?;

  if let Some(expected_type) = stor.product_type_id {
    let prod = product::Entity::load()
      .filter_by_id(prod_id)
      .filter(product::Column::DeletedAt.is_null())
      .with(product_group::Entity)
      .one(conn)
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Product '{}' not found", prod_id)))?;

    let pg = prod.product_group.as_ref().ok_or_else(|| {
      ApiError::NotFound(format!("Product group for product '{}' not found", prod_id))
    })?;

    if expected_type != pg.product_type_id {
      return Err(ApiError::Validation(
        "Storage type restriction violated for product".to_string(),
      ));
    }
  }

  Ok(())
}
