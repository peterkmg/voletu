use std::sync::Arc;

use axum::{extract::State, Json};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult},
  app::AppState,
  dtos::auth::{LoginRequest, LoginResponse},
};

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
  State(state): State<Arc<AppState>>,
  Valid(Json(req)): Valid<Json<LoginRequest>>,
) -> ApiResult<LoginResponse> {
  let res = state.auth_service.authenticate(&req).await?;
  Ok(ApiResponse::success(res))
}

pub fn auth_routes(state: Arc<AppState>) -> OpenApiRouter {
  let public = OpenApiRouter::new().routes(routes!(login));
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
