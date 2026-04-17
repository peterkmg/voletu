use super::*;

mod adjustment;
mod composite;
mod document;

pub(super) fn reconciliation_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(document::reconciliation_routes(state.clone()))
    .merge(adjustment::adjustment_routes(state.clone()))
    .merge(composite::composite_routes(state))
}
