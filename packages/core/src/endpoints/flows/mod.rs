use std::sync::Arc;

use utoipa_axum::router::OpenApiRouter;

use crate::api::ApiState;

pub mod rail_receipt;
pub mod truck_dispatch;
pub mod truck_receipt;

pub fn flow_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(truck_receipt::truck_receipt_routes(state.clone()))
    .merge(rail_receipt::rail_receipt_routes(state.clone()))
    .merge(truck_dispatch::truck_dispatch_routes(state))
}
