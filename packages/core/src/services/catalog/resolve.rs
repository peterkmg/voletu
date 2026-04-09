use sea_orm::{ColumnTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{base, company, product, product_group, product_type, storage, warehouse},
  services::CatalogService,
};

impl CatalogService {
  pub async fn product_list_with_names(
    &self,
    pagination: Option<(u64, u64)>,
  ) -> Result<Vec<dtos::ProductResponse>, ApiError> {
    let query = product::Entity::load()
      .filter(product::Column::DeletedAt.is_null())
      .with(product_group::Entity)
      .with(company::Entity);

    let items: Vec<product::ModelEx> = if let Some((page, per_page)) = pagination {
      query
        .paginate(self.db.as_ref(), per_page)
        .fetch_page(page - 1)
        .await?
    } else {
      query.all(self.db.as_ref()).await?
    };

    Ok(items.into_iter().map(Into::into).collect())
  }

  pub async fn product_get_with_names(&self, id: Uuid) -> Result<dtos::ProductResponse, ApiError> {
    let item = product::Entity::load()
      .filter_by_id(id)
      .filter(product::Column::DeletedAt.is_null())
      .with(product_group::Entity)
      .with(company::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Product '{}' not found", id)))?;

    Ok(item.into())
  }

  pub async fn product_group_list_with_names(
    &self,
    pagination: Option<(u64, u64)>,
  ) -> Result<Vec<dtos::ProductGroupResponse>, ApiError> {
    let query = product_group::Entity::load()
      .filter(product_group::Column::DeletedAt.is_null())
      .with(product_type::Entity);

    let items: Vec<product_group::ModelEx> = if let Some((page, per_page)) = pagination {
      query
        .paginate(self.db.as_ref(), per_page)
        .fetch_page(page - 1)
        .await?
    } else {
      query.all(self.db.as_ref()).await?
    };

    Ok(items.into_iter().map(Into::into).collect())
  }

  pub async fn product_group_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::ProductGroupResponse, ApiError> {
    let item = product_group::Entity::load()
      .filter_by_id(id)
      .filter(product_group::Column::DeletedAt.is_null())
      .with(product_type::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Product group '{}' not found", id)))?;

    Ok(item.into())
  }

  pub async fn warehouse_list_with_names(
    &self,
    pagination: Option<(u64, u64)>,
  ) -> Result<Vec<dtos::WarehouseResponse>, ApiError> {
    let query = warehouse::Entity::load()
      .filter(warehouse::Column::DeletedAt.is_null())
      .with(base::Entity);

    let items: Vec<warehouse::ModelEx> = if let Some((page, per_page)) = pagination {
      query
        .paginate(self.db.as_ref(), per_page)
        .fetch_page(page - 1)
        .await?
    } else {
      query.all(self.db.as_ref()).await?
    };

    Ok(items.into_iter().map(Into::into).collect())
  }

  pub async fn warehouse_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::WarehouseResponse, ApiError> {
    let item = warehouse::Entity::load()
      .filter_by_id(id)
      .filter(warehouse::Column::DeletedAt.is_null())
      .with(base::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Warehouse '{}' not found", id)))?;

    Ok(item.into())
  }

  pub async fn storage_list_with_names(
    &self,
    pagination: Option<(u64, u64)>,
  ) -> Result<Vec<dtos::StorageResponse>, ApiError> {
    let query = storage::Entity::load()
      .filter(storage::Column::DeletedAt.is_null())
      .with(warehouse::Entity)
      .with(product_type::Entity);

    let items: Vec<storage::ModelEx> = if let Some((page, per_page)) = pagination {
      query
        .paginate(self.db.as_ref(), per_page)
        .fetch_page(page - 1)
        .await?
    } else {
      query.all(self.db.as_ref()).await?
    };

    Ok(items.into_iter().map(Into::into).collect())
  }

  pub async fn storage_get_with_names(&self, id: Uuid) -> Result<dtos::StorageResponse, ApiError> {
    let item = storage::Entity::load()
      .filter_by_id(id)
      .filter(storage::Column::DeletedAt.is_null())
      .with(warehouse::Entity)
      .with(product_type::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Storage '{}' not found", id)))?;

    Ok(item.into())
  }
}
