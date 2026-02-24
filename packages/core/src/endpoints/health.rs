use axum::Json;
use serde::Serialize;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::api::ApiResponse;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct HealthData {
  pub status: String,
}

#[utoipa::path(
  get,
  path = "/health",
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
