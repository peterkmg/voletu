use std::sync::Arc;

use axum::{
  extract::{Path, State},
  Json,
};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{CreateUserRequest, UserResponse},
  endpoints::paths,
};

#[utoipa::path(
  get,
  tag = "System - User",
  operation_id = "system_user_list",
  summary = "List users",
  description = "Returns system users with their role bindings for administration workflows.",
  path = paths::users::ROOT,
  responses(
    (status = 200, description = "List of all users", body = ApiResponse<Vec<UserResponse>>),
  )
)]
#[axum::debug_handler]
async fn user_list(State(state): State<Arc<ApiState>>) -> ApiResult<Vec<UserResponse>> {
  tracing::debug!("GET /users");
  Ok(ApiResponse::success(state.svc.system.user_list().await?))
}

#[utoipa::path(
  post,
  tag = "System - User",
  operation_id = "system_user_create",
  summary = "Create user",
  description = "Creates a new application user assigned to a system role.",
  path = paths::users::ROOT,
  request_body = CreateUserRequest,
  responses(
    (status = 200, description = "User created", body = ApiResponse<UserResponse>),
    (status = 400, description = "Validation error envelope for malformed request fields."),
    (status = 404, description = "Not found envelope when referenced role does not exist."),
    (status = 409, description = "Conflict envelope when username is already taken."),
  )
)]
#[axum::debug_handler]
async fn user_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateUserRequest>>,
) -> ApiResult<UserResponse> {
  tracing::debug!(username = %req.username, "POST /users");
  let user = state.svc.system.user_create(&req).await?;
  Ok(ApiResponse::success(user))
}

#[utoipa::path(
  delete,
  tag = "System - User",
  operation_id = "system_user_delete",
  summary = "Delete user",
  description = "Soft deletes a user account by UUID.",
  path = paths::users::BY_ID,
  params(
    ("id" = Uuid, Path, description = "UUID of the user to delete"),
  ),
  responses(
    (status = 200, description = "User deleted"),
    (status = 404, description = "User not found"),
  )
)]
#[axum::debug_handler]
async fn user_delete(State(state): State<Arc<ApiState>>, Path(id): Path<Uuid>) -> ApiResult<()> {
  tracing::debug!(id = %id, "DELETE /users/:id");
  state.svc.system.user_soft_delete(id).await?;
  Ok(ApiResponse::success(()))
}

pub fn user_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(user_list, user_create))
    .routes(routes!(user_delete))
    .with_state(state)
}
