use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{rail_wagon_manifest, rail_waybill},
  services::{common::set_if_some, DocumentService},
};

pub(super) async fn ensure_parent_manifest_active(
  conn: &impl ConnectionTrait,
  manifest_id: uuid::Uuid,
) -> Result<(), ApiError> {
  let doc = rail_wagon_manifest::Entity::load()
    .filter_by_id(manifest_id)
    .filter(rail_wagon_manifest::Column::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| {
      ApiError::NotFound(format!("Rail wagon manifest '{}' not found", manifest_id))
    })?;

  match doc.deleted_at {
    Some(_) => Err(ApiError::BadRequest(format!(
      "Cannot modify measurement of deleted wagon manifest '{}'",
      manifest_id
    ))),
    None => Ok(()),
  }
}

fn apply_rail_manifest_update(
  model: &mut rail_wagon_manifest::ActiveModel,
  req: &dtos::UpdateRailWagonManifestRequest,
) {
  set_if_some(&mut model.wagon_number, req.wagon_number.clone());
  set_if_some(&mut model.product_id, req.product_id);
  set_if_some(&mut model.declared_volume, req.declared_volume);
  set_if_some(&mut model.declared_density, req.declared_density);
  set_if_some(&mut model.declared_mass, req.declared_mass);
}

async fn ensure_parent_waybill_active(
  conn: &impl ConnectionTrait,
  bill_id: Uuid,
) -> Result<(), ApiError> {
  let doc = rail_waybill::Entity::load()
    .filter_by_id(bill_id)
    .filter(rail_waybill::Column::DeletedAt.is_null())
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("Rail waybill '{}' not found", bill_id)))?;

  match doc.deleted_at {
    Some(_) => Err(ApiError::BadRequest(format!(
      "Cannot modify manifest of deleted waybill '{}'",
      bill_id
    ))),
    None => Ok(()),
  }
}

async fn ensure_rail_manifest_update_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &rail_wagon_manifest::Model,
  _req: &dtos::UpdateRailWagonManifestRequest,
) -> Result<(), ApiError> {
  ensure_parent_waybill_active(conn, existing.rail_waybill_id).await
}

async fn ensure_rail_manifest_soft_delete_allowed(
  _svc: &DocumentService,
  conn: &impl ConnectionTrait,
  existing: &rail_wagon_manifest::Model,
  _undo: bool,
) -> Result<(), ApiError> {
  ensure_parent_waybill_active(conn, existing.rail_waybill_id).await
}

#[voletu_core_macros::entity_service(
  entity_name = "Rail wagon manifest",
  entity = rail_manifest,
  entity_mod = rail_wagon_manifest,
  create_req = dtos::CreateRailWagonManifestRequest,
  update_req = dtos::UpdateRailWagonManifestRequest,
  response = dtos::RailWagonManifestResponse,
  apply_update = apply_rail_manifest_update,
  before_update = ensure_rail_manifest_update_allowed,
  before_soft_delete = ensure_rail_manifest_soft_delete_allowed,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl DocumentService {}
