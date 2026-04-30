use std::sync::Arc;

use utoipa_axum::router::OpenApiRouter;

use crate::api::ApiState;

mod composite;
mod document;
mod measurement;

pub fn dispatch_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(document::document_routes(state.clone()))
    .merge(composite::composite_routes(state.clone()))
    .merge(measurement::measurement_routes(state))
}
