use std::sync::Arc;

use utoipa_axum::router::OpenApiRouter;

use crate::api::ApiState;

pub mod items;
pub mod topology;

pub fn catalog_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(items::catalog_routes(state.clone()))
    .merge(topology::topology_routes(state))
}
