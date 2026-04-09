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
    CreateRailWagonManifestRequest,
    CreateRailWagonMeasurementRequest,
    CreateRailWagonWeightRequest,
    CreateRailWaybillRequest,
    RailWagonManifestResponse,
    RailWagonMeasurementResponse,
    RailWagonWeightResponse,
    RailWaybillCompositeRequest,
    RailWaybillCompositeResponse,
    RailWaybillResponse,
    UpdateRailWagonManifestRequest,
    UpdateRailWagonMeasurementRequest,
    UpdateRailWagonWeightRequest,
    UpdateRailWaybillRequest,
  },
  endpoints::{
    paths,
    query::{EmbedParams, PaginationParams},
  },
};

mod composite;
mod manifest;
mod measurement;
mod waybill;
mod weight;

pub fn rail_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(waybill::waybill_routes(state.clone()))
    .merge(manifest::manifest_routes(state.clone()))
    .merge(measurement::measurement_routes(state.clone()))
    .merge(weight::weight_routes(state.clone()))
    .merge(composite::composite_routes(state))
}
