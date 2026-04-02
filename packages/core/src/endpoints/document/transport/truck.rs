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
    CreateTruckWaybillItemRequest,
    CreateTruckWaybillRequest,
    CreateTruckWeightDocRequest,
    TruckWaybillCompositeRequest,
    TruckWaybillCompositeResponse,
    TruckWaybillItemResponse,
    TruckWaybillResponse,
    TruckWeightDocResponse,
    UpdateTruckWaybillItemRequest,
    UpdateTruckWaybillRequest,
    UpdateTruckWeightDocRequest,
  },
  endpoints::{paths, query::{EmbedParams, PaginationParams}},
};

mod composite;
mod item;
mod waybill;
mod weight_doc;

pub fn truck_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(waybill::waybill_routes(state.clone()))
    .merge(item::item_routes(state.clone()))
    .merge(weight_doc::weight_doc_routes(state.clone()))
    .merge(composite::composite_routes(state))
}
