use std::sync::{atomic::Ordering, Arc};

use axum::extract::{Extension, State};
use sea_orm::EntityTrait;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiError, ApiResponse, ApiResult, ApiState},
  endpoints::paths,
  entities::database_instance,
  enums::RoleType,
  utils::{jwt::Claims, lifecycle::request_restart},
};

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
struct RestartResponse {
  message: String,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
struct NodeStatusResponse {
  is_initialized: bool,
  node_type: String,
  node_name: String,
  worker_state: String,
  last_sync_at: Option<String>,
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
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
) -> ApiResult<RestartResponse> {
  let role: RoleType = claims
    .role
    .parse()
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

#[utoipa::path(
  get,
  tag = "System - Lifecycle",
  operation_id = "node_status",
  summary = "Get node status",
  description = "Returns current node identity, initialization state, and sync worker status.",
  path = paths::node::STATUS,
  responses(
    (status = 200, description = "Node status", body = ApiResponse<NodeStatusResponse>),
    (status = 401, description = "Unauthorized envelope.")
  )
)]
#[axum::debug_handler]
async fn node_status(State(state): State<Arc<ApiState>>) -> ApiResult<NodeStatusResponse> {
  let node_name = database_instance::Entity::find_by_id(state.cfg.node.db_id)
    .one(state.db.as_ref())
    .await
    .map_err(ApiError::Database)?
    .map(|row| row.common_name)
    .unwrap_or_default();

  let worker = state.worker_status.read().await;
  Ok(ApiResponse::success(NodeStatusResponse {
    is_initialized: state.is_initialized.load(Ordering::Relaxed),
    node_type: state.cfg.node.node_type.clone(),
    node_name,
    worker_state: format!("{:?}", worker.state),
    last_sync_at: worker.last_sync_at.map(|t| t.to_rfc3339()),
  }))
}

pub fn lifecycle_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(restart_api))
    .routes(routes!(node_status))
    .with_state(state)
}
