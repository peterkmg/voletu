use std::sync::Arc;

use axum::{extract::State, Json};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{
    ChangePasswordRequest,
    CompleteInitializationRequest,
    LoginRequest,
    LoginResponse,
    RefreshTokenRequest,
  },
  endpoints::paths,
  utils::lifecycle::request_restart,
};

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
struct ChangePasswordResponse {
  message: String,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
struct CompleteInitializationResponse {
  message: String,
}

#[utoipa::path(
  post,
  tag = "System - Auth",
  operation_id = "auth_login",
  summary = "Login",
  description = "Authenticates a user and returns access and refresh tokens.",
  path = paths::auth::LOGIN,
  request_body = LoginRequest,
  responses(
    (status = 200, description = "Successful login", body = ApiResponse<LoginResponse>),
    (status = 401, description = "Unauthorized envelope. Example: {\"success\":false,\"error\":{\"code\":\"UNAUTHORIZED\",\"message\":\"Unauthorized: invalid credentials\"}}")
  )
)]
#[axum::debug_handler]
async fn login(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<LoginRequest>>,
) -> ApiResult<LoginResponse> {
  let res = state.svc.system.authenticate(&req).await?;
  Ok(ApiResponse::success(res))
}

#[utoipa::path(
  post,
  tag = "System - Auth",
  operation_id = "auth_refresh",
  summary = "Refresh access token",
  description = "Issues a new access token from a valid refresh token.",
  path = paths::auth::REFRESH,
  request_body = RefreshTokenRequest,
  responses(
    (status = 200, description = "Token refreshed", body = ApiResponse<LoginResponse>),
    (status = 401, description = "Unauthorized envelope when refresh token is invalid or expired.")
  )
)]
#[axum::debug_handler]
async fn refresh(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<RefreshTokenRequest>>,
) -> ApiResult<LoginResponse> {
  let res = state
    .svc
    .system
    .refresh_access_token(&req.refresh_token)
    .await?;
  Ok(ApiResponse::success(res))
}

#[utoipa::path(
  post,
  tag = "System - Auth",
  operation_id = "auth_change_password",
  summary = "Change password",
  description = "Changes the current authenticated user's password.",
  path = paths::auth::CHANGE_PASSWORD,
  request_body = ChangePasswordRequest,
  responses(
    (status = 200, description = "Password changed", body = ApiResponse<ChangePasswordResponse>),
    (status = 401, description = "Unauthorized envelope when credentials or token are invalid.")
  )
)]
#[axum::debug_handler]
async fn change_password(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<ChangePasswordRequest>>,
) -> ApiResult<ChangePasswordResponse> {
  state.svc.system.change_password(&req).await?;
  Ok(ApiResponse::success(ChangePasswordResponse {
    message: "Password changed".to_string(),
  }))
}

#[utoipa::path(
  post,
  tag = "System - Auth",
  operation_id = "node_complete_initialization",
  summary = "Complete node initialization",
  description = "Completes first-time node initialization and triggers a controlled API restart to apply the new system state.",
  path = paths::node::INITIALIZE,
  request_body = CompleteInitializationRequest,
  responses(
    (status = 200, description = "Initialization completed", body = ApiResponse<CompleteInitializationResponse>),
    (status = 400, description = "Validation error envelope for malformed payload."),
    (status = 401, description = "Unauthorized envelope for invalid credentials."),
    (status = 409, description = "Conflict envelope when initialization is already completed or current state disallows action.")
  )
)]
#[axum::debug_handler]
async fn complete_initialization(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CompleteInitializationRequest>>,
) -> ApiResult<CompleteInitializationResponse> {
  state.svc.system.complete_initialization(&req).await?;
  state
    .is_initialized
    .store(true, std::sync::atomic::Ordering::Relaxed);
  request_restart(&state.restart_tx)?;
  Ok(ApiResponse::success(CompleteInitializationResponse {
    message: "Initialization completed".to_string(),
  }))
}

pub fn auth_public_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(login))
    .routes(routes!(refresh))
    .with_state(state)
}

pub fn auth_protected_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(change_password))
    .routes(routes!(complete_initialization))
    .with_state(state)
}
