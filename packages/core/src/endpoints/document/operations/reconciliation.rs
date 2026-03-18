use super::*;

mod adjustment;
mod reconciliation;

pub(super) fn reconciliation_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(reconciliation::reconciliation_routes(state.clone()))
    .merge(adjustment::adjustment_routes(state))
}
