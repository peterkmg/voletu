use crate::{
  dtos,
  entities::warehouse,
  services::{
    common::{set_if_some, set_if_some_mapped},
    CatalogService,
  },
};

fn apply_warehouse_update(model: &mut warehouse::ActiveModel, req: &dtos::UpdateWarehouseRequest) {
  set_if_some(&mut model.base_id, req.base_id);
  set_if_some(&mut model.common_name, req.common_name.clone());
  set_if_some_mapped(&mut model.long_name, req.long_name.clone(), Some);
}

#[voletu_core_macros::entity_service(
  entity_name = "Warehouse",
  entity = warehouse,
  entity_mod = warehouse,
  create_req = dtos::CreateWarehouseRequest,
  update_req = dtos::UpdateWarehouseRequest,
  response = dtos::WarehouseResponse,
  apply_update = apply_warehouse_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl CatalogService {}
