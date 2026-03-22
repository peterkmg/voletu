use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  services::{common::resolve_names, CatalogService},
};

impl CatalogService {
  pub async fn product_list_with_names(
    &self,
    pagination: Option<(u64, u64)>,
  ) -> Result<Vec<dtos::ProductResponse>, ApiError> {
    let mut items = self.product_list(pagination).await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn product_get_with_names(&self, id: Uuid) -> Result<dtos::ProductResponse, ApiError> {
    let mut item = self.product_get(id).await?;
    resolve_names(self.db.as_ref(), std::slice::from_mut(&mut item)).await?;
    Ok(item)
  }

  pub async fn product_group_list_with_names(
    &self,
    pagination: Option<(u64, u64)>,
  ) -> Result<Vec<dtos::ProductGroupResponse>, ApiError> {
    let mut items = self.product_group_list(pagination).await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn product_group_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::ProductGroupResponse, ApiError> {
    let mut item = self.product_group_get(id).await?;
    resolve_names(self.db.as_ref(), std::slice::from_mut(&mut item)).await?;
    Ok(item)
  }

  pub async fn warehouse_list_with_names(
    &self,
    pagination: Option<(u64, u64)>,
  ) -> Result<Vec<dtos::WarehouseResponse>, ApiError> {
    let mut items = self.warehouse_list(pagination).await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn warehouse_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::WarehouseResponse, ApiError> {
    let mut item = self.warehouse_get(id).await?;
    resolve_names(self.db.as_ref(), std::slice::from_mut(&mut item)).await?;
    Ok(item)
  }

  pub async fn storage_list_with_names(
    &self,
    pagination: Option<(u64, u64)>,
  ) -> Result<Vec<dtos::StorageResponse>, ApiError> {
    let mut items = self.storage_list(pagination).await?;
    resolve_names(self.db.as_ref(), &mut items).await?;
    Ok(items)
  }

  pub async fn storage_get_with_names(&self, id: Uuid) -> Result<dtos::StorageResponse, ApiError> {
    let mut item = self.storage_get(id).await?;
    resolve_names(self.db.as_ref(), std::slice::from_mut(&mut item)).await?;
    Ok(item)
  }
}
