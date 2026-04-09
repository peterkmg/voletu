use std::sync::Arc;

use axum::extract::State;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{response::ApiResponse, result::ApiResult, state::ApiState},
  dtos::SeedResult,
};

#[utoipa::path(
  post,
  tag = "Dev",
  operation_id = "dev_seed",
  summary = "Seed database with fake data",
  description = "Populates the database with realistic fake data for development. Additive — can be called multiple times. Only available in debug builds.",
  path = "/dev/seed",
  responses((status = 200, body = ApiResponse<SeedResult>))
)]
#[axum::debug_handler]
async fn dev_seed(State(state): State<Arc<ApiState>>) -> ApiResult<SeedResult> {
  Ok(ApiResponse::success(state.svc.system.dev_seed().await?))
}

pub fn dev_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(dev_seed))
    .with_state(state)
}
