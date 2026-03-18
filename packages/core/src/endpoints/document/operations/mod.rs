use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  Extension,
  Json,
};
use axum_valid::Valid;
use serde::Deserialize;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{
    BlendingComponentResponse,
    BlendingCompositeResponse,
    BlendingResponse,
    BlendingResultResponse,
    CreateBlendingComponentRequest,
    CreateBlendingCompositeRequest,
    CreateBlendingRequest,
    CreateBlendingResultRequest,
    CreateInventoryAdjustmentRequest,
    CreateInventoryReconciliationRequest,
    CreateOwnershipTransferItemRequest,
    CreateOwnershipTransferRequest,
    CreatePhysicalTransferItemRequest,
    CreatePhysicalTransferRequest,
    InventoryAdjustmentResponse,
    InventoryReconciliationResponse,
    OwnershipTransferItemResponse,
    OwnershipTransferResponse,
    PhysicalTransferItemResponse,
    PhysicalTransferResponse,
    UpdateBlendingComponentRequest,
    UpdateBlendingRequest,
    UpdateBlendingResultRequest,
    UpdateInventoryAdjustmentRequest,
    UpdateInventoryReconciliationRequest,
    UpdateOwnershipTransferItemRequest,
    UpdateOwnershipTransferRequest,
    UpdatePhysicalTransferItemRequest,
    UpdatePhysicalTransferRequest,
  },
  endpoints::{
    paths,
    query::PaginationParams,
  },
  enums,
  services::common::{ensure_senior_supervisor_or_higher, ensure_supervisor_or_higher},
  utils::jwt::Claims,
};

mod blending;
mod ownership;
mod physical;
mod reconciliation;

pub fn operations_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(physical::physical_routes(state.clone()))
    .merge(ownership::ownership_routes(state.clone()))
    .merge(blending::blending_routes(state.clone()))
    .merge(reconciliation::reconciliation_routes(state))
}
