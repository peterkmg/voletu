use crate::{
  dtos,
  entities::storage,
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

#[voletu_core_macros::entity_service(
  entity_name = "Storage",
  entity = storage,
  entity_mod = storage,
  create_req = dtos::CreateStorageRequest,
  update_req = dtos::UpdateStorageRequest,
  response = dtos::StorageResponse,
  apply_update = apply_storage_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl CatalogService {}
