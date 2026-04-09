use std::sync::{atomic::Ordering, Arc};

use axum::{extract::State, Json};
use serde::Serialize;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiState},
  endpoints::paths,
  services::system::database_instance::load_active_database_instance,
};

#[derive(Debug, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthData {
  pub status: String,
  pub is_initialized: bool,
  pub node_type: String,
  pub node_name: String,
}

#[utoipa::path(
  get,
  tag = "System - Health",
  operation_id = "system_health",
  summary = "Health check",
  description = "Returns API liveness status and node identity information.",
  path = paths::health::ROOT,
  responses((status = 200, description = "Health check", body = ApiResponse<HealthData>))
)]
#[axum::debug_handler]
async fn health(State(state): State<Arc<ApiState>>) -> Json<ApiResponse<HealthData>> {
  let node_name = load_active_database_instance(state.db.as_ref(), state.cfg.node.db_id)
    .await
    .map(|row| row.common_name)
    .unwrap_or_default();

  Json(ApiResponse::success(HealthData {
    status: "ok".to_string(),
    is_initialized: state.is_initialized.load(Ordering::Relaxed),
    node_type: state.cfg.node.node_type.clone(),
    node_name,
  }))
}

pub fn health_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(health))
    .with_state(state)
}
