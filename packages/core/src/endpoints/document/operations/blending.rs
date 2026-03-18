use super::*;

mod component;
mod composite;
mod document;
mod result;

pub(super) fn blending_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(document::document_routes(state.clone()))
    .merge(component::component_routes(state.clone()))
    .merge(result::result_routes(state.clone()))
    .merge(composite::composite_routes(state))
}
