use std::sync::Arc;

use axum::{http::Uri, middleware, Router};
use tower_http::cors::CorsLayer;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
  api::{state::ApiState, ApiError},
  endpoints,
  middleware::{
    auth::auth_middleware,
    error::error_middleware,
    idempotency::idempotency_middleware,
  },
};

pub fn build_router(state: Arc<ApiState>) -> Router {
  let cors = CorsLayer::new()
    .allow_methods(tower_http::cors::Any)
    .allow_headers(tower_http::cors::Any)
    .allow_origin(tower_http::cors::Any);

  let public = OpenApiRouter::new()
    .merge(endpoints::health::health_routes())
    .merge(endpoints::auth::auth_public_routes(state.clone()))
    .merge(endpoints::sync::sync_routes(state.clone()));

  let protected = OpenApiRouter::new()
    .merge(endpoints::auth::auth_protected_routes(state.clone()))
    .merge(endpoints::lifecycle::lifecycle_routes(state.clone()))
    .merge(endpoints::user::user_routes(state.clone()))
    .merge(endpoints::catalog::catalog_routes(state.clone()))
    .merge(endpoints::document::document_routes(state.clone()))
    .merge(endpoints::ledger::ledger_routes(state.clone()));

  let (router, api) = OpenApiRouter::new()
    .merge(public)
    .merge(protected.layer(middleware::from_fn_with_state(
      state.clone(),
      auth_middleware,
    )))
    .layer(middleware::from_fn_with_state(
      state.clone(),
      idempotency_middleware,
    ))
    .layer(cors)
    .split_for_parts();

  Router::new()
    .merge(router)
    .merge(
      SwaggerUi::new(endpoints::paths::docs::SWAGGER_UI)
        .url(endpoints::paths::docs::OPENAPI_JSON, api),
    )
    .fallback(async |uri: Uri| ApiError::NotFound(format!("'{}' does not exist", uri.path())))
    .layer(middleware::from_fn(error_middleware))
}
