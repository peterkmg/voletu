use super::*;

mod document;
mod transfer;

pub(super) fn physical_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(document::document_routes(state.clone()))
    // Standalone item CRUD disabled — items managed through composite endpoints.
    .merge(transfer::transfer_routes(state))
}
