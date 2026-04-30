use std::{
  sync::{atomic::Ordering, Arc},
  time::Duration,
};

use axum::extract::{Extension, State};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiError, ApiResponse, ApiResult, ApiState},
  dtos::{NodeStatusResponse, OperationMessageResponse, UpdateCentralApiUrlRequest},
  endpoints::paths,
  enums::RoleType,
  services::system::{
    database_instance::load_active_database_instance,
    local::load_local_bootstrap,
    node_bases::load_node_base_ids,
    SystemService,
  },
  utils::{jwt::Claims, lifecycle::request_restart},
};

#[utoipa::path(
  post,
  tag = "System - Lifecycle",
  operation_id = "node_restart",
  summary = "Trigger API restart",
  description = "Triggers a controlled API restart signal. This endpoint is restricted to admin role.",
  path = paths::node::RESTART,
  responses(
    (status = 200, description = "Restart initiated", body = ApiResponse<OperationMessageResponse>),
    (status = 403, description = "Forbidden envelope when caller role is not admin."),
    (status = 409, description = "Conflict envelope when a restart is already in progress.")
  )
)]
#[axum::debug_handler]
async fn restart_api(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
) -> ApiResult<OperationMessageResponse> {
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

  Ok(ApiResponse::success(OperationMessageResponse {
    message: "API restart initiated".to_string(),
  }))
}

async fn build_node_status_response(state: &ApiState) -> ApiResult<NodeStatusResponse> {
  let node_name = load_active_database_instance(state.db.as_ref(), state.cfg.node.db_id)
    .await
    .map(|row| row.common_name)
    .unwrap_or_default();

  let central_api_url = load_local_bootstrap(state.db.as_ref())
    .await
    .ok()
    .and_then(|row| row.central_api_url)
    .or_else(|| state.cfg.node.central_api_url.clone());
  let assigned_base_ids = load_node_base_ids(state.db.as_ref(), state.cfg.node.db_id).await?;

  let worker = state.worker_status.read().await;
  Ok(ApiResponse::success(NodeStatusResponse {
    is_initialized: state.is_initialized.load(Ordering::Relaxed),
    node_type: state.cfg.node.node_type.clone(),
    node_name,
    worker_state: format!("{:?}", worker.state),
    last_sync_at: worker.last_sync_at.map(|t| t.to_rfc3339()),
    central_api_url,
    assigned_base_ids,
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
  build_node_status_response(&state).await
}

#[utoipa::path(
  patch,
  tag = "System - Lifecycle",
  operation_id = "node_update_central_api_url",
  summary = "Change central API URL",
  description = "Updates the peripheral's central API URL after a reachability probe. Admin only. \
                The worker picks up the new URL on its next tick without needing a restart.",
  path = paths::node::CENTRAL_API_URL,
  request_body = UpdateCentralApiUrlRequest,
  responses(
    (status = 200, description = "Updated central API URL", body = ApiResponse<NodeStatusResponse>),
    (status = 400, description = "Invalid URL or probe failed"),
    (status = 403, description = "Only admin can change the central URL")
  )
)]
#[axum::debug_handler]
async fn update_central_api_url(
  State(state): State<Arc<ApiState>>,
  Extension(claims): Extension<Claims>,
  axum::Json(req): axum::Json<UpdateCentralApiUrlRequest>,
) -> ApiResult<NodeStatusResponse> {
  let role: RoleType = claims
    .role
    .parse()
    .map_err(|_| ApiError::Forbidden("Invalid role in token".to_string()))?;
  if role != RoleType::Admin {
    return Err(ApiError::Forbidden(
      "Only admin can change the central API URL".to_string(),
    ));
  }

  let url = req.url.trim();

  let parsed =
    reqwest::Url::parse(url).map_err(|err| ApiError::BadRequest(format!("Invalid URL: {err}")))?;
  match parsed.scheme() {
    "http" | "https" => {}
    other => {
      return Err(ApiError::BadRequest(format!(
        "URL must use http or https scheme (got {other})"
      )));
    }
  }

  let probe_url = format!("{}/health", url.trim_end_matches('/'));
  let client = reqwest::Client::builder()
    .timeout(Duration::from_secs(3))
    .build()
    .map_err(|err| ApiError::Internal(anyhow::anyhow!("Failed to build probe client: {err}")))?;

  let response = client
    .get(&probe_url)
    .send()
    .await
    .map_err(|err| ApiError::BadRequest(format!("Central API at {url} is unreachable: {err}")))?;
  if !response.status().is_success() {
    return Err(ApiError::BadRequest(format!(
      "Central API at {url} responded with status {}",
      response.status()
    )));
  }

  let service = SystemService::new(state.db.clone(), state.cfg.clone());
  service.update_central_api_url(url).await?;

  build_node_status_response(&state).await
}

pub fn lifecycle_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(restart_api))
    .routes(routes!(node_status))
    .routes(routes!(update_central_api_url))
    .with_state(state)
}
