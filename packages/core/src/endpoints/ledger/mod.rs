use std::sync::Arc;

use axum::{extract::State, Json};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{LedgerBalanceLookupRequest, LedgerBalanceResponse},
  endpoints::paths,
};

#[utoipa::path(
  get,
  tag = "Ledger",
  operation_id = "ledger_balance_list",
  summary = "List ledger balances",
  description = "Returns derived inventory balances grouped by storage, product, and contractor.",
  path = paths::ledger::ROOT,
  responses(
    (status = 200, body = ApiResponse<Vec<LedgerBalanceResponse>>, description = "Ledger balance list envelope. Example: {\"success\":true,\"data\":[{\"storageId\":\"...\",\"productId\":\"...\",\"quantity\":125.5}]}" )
  )
)]
#[axum::debug_handler]
async fn balance_list(State(state): State<Arc<ApiState>>) -> ApiResult<Vec<LedgerBalanceResponse>> {
  Ok(ApiResponse::success(
    state.svc.ledger.list_balances().await?,
  ))
}

#[utoipa::path(
  post,
  tag = "Ledger",
  operation_id = "ledger_balance_get_by_dimensions",
  summary = "Get ledger balance by dimensions",
  description = "Returns the derived inventory balance for a storage, product, and contractor tuple.",
  path = paths::ledger::QUERY,
  request_body = LedgerBalanceLookupRequest,
  responses(
    (status = 200, body = ApiResponse<Option<LedgerBalanceResponse>>, description = "Lookup envelope. Example: {\"success\":true,\"data\":{\"storageId\":\"...\",\"productId\":\"...\",\"quantity\":80.0}}"),
    (status = 400, description = "Validation envelope for malformed request payload.")
  )
)]
#[axum::debug_handler]
async fn balance_get(
  State(state): State<Arc<ApiState>>,
  Valid(Json(req)): Valid<Json<LedgerBalanceLookupRequest>>,
) -> ApiResult<Option<LedgerBalanceResponse>> {
  Ok(ApiResponse::success(
    state
      .svc
      .ledger
      .balance_by_dimensions(req.storage_id, req.product_id, req.contractor_id)
      .await?,
  ))
}

pub fn ledger_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(balance_list))
    .routes(routes!(balance_get))
    .with_state(state)
}
