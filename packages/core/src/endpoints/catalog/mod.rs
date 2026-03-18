use std::sync::Arc;

use utoipa_axum::router::OpenApiRouter;

use crate::api::ApiState;

pub mod catalog;
pub mod topology;

pub fn catalog_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(catalog::catalog_routes(state.clone()))
    .merge(topology::topology_routes(state))
}
