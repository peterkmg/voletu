use crate::{
  dtos,
  entities::product,
  services::{
    common::{set_if_some, set_if_some_mapped},
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

#[voletu_core_macros::entity_service(
  entity_name = "Product",
  entity = product,
  entity_mod = product,
  create_req = dtos::CreateProductRequest,
  update_req = dtos::UpdateProductRequest,
  response = dtos::ProductResponse,
  apply_update = apply_product_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl CatalogService {}
