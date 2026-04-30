use std::sync::Arc;

use utoipa_axum::router::OpenApiRouter;

use crate::api::ApiState;

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
