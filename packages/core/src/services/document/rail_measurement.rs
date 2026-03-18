use sea_orm::ConnectionTrait;

use crate::{
  api::ApiError,
  dtos,
  entities::rail_wagon_measurement,
  services::{
    common::{set_if_some, set_if_some_mapped},
    document::rail_manifest::ensure_parent_manifest_active,
    DocumentService,
  },
};

fn apply_rail_measurement_update(
  model: &mut rail_wagon_measurement::ActiveModel,
  req: &dtos::UpdateRailWagonMeasurementRequest,
) {
  set_if_some(&mut model.measured_height, req.measured_height);
  set_if_some_mapped(&mut model.lab_density, req.lab_density, Some);
  set_if_some(&mut model.calculated_mass, req.calculated_mass);
}

async fn ensure_rail_measurement_update_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &rail_wagon_measurement::Model,
  _req: &dtos::UpdateRailWagonMeasurementRequest,
) -> Result<(), ApiError> {
  ensure_parent_manifest_active(conn, existing.wagon_manifest_id).await
}

async fn ensure_rail_measurement_soft_delete_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &rail_wagon_measurement::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  ensure_parent_manifest_active(conn, existing.wagon_manifest_id).await
}

#[voletu_core_macros::entity_service(
  entity_name = "Rail wagon measurement",
  entity = rail_measurement,
  entity_mod = rail_wagon_measurement,
  create_req = dtos::CreateRailWagonMeasurementRequest,
  update_req = dtos::UpdateRailWagonMeasurementRequest,
  response = dtos::RailWagonMeasurementResponse,
  apply_update = apply_rail_measurement_update,
  before_update = ensure_rail_measurement_update_allowed,
  before_soft_delete = ensure_rail_measurement_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
