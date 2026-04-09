use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  Json,
};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{
    BaseResponse,
    CreateBaseRequest,
    CreatePortRequest,
    CreateStorageRequest,
    CreateWarehouseRequest,
    EmbedParams,
    PaginationParams,
    PortResponse,
    StorageResponse,
    UpdateBaseRequest,
    UpdatePortRequest,
    UpdateStorageRequest,
    UpdateWarehouseRequest,
    WarehouseResponse,
  },
  endpoints::paths,
};

mod base;
mod port;
mod storage;
mod warehouse;

pub fn topology_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(base::base_routes(state.clone()))
    .merge(warehouse::warehouse_routes(state.clone()))
    .merge(storage::storage_routes(state.clone()))
    .merge(port::port_routes(state))
}
