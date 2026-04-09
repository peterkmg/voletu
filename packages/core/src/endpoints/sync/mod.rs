use std::sync::Arc;

use axum::{
  extract::{Query, State},
  Json,
};
use axum_valid::Valid;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  api::{ApiResponse, ApiResult, ApiState},
  dtos::{
    AuditLogResponse,
    PullAuditLogsResponse,
    PushAuditLogRequest,
    PushAuditLogsRequest,
    PushAuditLogsResponse,
    SyncWatermarkResponse,
    UpsertWatermarkRequest,
  },
  endpoints::paths,
};

mod audit;
mod status;
mod transfer;
mod watermark;

pub fn sync_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(status::status_routes(state.clone()))
    .merge(audit::audit_routes(state.clone()))
    .merge(watermark::watermark_routes(state.clone()))
    .merge(transfer::transfer_routes(state))
}
