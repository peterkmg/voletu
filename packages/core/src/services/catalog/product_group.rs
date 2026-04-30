use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{product_group, product_type},
  services::{
    common::{set_if_some, set_if_some_mapped},
    CatalogService,
  },
};

async fn ensure_active_product_type(
  conn: &impl ConnectionTrait,
  product_type_id: Uuid,
  field_name: &str,
) -> Result<(), ApiError> {
  let exists = product_type::Entity::load()
    .filter_by_id(product_type_id)
    .filter(product_type::Column::DeletedAt.is_null())
    .one(conn)
    .await?;

  if exists.is_none() {
    return Err(ApiError::BadRequest(format!(
      "{field_name} '{product_type_id}' does not reference a valid record"
    )));
  }

  Ok(())
}

fn apply_product_group_update(
  model: &mut product_group::ActiveModel,
  req: &dtos::UpdateProductGroupRequest,
) {
  set_if_some(&mut model.product_type_id, req.product_type_id);
  set_if_some(&mut model.common_name, req.common_name.clone());
  set_if_some_mapped(&mut model.long_name, req.long_name.clone(), Some);
}

async fn before_product_group_create(
  _svc: &CatalogService,
  conn: &impl sea_orm::ConnectionTrait,
  req: &dtos::CreateProductGroupRequest,
) -> Result<(), ApiError> {
  ensure_active_product_type(conn, req.product_type_id, "productTypeId").await?;
  Ok(())
}

async fn before_product_group_update(
  _svc: &CatalogService,
  conn: &impl sea_orm::ConnectionTrait,
  _existing: &product_group::Model,
  req: &dtos::UpdateProductGroupRequest,
) -> Result<(), ApiError> {
  if let Some(product_type_id) = req.product_type_id {
    ensure_active_product_type(conn, product_type_id, "productTypeId").await?;
  }
  Ok(())
}

#[voletu_core_macros::entity_service(
  entity_name = "Product group",
  entity = product_group,
  entity_mod = product_group,
  create_req = dtos::CreateProductGroupRequest,
  update_req = dtos::UpdateProductGroupRequest,
  response = dtos::ProductGroupResponse,
  apply_update = apply_product_group_update,
  before_create = before_product_group_create,
  before_update = before_product_group_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl CatalogService {}
