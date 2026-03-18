use std::sync::Arc;

use utoipa_axum::router::OpenApiRouter;

use crate::api::ApiState;

pub mod acceptance;
pub mod dispatch;
pub mod operations;
pub mod transport;

pub fn document_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(transport::truck::truck_routes(state.clone()))
    .merge(transport::rail::rail_routes(state.clone()))
    .merge(acceptance::acceptance_routes(state.clone()))
    .merge(dispatch::dispatch_routes(state.clone()))
    .merge(operations::operations_routes(state))
}
