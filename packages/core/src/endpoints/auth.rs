use std::sync::Arc;

use axum::{extract::State, Json};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::auth::{ChangePasswordRequest, LoginRequest, LoginResponse},
};

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
struct ChangePasswordResponse {
  message: String,
}

#[utoipa::path(
  post,
  path = "/login",
  request_body = LoginRequest,
  responses(
    (status = 200, description = "Successful login", body = ApiResponse<LoginResponse>),
    (status = 401, description = "Unauthorized")
  )
)]
#[axum::debug_handler]
async fn login(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<LoginRequest>>,
) -> ApiResult<LoginResponse> {
  let res = state.auth_service.authenticate(&req).await?;
  Ok(ApiResponse::success(res))
}

#[utoipa::path(
  post,
  path = "/change-password",
  request_body = ChangePasswordRequest,
  responses(
    (status = 200, description = "Password changed", body = ApiResponse<ChangePasswordResponse>),
    (status = 401, description = "Unauthorized")
  )
)]
#[axum::debug_handler]
async fn change_password(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<ChangePasswordRequest>>,
) -> ApiResult<ChangePasswordResponse> {
  state.auth_service.change_password(&req).await?;
  Ok(ApiResponse::success(ChangePasswordResponse {
    message: "Password changed".to_string(),
  }))
}

pub fn auth_routes(state: Arc<ApiState>) -> OpenApiRouter {
  let public = OpenApiRouter::new().routes(routes!(login, change_password));
  //     .routes(routes!(register))

  // let protected = OpenApiRouter::new()
  //     .routes(routes!(logout))
  //     .routes(routes!(refresh))
  //     .routes(routes!(me))
  //     .routes(routes!(update_profile))
  //     .routes(routes!(change_password))
  //     .routes(routes!(delete_account))
  // .layer(from_fn_with_state(state.clone(), auth_middleware));

  OpenApiRouter::new()
    .nest(
      "/auth",
      OpenApiRouter::new().merge(public), // .merge(protected))
    )
    .with_state(state)
}
