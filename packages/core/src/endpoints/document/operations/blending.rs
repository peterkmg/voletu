use super::*;

mod composite;
mod document;

pub(super) fn blending_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(document::document_routes(state.clone()))
    // Standalone component/result CRUD disabled — managed through composite endpoints.
    .merge(composite::composite_routes(state))
}
