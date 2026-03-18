use crate::{
  dtos,
  entities::product_type,
  services::{
    common::{set_if_some, set_if_some_mapped},
    CatalogService,
  },
};

fn apply_product_type_update(
  model: &mut product_type::ActiveModel,
  req: &dtos::UpdateProductTypeRequest,
) {
  set_if_some(&mut model.common_name, req.common_name.clone());
  set_if_some_mapped(&mut model.long_name, req.long_name.clone(), Some);
}

#[voletu_core_macros::entity_service(
  entity_name = "Product type",
  entity = product_type,
  entity_mod = product_type,
  create_req = dtos::CreateProductTypeRequest,
  update_req = dtos::UpdateProductTypeRequest,
  response = dtos::ProductTypeResponse,
  apply_update = apply_product_type_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl CatalogService {}
