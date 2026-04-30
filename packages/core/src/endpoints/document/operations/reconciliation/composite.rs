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
  dtos::{
    CreateInventoryReconciliationCompositeRequest,
    InventoryReconciliationCompositeResponse,
    UpdateInventoryReconciliationCompositeRequest,
  },
  endpoints::paths,
};

#[utoipa::path(
  post,
  tag = "Document - Operations",
  operation_id = "inventory_reconciliation_composite_create",
  summary = "Create reconciliation composite",
  description = "Creates a reconciliation document together with its adjustments in one transactional flow.",
  path = paths::operations::RECONCILIATIONS_COMPOSITE_SAVE,
  request_body = CreateInventoryReconciliationCompositeRequest,
  responses(
    (status = 200, body = ApiResponse<InventoryReconciliationCompositeResponse>),
    (status = 400)
  )
)]
#[axum::debug_handler]
pub(super) async fn inventory_reconciliation_composite_create(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<CreateInventoryReconciliationCompositeRequest>>,
) -> ApiResult<InventoryReconciliationCompositeResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .inventory_reconciliation_composite_create(&req)
      .await?,
  ))
}

#[utoipa::path(
  put,
  tag = "Document - Operations",
  operation_id = "inventory_reconciliation_composite_update",
  summary = "Update reconciliation composite",
  description = "Applies a header partial update plus a full diff on the adjustments list. Adjustments with `id: Some(uuid)` are updated, `id: None` are inserted, and existing adjustments not present in the request are hard-deleted.",
  path = paths::operations::RECONCILIATIONS_COMPOSITE_BY_ID,
  params(("id" = Uuid, Path)),
  request_body = UpdateInventoryReconciliationCompositeRequest,
  responses(
    (status = 200, body = ApiResponse<InventoryReconciliationCompositeResponse>),
    (status = 400),
    (status = 404)
  )
)]
#[axum::debug_handler]
pub(super) async fn inventory_reconciliation_composite_update(
  State(state): State<Arc<ApiState>>,
  Path(id): Path<Uuid>,
  Valid(Json(req)): Valid<Json<UpdateInventoryReconciliationCompositeRequest>>,
) -> ApiResult<InventoryReconciliationCompositeResponse> {
  Ok(ApiResponse::success(
    state
      .svc
      .document
      .inventory_reconciliation_composite_update(id, &req)
      .await?,
  ))
}

pub(super) fn composite_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(inventory_reconciliation_composite_create))
    .routes(routes!(inventory_reconciliation_composite_update))
    .with_state(state)
}
