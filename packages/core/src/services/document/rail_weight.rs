use sea_orm::ConnectionTrait;

use crate::{
  api::ApiError,
  dtos,
  entities::rail_wagon_weight,
  services::{
    common::set_if_some, document::rail_manifest::ensure_parent_manifest_active, DocumentService,
  },
};

fn apply_rail_weight_update(
  model: &mut rail_wagon_weight::ActiveModel,
  req: &dtos::UpdateRailWagonWeightRequest,
) {
  set_if_some(&mut model.gross_weight, req.gross_weight);
  set_if_some(&mut model.tare_weight, req.tare_weight);
  set_if_some(&mut model.net_product_weight, req.net_product_weight);
}

async fn ensure_rail_weight_update_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &rail_wagon_weight::Model,
  _req: &dtos::UpdateRailWagonWeightRequest,
) -> Result<(), ApiError> {
  ensure_parent_manifest_active(conn, existing.wagon_manifest_id).await
}

async fn ensure_rail_weight_soft_delete_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &rail_wagon_weight::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  ensure_parent_manifest_active(conn, existing.wagon_manifest_id).await
}

#[voletu_core_macros::entity_service(
  entity_name = "Rail wagon weight",
  entity = rail_weight,
  entity_mod = rail_wagon_weight,
  create_req = dtos::CreateRailWagonWeightRequest,
  update_req = dtos::UpdateRailWagonWeightRequest,
  response = dtos::RailWagonWeightResponse,
  apply_update = apply_rail_weight_update,
  before_update = ensure_rail_weight_update_allowed,
  before_soft_delete = ensure_rail_weight_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
