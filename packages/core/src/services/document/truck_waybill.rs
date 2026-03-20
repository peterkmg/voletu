use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};

use crate::{
  api::ApiError,
  dtos,
  entities::truck_waybill,
  services::{common::set_if_some, DocumentService},
};

pub(super) async fn ensure_parent_waybill_active(
  conn: &impl ConnectionTrait,
  truck_waybill_id: uuid::Uuid,
) -> Result<(), ApiError> {
  let doc = truck_waybill::Entity::load()
    .filter_by_id(truck_waybill_id)
    .filter(truck_waybill::Column::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Truck waybill '{}' not found", truck_waybill_id)))?;

  match doc.deleted_at {
    Some(_) => Err(ApiError::BadRequest(format!(
      "Cannot modify item of deleted truck waybill '{}'",
      truck_waybill_id
    ))),
    None => Ok(()),
  }
}

fn apply_truck_waybill_update(
  model: &mut truck_waybill::ActiveModel,
  req: &dtos::UpdateTruckWaybillRequest,
) {
  set_if_some(&mut model.document_number, req.document_number.clone());
  set_if_some(&mut model.date, req.date);
  set_if_some(&mut model.sender_id, req.sender_id);
}

#[voletu_core_macros::entity_service(
  entity_name = "Truck waybill",
  entity = truck_waybill,
  entity_mod = truck_waybill,
  create_req = dtos::CreateTruckWaybillRequest,
  update_req = dtos::UpdateTruckWaybillRequest,
  response = dtos::TruckWaybillResponse,
  apply_update = apply_truck_waybill_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
