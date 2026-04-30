use std::sync::Arc;

use utoipa_axum::router::OpenApiRouter;

use crate::api::ApiState;

mod base;
mod port;
mod storage;
mod warehouse;

pub fn topology_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(base::base_routes(state.clone()))
    .merge(warehouse::warehouse_routes(state.clone()))
    .merge(storage::storage_routes(state.clone()))
    .merge(port::port_routes(state))
}
