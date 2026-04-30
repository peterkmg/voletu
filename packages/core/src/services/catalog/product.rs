use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{company, product, product_group},
  services::{
    common::{set_if_some, set_if_some_mapped},
    CatalogService,
  },
};

async fn ensure_active_product_group(
  conn: &impl ConnectionTrait,
  product_group_id: Uuid,
  field_name: &str,
) -> Result<(), ApiError> {
  let exists = product_group::Entity::load()
    .filter_by_id(product_group_id)
    .filter(product_group::Column::DeletedAt.is_null())
    .one(conn)
    .await?;

  if exists.is_none() {
    return Err(ApiError::BadRequest(format!(
      "{field_name} '{product_group_id}' does not reference a valid record"
    )));
  }

  Ok(())
}

async fn ensure_active_company(
  conn: &impl ConnectionTrait,
  company_id: Uuid,
  field_name: &str,
) -> Result<(), ApiError> {
  let exists = company::Entity::load()
    .filter_by_id(company_id)
    .filter(company::Column::DeletedAt.is_null())
    .one(conn)
    .await?;

  if exists.is_none() {
    return Err(ApiError::BadRequest(format!(
      "{field_name} '{company_id}' does not reference a valid record"
    )));
  }

  Ok(())
}

fn apply_product_update(model: &mut product::ActiveModel, req: &dtos::UpdateProductRequest) {
  set_if_some(&mut model.product_group_id, req.product_group_id);
  set_if_some_mapped(&mut model.manufacturer_id, req.manufacturer_id, Some);
  set_if_some(&mut model.common_name, req.common_name.clone());
  set_if_some_mapped(&mut model.long_name, req.long_name.clone(), Some);
  set_if_some_mapped(
    &mut model.add_identification,
    req.add_identification.clone(),
    Some,
  );
  set_if_some(&mut model.is_component, req.is_component);
}

async fn before_product_create(
  _svc: &CatalogService,
  conn: &impl sea_orm::ConnectionTrait,
  req: &dtos::CreateProductRequest,
) -> Result<(), ApiError> {
  ensure_active_product_group(conn, req.product_group_id, "productGroupId").await?;
  if let Some(manufacturer_id) = req.manufacturer_id {
    ensure_active_company(conn, manufacturer_id, "manufacturerId").await?;
  }
  Ok(())
}

async fn before_product_update(
  _svc: &CatalogService,
  conn: &impl sea_orm::ConnectionTrait,
  _existing: &product::Model,
  req: &dtos::UpdateProductRequest,
) -> Result<(), ApiError> {
  if let Some(product_group_id) = req.product_group_id {
    ensure_active_product_group(conn, product_group_id, "productGroupId").await?;
  }
  if let Some(manufacturer_id) = req.manufacturer_id {
    ensure_active_company(conn, manufacturer_id, "manufacturerId").await?;
  }
  Ok(())
}

#[voletu_core_macros::entity_service(
  entity_name = "Product",
  entity = product,
  entity_mod = product,
  create_req = dtos::CreateProductRequest,
  update_req = dtos::UpdateProductRequest,
  response = dtos::ProductResponse,
  apply_update = apply_product_update,
  before_create = before_product_create,
  before_update = before_product_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl CatalogService {}
