use crate::{
  dtos,
  entities::{product_group, product_type},
  services::{
    common::{set_if_some, set_if_some_mapped},
    CatalogService,
  },
};

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
) -> Result<(), crate::api::ApiError> {
  crate::services::common::validate_fk_exists::<product_type::Entity>(
    conn,
    req.product_type_id,
    product_type::Column::Id,
    product_type::Column::DeletedAt,
    "productTypeId",
  )
  .await?;
  Ok(())
}

async fn before_product_group_update(
  _svc: &CatalogService,
  conn: &impl sea_orm::ConnectionTrait,
  _existing: &product_group::Model,
  req: &dtos::UpdateProductGroupRequest,
) -> Result<(), crate::api::ApiError> {
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
