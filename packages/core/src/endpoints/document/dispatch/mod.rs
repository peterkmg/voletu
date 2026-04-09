use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  Extension,
  Json,
};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{
    CreateDispatchCompositeRequest,
    CreateDispatchMeasurementRequest,
    CreateDispatchRequest,
    DispatchCompositeResponse,
    DispatchMeasurementResponse,
    DispatchResponse,
    EmbedParams,
  },
  endpoints::paths,
  enums,
  services::common::{ensure_senior_supervisor_or_higher, ensure_supervisor_or_higher},
  utils::jwt::Claims,
};

mod composite;
mod document;
mod measurement;

pub fn dispatch_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(document::document_routes(state.clone()))
    .merge(composite::composite_routes(state.clone()))
    // Standalone item CRUD disabled — items managed through composite endpoints.
    .merge(measurement::measurement_routes(state))
}
