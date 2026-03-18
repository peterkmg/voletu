use crate::{
  dtos,
  entities::port,
  services::{
    common::{set_if_some, set_if_some_mapped},
    CatalogService,
  },
};

fn apply_port_update(model: &mut port::ActiveModel, req: &dtos::UpdatePortRequest) {
  set_if_some(&mut model.common_name, req.common_name.clone());
  set_if_some_mapped(&mut model.country, req.country.clone(), Some);
}

#[voletu_core_macros::entity_service(
  entity_name = "Port",
  entity = port,
  entity_mod = port,
  create_req = dtos::CreatePortRequest,
  update_req = dtos::UpdatePortRequest,
  response = dtos::PortResponse,
  apply_update = apply_port_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl CatalogService {}
