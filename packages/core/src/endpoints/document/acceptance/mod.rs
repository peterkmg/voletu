use std::sync::Arc;

use utoipa_axum::{router::OpenApiRouter, routes};

use crate::api::ApiState;

mod composite;
mod document;
mod item;

pub fn acceptance_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(
      document::acceptance_document_list,
      document::acceptance_document_create
    ))
    .routes(routes!(document::acceptance_document_create_and_execute))
    .routes(routes!(document::acceptance_document_query))
    .routes(routes!(
      document::acceptance_document_get,
      document::acceptance_document_update
    ))
    .routes(routes!(document::acceptance_document_soft_delete))
    .routes(routes!(document::acceptance_document_hard_delete))
    .routes(routes!(document::acceptance_document_execute))
    .routes(routes!(document::acceptance_document_revert))
    .routes(routes!(composite::acceptance_composite_get))
    .routes(routes!(composite::acceptance_composite_create))
    .routes(routes!(composite::acceptance_composite_create_and_execute))
    .routes(routes!(item::acceptance_item_list, item::acceptance_item_create))
    .routes(routes!(item::acceptance_item_get, item::acceptance_item_update))
    .routes(routes!(item::acceptance_item_soft_delete))
    .routes(routes!(item::acceptance_item_hard_delete))
    .with_state(state)
}
