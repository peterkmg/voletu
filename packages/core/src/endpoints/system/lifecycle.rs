use std::str::FromStr;

use axum::extract::{Extension, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiError, ApiResponse, ApiResult, ApiState},
  endpoints::paths,
  enums::RoleType,
  utils::{jwt::Claims, lifecycle::request_restart},
};

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
struct RestartResponse {
  message: String,
}

#[utoipa::path(
  post,
  tag = "System - Lifecycle",
  operation_id = "node_restart",
  summary = "Trigger API restart",
  description = "Triggers a controlled API restart signal. This endpoint is restricted to admin role.",
  path = paths::node::RESTART,
  responses(
    (status = 200, description = "Restart initiated", body = ApiResponse<RestartResponse>),
    (status = 403, description = "Forbidden envelope when caller role is not admin."),
    (status = 409, description = "Conflict envelope when a restart is already in progress.")
  )
)]
#[axum::debug_handler]
async fn restart_api(
  State(state): State<std::sync::Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
) -> ApiResult<RestartResponse> {
  let role = RoleType::from_str(&claims.role)
    .map_err(|_| ApiError::Forbidden("Invalid role in token".to_string()))?;

  if role != RoleType::Admin {
    return Err(ApiError::Forbidden(
      "Only admin can trigger API restart".to_string(),
    ));
  }

  request_restart(&state.restart_tx)?;

  Ok(ApiResponse::success(RestartResponse {
    message: "API restart initiated".to_string(),
  }))
}

pub fn lifecycle_routes(state: std::sync::Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(restart_api))
    .with_state(state)
}
