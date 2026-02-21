use std::sync::Arc;

use axum::{
  extract::{Path, State},
  Json,
};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult},
  app::AppState,
  dtos::user::{CreateUserRequest, UserResponse},
};

#[utoipa::path(
  get,
  path = "/users",
  responses(
    (status = 200, description = "List of all users", body = ApiResponse<Vec<UserResponse>>),
  )
)]
#[axum::debug_handler]
async fn list_users(State(state): State<Arc<AppState>>) -> ApiResult<Vec<UserResponse>> {
  tracing::debug!("GET /users");
  let users = state.user_service.list().await?;
  Ok(ApiResponse::success(users))
}

#[utoipa::path(
  post,
  path = "/users",
  request_body = CreateUserRequest,
  responses(
    (status = 200, description = "User created", body = ApiResponse<UserResponse>),
    (status = 400, description = "Validation error"),
    (status = 404, description = "Role not found"),
    (status = 409, description = "Username already taken"),
  )
)]
#[axum::debug_handler]
async fn create_user(
  State(state): State<Arc<AppState>>,
  Valid(Json(req)): Valid<Json<CreateUserRequest>>,
) -> ApiResult<UserResponse> {
  tracing::debug!(username = %req.username, "POST /users");
  let user = state.user_service.create(&req).await?;
  Ok(ApiResponse::success(user))
}

#[utoipa::path(
  delete,
  path = "/users/{id}",
  params(
    ("id" = Uuid, Path, description = "UUID of the user to delete"),
  ),
  responses(
    (status = 200, description = "User deleted"),
    (status = 404, description = "User not found"),
  )
)]
#[axum::debug_handler]
async fn delete_user(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> ApiResult<()> {
  tracing::debug!(id = %id, "DELETE /users/:id");
  state.user_service.delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub fn user_routes(state: Arc<AppState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(list_users, create_user))
    .routes(routes!(delete_user))
    .with_state(state)
}
