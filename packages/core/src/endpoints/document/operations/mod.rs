use std::sync::Arc;

use utoipa_axum::router::OpenApiRouter;

use crate::api::ApiState;

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
