use super::*;

mod document;
mod item;
mod transfer;

pub(super) fn ownership_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(document::document_routes(state.clone()))
    .merge(item::item_routes(state.clone()))
    .merge(transfer::transfer_routes(state))
}
