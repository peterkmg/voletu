use sea_orm::{ColumnTrait, ConnectionTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  dtos,
  entities::{base, warehouse},
  services::{
    common::{set_if_some, set_if_some_mapped},
    CatalogService,
  },
};

async fn ensure_active_base(
  conn: &impl ConnectionTrait,
  base_id: Uuid,
  field_name: &str,
) -> Result<(), crate::api::ApiError> {
  let exists = base::Entity::load()
    .filter_by_id(base_id)
    .filter(base::Column::DeletedAt.is_null())
    .one(conn)
    .await?;

  if exists.is_none() {
    return Err(crate::api::ApiError::BadRequest(format!(
      "{field_name} '{base_id}' does not reference a valid record"
    )));
  }

  Ok(())
}

fn apply_warehouse_update(model: &mut warehouse::ActiveModel, req: &dtos::UpdateWarehouseRequest) {
  set_if_some(&mut model.base_id, req.base_id);
  set_if_some(&mut model.common_name, req.common_name.clone());
  set_if_some_mapped(&mut model.long_name, req.long_name.clone(), Some);
}

async fn before_warehouse_create(
  _svc: &CatalogService,
  conn: &impl sea_orm::ConnectionTrait,
  req: &dtos::CreateWarehouseRequest,
) -> Result<(), crate::api::ApiError> {
  ensure_active_base(conn, req.base_id, "baseId").await?;
  Ok(())
}

async fn before_warehouse_update(
  _svc: &CatalogService,
  conn: &impl sea_orm::ConnectionTrait,
  _existing: &warehouse::Model,
  req: &dtos::UpdateWarehouseRequest,
) -> Result<(), crate::api::ApiError> {
  if let Some(base_id) = req.base_id {
    ensure_active_base(conn, base_id, "baseId").await?;
  }
  Ok(())
}

#[voletu_core_macros::entity_service(
  entity_name = "Warehouse",
  entity = warehouse,
  entity_mod = warehouse,
  create_req = dtos::CreateWarehouseRequest,
  update_req = dtos::UpdateWarehouseRequest,
  response = dtos::WarehouseResponse,
  apply_update = apply_warehouse_update,
  before_create = before_warehouse_create,
  before_update = before_warehouse_update,
  ops(create, list, get, update, soft_delete, hard_delete),
)]
impl CatalogService {}
