use std::sync::Arc;

use utoipa_axum::router::OpenApiRouter;

use crate::api::ApiState;

mod company;
mod product;
mod product_group;
mod product_type;

pub fn catalog_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .merge(company::company_routes(state.clone()))
    .merge(product_type::product_type_routes(state.clone()))
    .merge(product_group::product_group_routes(state.clone()))
    .merge(product::product_routes(state))
}
