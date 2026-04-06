use std::sync::Arc;

use utoipa_axum::router::OpenApiRouter;

use crate::api::ApiState;

pub mod acceptance;
pub mod blending;
pub mod cargo_flow;
pub mod dispatch;
pub mod ownership_transfer;
pub mod physical_transfer;
pub mod rail_receipt;
pub mod reconciliation;
pub mod truck_dispatch;
pub mod truck_receipt;

pub fn flow_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(truck_receipt::truck_receipt_routes(state.clone()))
    .merge(rail_receipt::rail_receipt_routes(state.clone()))
    .merge(truck_dispatch::truck_dispatch_routes(state.clone()))
    .merge(acceptance::acceptance_flat_routes(state.clone()))
    .merge(dispatch::dispatch_flat_routes(state.clone()))
    .merge(physical_transfer::physical_transfer_flat_routes(
      state.clone(),
    ))
    .merge(ownership_transfer::ownership_transfer_flat_routes(
      state.clone(),
    ))
    .merge(blending::blending_flat_routes(state.clone()))
    .merge(reconciliation::reconciliation_flat_routes(state.clone()))
    .merge(cargo_flow::cargo_flow_routes(state))
}
