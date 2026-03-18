use crate::{
  dtos,
  entities::product_group,
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

#[voletu_core_macros::entity_service(
  entity_name = "Product group",
  entity = product_group,
  entity_mod = product_group,
  create_req = dtos::CreateProductGroupRequest,
  update_req = dtos::UpdateProductGroupRequest,
  response = dtos::ProductGroupResponse,
  apply_update = apply_product_group_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl CatalogService {}
