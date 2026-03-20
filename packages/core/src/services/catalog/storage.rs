use crate::{
  dtos,
  entities::{product_type, storage, warehouse},
  services::{
    common::{set_if_some, set_if_some_mapped},
    CatalogService,
  },
};

fn apply_storage_update(model: &mut storage::ActiveModel, req: &dtos::UpdateStorageRequest) {
  set_if_some(&mut model.warehouse_id, req.warehouse_id);
  set_if_some(&mut model.common_name, req.common_name.clone());
  set_if_some_mapped(&mut model.long_name, req.long_name.clone(), Some);
  set_if_some_mapped(&mut model.capacity, req.capacity, Some);
  set_if_some(&mut model.is_type_specific, req.is_type_specific);
  set_if_some_mapped(&mut model.product_type_id, req.product_type_id, Some);
}

async fn before_storage_create(
  _svc: &CatalogService,
  conn: &impl sea_orm::ConnectionTrait,
  req: &dtos::CreateStorageRequest,
) -> Result<(), crate::api::ApiError> {
  crate::services::common::validate_fk_exists::<warehouse::Entity>(
    conn,
    req.warehouse_id,
    warehouse::Column::Id,
    warehouse::Column::DeletedAt,
    "warehouseId",
  )
  .await?;
  crate::services::common::validate_optional_fk_exists::<product_type::Entity>(
    conn,
    req.product_type_id,
    product_type::Column::Id,
    product_type::Column::DeletedAt,
    "productTypeId",
  )
  .await?;
  Ok(())
}

async fn before_storage_update(
  _svc: &CatalogService,
  conn: &impl sea_orm::ConnectionTrait,
  _existing: &storage::Model,
  req: &dtos::UpdateStorageRequest,
) -> Result<(), crate::api::ApiError> {
  if let Some(warehouse_id) = req.warehouse_id {
    crate::services::common::validate_fk_exists::<warehouse::Entity>(
      conn,
      warehouse_id,
      warehouse::Column::Id,
      warehouse::Column::DeletedAt,
      "warehouseId",
    )
    .await?;
  }
  if let Some(product_type_id) = req.product_type_id {
    crate::services::common::validate_fk_exists::<product_type::Entity>(
      conn,
      product_type_id,
      product_type::Column::Id,
      product_type::Column::DeletedAt,
      "productTypeId",
    )
    .await?;
  }
  Ok(())
}

#[voletu_core_macros::entity_service(
  entity_name = "Storage",
  entity = storage,
  entity_mod = storage,
  create_req = dtos::CreateStorageRequest,
  update_req = dtos::UpdateStorageRequest,
  response = dtos::StorageResponse,
  apply_update = apply_storage_update,
  before_create = before_storage_create,
  before_update = before_storage_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl CatalogService {}
