use std::sync::Arc;

use utoipa_axum::router::OpenApiRouter;

use crate::api::ApiState;

mod composite;
mod manifest;
mod measurement;
mod waybill;
mod weight;

pub fn rail_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(waybill::waybill_routes(state.clone()))
    .merge(manifest::manifest_routes(state.clone()))
    .merge(measurement::measurement_routes(state.clone()))
    .merge(weight::weight_routes(state.clone()))
    .merge(composite::composite_routes(state))
}
