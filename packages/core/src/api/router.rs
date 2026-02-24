use std::sync::Arc;

use axum::Router;
use tower_http::cors::CorsLayer;
use utoipa_axum::router::OpenApiRouter;

use crate::api::state::ApiState;
use crate::endpoints;

pub fn build_router(state: Arc<ApiState>) -> Router {
  let cors = CorsLayer::new()
    .allow_methods(tower_http::cors::Any)
    .allow_headers(tower_http::cors::Any)
    .allow_origin(tower_http::cors::Any);

  OpenApiRouter::new()
    .merge(endpoints::health::health_routes())
    .merge(endpoints::auth::auth_routes(state.clone()))
    .merge(endpoints::user::user_routes(state.clone()))
    .layer(cors)
    .into()
}
