use crate::{
  dtos,
  entities::rail_waybill,
  services::{common::set_if_some, document::DocumentService},
};

fn apply_rail_waybill_update(
  model: &mut rail_waybill::ActiveModel,
  req: &dtos::UpdateRailWaybillRequest,
) {
  set_if_some(&mut model.document_number, req.document_number.clone());
  set_if_some(&mut model.date, req.date);
  set_if_some(&mut model.sender_id, req.sender_id);
}

#[voletu_core_macros::entity_service(
  entity = rail_waybill,
  entity_mod = crate::entities::rail_waybill,
  create_req = crate::dtos::CreateRailWaybillRequest,
  update_req = crate::dtos::UpdateRailWaybillRequest,
  response = crate::dtos::RailWaybillResponse,
  apply_update = apply_rail_waybill_update,
  entity_name = "Rail waybill",
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
