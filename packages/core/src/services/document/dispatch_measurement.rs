use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{dispatch_document, dispatch_storage_measurement},
  services::{
    common::{ensure_doc_mod_allowed, set_if_some, set_if_some_mapped},
    document::DocumentService,
  },
};

async fn get_by_id(
  conn: &impl ConnectionTrait,
  doc_id: Uuid,
) -> Result<dispatch_document::ModelEx, ApiError> {
  dispatch_document::Entity::load()
    .filter_by_id(doc_id)
    .filter(dispatch_document::Column::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Dispatch document '{}' not found", doc_id)))
}

pub(super) async fn ensure_dispatch_storage_measurement_create_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  req: &dtos::CreateDispatchMeasurementRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(conn, req.dispatch_doc_id).await?;
  ensure_doc_mod_allowed(doc.status)
}

pub(super) fn apply_dispatch_storage_measurement_update(
  model: &mut dispatch_storage_measurement::ActiveModel,
  req: &dtos::UpdateDispatchMeasurementRequest,
) {
  set_if_some(&mut model.storage_id, req.storage_id);
  set_if_some_mapped(&mut model.before_height, req.before_height, Some);
  set_if_some_mapped(&mut model.before_volume, req.before_volume, Some);
  set_if_some_mapped(&mut model.before_density, req.before_density, Some);
  set_if_some(&mut model.before_mass, req.before_mass);
  set_if_some_mapped(&mut model.after_height, req.after_height, Some);
  set_if_some_mapped(&mut model.after_volume, req.after_volume, Some);
  set_if_some_mapped(&mut model.after_density, req.after_density, Some);
  set_if_some(&mut model.after_mass, req.after_mass);
}

pub(super) async fn ensure_dispatch_storage_measurement_update_allowed(
  _svc: &DocumentService,
  txn: &impl sea_orm::ConnectionTrait,
  existing: &dispatch_storage_measurement::Model,
  _req: &dtos::UpdateDispatchMeasurementRequest,
) -> Result<(), ApiError> {
  let doc = get_by_id(txn, existing.dispatch_doc_id).await?;
  ensure_doc_mod_allowed(doc.status)
}

pub(super) async fn ensure_dispatch_storage_measurement_soft_delete_allowed(
  _svc: &DocumentService,
  txn: &impl sea_orm::ConnectionTrait,
  existing: &dispatch_storage_measurement::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  let doc = get_by_id(txn, existing.dispatch_doc_id).await?;
  ensure_doc_mod_allowed(doc.status)
}

#[voletu_core_macros::entity_service(
  entity_name = "Dispatch measurement",
	entity = dispatch_storage_measurement,
	entity_mod = dispatch_storage_measurement,
  create_req = dtos::CreateDispatchMeasurementRequest,
	update_req = dtos::UpdateDispatchMeasurementRequest,
	response = dtos::DispatchMeasurementResponse,
  before_create = ensure_dispatch_storage_measurement_create_allowed,
	apply_update = apply_dispatch_storage_measurement_update,
	before_update = ensure_dispatch_storage_measurement_update_allowed,
	before_soft_delete = ensure_dispatch_storage_measurement_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
