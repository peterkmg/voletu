use crate::{
  dtos,
  entities::{company, product, product_group},
  services::{
    common::{set_if_some, set_if_some_mapped, validate_fk_exists, validate_optional_fk_exists},
    CatalogService,
  },
};

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
) -> Result<(), crate::api::ApiError> {
  validate_fk_exists::<product_group::Entity>(
    conn,
    req.product_group_id,
    product_group::Column::Id,
    product_group::Column::DeletedAt,
    "productGroupId",
  )
  .await?;

  validate_optional_fk_exists::<company::Entity>(
    conn,
    req.manufacturer_id,
    company::Column::Id,
    company::Column::DeletedAt,
    "manufacturerId",
  )
  .await?;
  Ok(())
}

async fn before_product_update(
  _svc: &CatalogService,
  conn: &impl sea_orm::ConnectionTrait,
  _existing: &product::Model,
  req: &dtos::UpdateProductRequest,
) -> Result<(), crate::api::ApiError> {
  if let Some(product_group_id) = req.product_group_id {
    validate_fk_exists::<product_group::Entity>(
      conn,
      product_group_id,
      product_group::Column::Id,
      product_group::Column::DeletedAt,
      "productGroupId",
    )
    .await?;
  }
  if let Some(manufacturer_id) = req.manufacturer_id {
    validate_fk_exists::<company::Entity>(
      conn,
      manufacturer_id,
      company::Column::Id,
      company::Column::DeletedAt,
      "manufacturerId",
    )
    .await?;
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
