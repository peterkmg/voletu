use std::sync::Arc;

use axum::extract::{Path, State};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde::Deserialize;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiError, ApiResponse, ApiResult, ApiState},
  endpoints::paths,
  entities::node_base_assignment,
  services::system::node_bases::{load_node_base_assignment, load_node_base_assignments},
};

#[derive(Debug, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
struct AddBaseAssignmentRequest {
  base_id: Uuid,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
struct BaseAssignmentResponse {
  id: Uuid,
  node_id: Uuid,
  base_id: Uuid,
}

impl From<node_base_assignment::Model> for BaseAssignmentResponse {
  fn from(m: node_base_assignment::Model) -> Self {
    Self {
      id: m.id,
      node_id: m.node_id,
      base_id: m.base_id,
    }
  }
}

impl From<&node_base_assignment::ModelEx> for BaseAssignmentResponse {
  fn from(m: &node_base_assignment::ModelEx) -> Self {
    Self {
      id: m.id,
      node_id: m.node_id,
      base_id: m.base_id,
    }
  }
}

#[utoipa::path(
  get,
  tag = "Node",
  operation_id = "node_list_base_assignments",
  summary = "List base assignments",
  description = "Returns the bases this node is configured to handle.",
  path = paths::node::BASES,
  responses((status = 200, body = ApiResponse<Vec<BaseAssignmentResponse>>))
)]
#[axum::debug_handler]
async fn list_base_assignments(
  State(state): State<Arc<ApiState>>,
) -> ApiResult<Vec<BaseAssignmentResponse>> {
  let local_node_id = state.cfg.node.db_id;
  let rows = load_node_base_assignments(state.db.as_ref(), local_node_id).await?;
  Ok(ApiResponse::success(
    rows.iter().map(BaseAssignmentResponse::from).collect(),
  ))
}

#[utoipa::path(
  post,
  tag = "Node",
  operation_id = "node_add_base_assignment",
  summary = "Add base assignment",
  description = "Assigns a base to this node for sync routing.",
  path = paths::node::BASES,
  request_body = AddBaseAssignmentRequest,
  responses(
    (status = 200, body = ApiResponse<BaseAssignmentResponse>),
    (status = 409, description = "Base already assigned")
  )
)]
#[axum::debug_handler]
async fn add_base_assignment(
  State(state): State<Arc<ApiState>>,
  axum::Json(req): axum::Json<AddBaseAssignmentRequest>,
) -> ApiResult<BaseAssignmentResponse> {
  let local_node_id = state.cfg.node.db_id;

  let existing = load_node_base_assignment(state.db.as_ref(), local_node_id, req.base_id).await?;

  if existing.is_some() {
    return Err(ApiError::Conflict(
      "Base already assigned to this node".into(),
    ));
  }

  let row = node_base_assignment::ActiveModel {
    node_id: Set(local_node_id),
    base_id: Set(req.base_id),
    ..Default::default()
  }
  .insert(state.db.as_ref())
  .await?;

  Ok(ApiResponse::success(BaseAssignmentResponse::from(row)))
}

#[utoipa::path(
  delete,
  tag = "Node",
  operation_id = "node_remove_base_assignment",
  summary = "Remove base assignment",
  description = "Removes a base assignment from this node.",
  path = paths::node::BASES_BY_ID,
  params(("baseId" = Uuid, Path, description = "Base UUID to remove")),
  responses(
    (status = 200, body = ApiResponse<String>),
    (status = 404, description = "Base assignment not found")
  )
)]
#[axum::debug_handler]
async fn remove_base_assignment(
  State(state): State<Arc<ApiState>>,
  Path(base_id): Path<Uuid>,
) -> ApiResult<String> {
  let local_node_id = state.cfg.node.db_id;

  let row = load_node_base_assignment(state.db.as_ref(), local_node_id, base_id)
    .await?
    .ok_or_else(|| ApiError::NotFound("Base assignment not found".into()))?;

  node_base_assignment::Entity::delete_by_id(row.id)
    .exec(state.db.as_ref())
    .await?;

  Ok(ApiResponse::success("removed".to_string()))
}

pub fn node_base_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(list_base_assignments, add_base_assignment))
    .routes(routes!(remove_base_assignment))
    .with_state(state)
}

// Workaround: utoipa routes macro needs to be split when paths differ.
