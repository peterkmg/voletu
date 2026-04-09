use std::sync::Arc;

use axum::{extract::State, Json};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{LedgerEntryLookupRequest, LedgerEntryResponse},
  endpoints::paths,
};

#[utoipa::path(
  get,
  tag = "Ledger",
  operation_id = "ledger_entry_list",
  summary = "List ledger entries",
  description = "Returns inventory ledger entries used for operational balance and audit visibility.",
  path = paths::ledger::ROOT,
  responses(
    (status = 200, body = ApiResponse<Vec<LedgerEntryResponse>>, description = "Ledger entry list envelope. Example: {\"success\":true,\"data\":[{\"storageId\":\"...\",\"productId\":\"...\",\"quantity\":125.5}]}" )
  )
)]
#[axum::debug_handler]
async fn entry_list(State(state): State<Arc<ApiState>>) -> ApiResult<Vec<LedgerEntryResponse>> {
  Ok(ApiResponse::success(state.svc.ledger.list().await?))
}

#[utoipa::path(
  post,
  tag = "Ledger",
  operation_id = "ledger_entry_get_by_dimensions",
  summary = "Get ledger entry by dimensions",
  description = "Returns the current ledger entry for a storage, product, and contractor tuple.",
  path = paths::ledger::QUERY,
  request_body = LedgerEntryLookupRequest,
  responses(
    (status = 200, body = ApiResponse<Option<LedgerEntryResponse>>, description = "Lookup envelope. Example: {\"success\":true,\"data\":{\"storageId\":\"...\",\"productId\":\"...\",\"quantity\":80.0}}"),
    (status = 400, description = "Validation envelope for malformed request payload.")
  )
)]
#[axum::debug_handler]
async fn entry_get(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<LedgerEntryLookupRequest>>,
) -> ApiResult<Option<LedgerEntryResponse>> {
  Ok(ApiResponse::success(
    state
      .svc
      .ledger
      .by_dimensions(req.storage_id, req.product_id, req.contractor_id)
      .await?,
  ))
}

pub fn ledger_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(entry_list))
    .routes(routes!(entry_get))
    .with_state(state)
}
