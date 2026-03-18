use crate::{
  api::ApiError,
  dtos,
  entities::truck_waybill_item,
  services::{
    common::set_if_some, document::truck_waybill::ensure_parent_waybill_active, DocumentService,
  },
};

fn apply_truck_waybill_item_update(
  model: &mut truck_waybill_item::ActiveModel,
  req: &dtos::UpdateTruckWaybillItemRequest,
) {
  set_if_some(&mut model.product_id, req.product_id);
  set_if_some(&mut model.declared_amount, req.declared_amount);
}

async fn ensure_truck_waybill_item_update_allowed(
  _svc: &DocumentService,
  txn: &impl sea_orm::ConnectionTrait,
  existing: &truck_waybill_item::Model,
  _req: &dtos::UpdateTruckWaybillItemRequest,
) -> Result<(), ApiError> {
  ensure_parent_waybill_active(txn, existing.truck_waybill_id).await
}

async fn ensure_truck_waybill_item_soft_delete_allowed(
  _svc: &DocumentService,
  txn: &impl sea_orm::ConnectionTrait,
  existing: &truck_waybill_item::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  ensure_parent_waybill_active(txn, existing.truck_waybill_id).await
}

#[voletu_core_macros::entity_service(
  entity_name = "Truck waybill item",
  entity = truck_waybill_item,
  entity_mod = truck_waybill_item,
  create_req = dtos::CreateTruckWaybillItemRequest,
  update_req = dtos::UpdateTruckWaybillItemRequest,
  response = dtos::TruckWaybillItemResponse,
  apply_update = apply_truck_waybill_item_update,
  before_update = ensure_truck_waybill_item_update_allowed,
  before_soft_delete = ensure_truck_waybill_item_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
