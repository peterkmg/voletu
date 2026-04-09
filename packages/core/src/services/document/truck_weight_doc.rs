use sea_orm::ConnectionTrait;

use crate::{
  api::ApiError,
  dtos,
  entities::truck_weight_doc,
  services::{
    common::set_if_some, document::truck_waybill::ensure_parent_waybill_active, DocumentService,
  },
};

fn apply_truck_weight_doc_update(
  model: &mut truck_weight_doc::ActiveModel,
  req: &dtos::UpdateTruckWeightDocRequest,
) {
  set_if_some(&mut model.total_weight, req.total_weight);
}

async fn ensure_truck_weight_doc_update_allowed(
  _svc: &DocumentService,
  txn: &impl ConnectionTrait,
  existing: &truck_weight_doc::Model,
  _req: &dtos::UpdateTruckWeightDocRequest,
) -> Result<(), ApiError> {
  ensure_parent_waybill_active(txn, existing.truck_waybill_id).await
}

async fn ensure_truck_weight_doc_soft_delete_allowed(
  _svc: &DocumentService,
  txn: &impl ConnectionTrait,
  existing: &truck_weight_doc::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  ensure_parent_waybill_active(txn, existing.truck_waybill_id).await
}

#[voletu_core_macros::entity_service(
  entity_name = "Truck weight doc",
  entity = truck_weight_doc,
  entity_mod = truck_weight_doc,
  create_req = dtos::CreateTruckWeightDocRequest,
  update_req = dtos::UpdateTruckWeightDocRequest,
  response = dtos::TruckWeightDocResponse,
  apply_update = apply_truck_weight_doc_update,
  before_update = ensure_truck_weight_doc_update_allowed,
  before_soft_delete = ensure_truck_weight_doc_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
