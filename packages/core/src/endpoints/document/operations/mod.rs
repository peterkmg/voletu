use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  Extension,
  Json,
};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{
    BlendingCompositeResponse,
    BlendingResponse,
    CreateBlendingCompositeRequest,
    CreateBlendingRequest,
    CreateInventoryAdjustmentRequest,
    CreateInventoryReconciliationCompositeRequest,
    CreateInventoryReconciliationRequest,
    CreateOwnershipTransferRequest,
    CreatePhysicalTransferRequest,
    EmbedParams,
    InventoryAdjustmentResponse,
    InventoryReconciliationCompositeResponse,
    InventoryReconciliationResponse,
    OwnershipTransferResponse,
    PhysicalTransferResponse,
    UpdateBlendingRequest,
    UpdateInventoryAdjustmentRequest,
    UpdateInventoryReconciliationCompositeRequest,
    UpdateInventoryReconciliationRequest,
    UpdateOwnershipTransferRequest,
    UpdatePhysicalTransferRequest,
  },
  endpoints::paths,
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
