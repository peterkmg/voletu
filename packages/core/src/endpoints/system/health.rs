use axum::Json;
use serde::Serialize;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{api::ApiResponse, endpoints::paths};

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct HealthData {
  pub status: String,
}

#[utoipa::path(
  get,
  tag = "System - Health",
  operation_id = "system_health",
  summary = "Health check",
  description = "Returns basic API liveness status used by orchestration and readiness probes.",
  path = paths::health::ROOT,
  responses((status = 200, description = "Health check", body = ApiResponse<HealthData>))
)]
#[axum::debug_handler]
async fn health() -> Json<ApiResponse<HealthData>> {
  Json(ApiResponse::success(HealthData {
    status: "ok".to_string(),
  }))
}

pub fn health_routes() -> OpenApiRouter {
  OpenApiRouter::new().routes(routes!(health))
}
