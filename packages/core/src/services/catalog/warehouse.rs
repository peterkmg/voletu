use crate::{
  dtos,
  entities::{base, warehouse},
  services::{
    common::{set_if_some, set_if_some_mapped},
    CatalogService,
  },
};

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
  crate::services::common::validate_fk_exists::<base::Entity>(
    conn,
    req.base_id,
    base::Column::Id,
    base::Column::DeletedAt,
    "baseId",
  )
  .await?;
  Ok(())
}

async fn before_warehouse_update(
  _svc: &CatalogService,
  conn: &impl sea_orm::ConnectionTrait,
  _existing: &warehouse::Model,
  req: &dtos::UpdateWarehouseRequest,
) -> Result<(), crate::api::ApiError> {
  if let Some(base_id) = req.base_id {
    crate::services::common::validate_fk_exists::<base::Entity>(
      conn,
      base_id,
      base::Column::Id,
      base::Column::DeletedAt,
      "baseId",
    )
    .await?;
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
