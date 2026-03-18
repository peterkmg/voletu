use crate::{
  dtos,
  entities::base,
  services::{
    common::{set_if_some, set_if_some_mapped},
    CatalogService,
  },
};

fn apply_base_update(model: &mut base::ActiveModel, req: &dtos::UpdateBaseRequest) {
  set_if_some(&mut model.common_name, req.common_name.clone());
  set_if_some_mapped(&mut model.long_name, req.long_name.clone(), Some);
}

#[voletu_core_macros::entity_service(
  entity_name = "Base",
  entity = base,
  entity_mod = base,
  create_req = dtos::CreateBaseRequest,
  update_req = dtos::UpdateBaseRequest,
  response = dtos::BaseResponse,
  apply_update = apply_base_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl CatalogService {}
