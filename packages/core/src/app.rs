use std::sync::Arc;

use anyhow::Result;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, DatabaseConnection};
use tower_http::cors::CorsLayer;
use utoipa_axum::router::OpenApiRouter;

use crate::{
  config::AppConfig,
  database::init_database,
  entities::{role, user},
  endpoints,
  services::{auth::AuthService, jwt::JwtService, user::UserService},
};

#[derive(Clone, Debug)]
pub struct StartupPreflight {
  pub has_superadmin_user: bool,
}

async fn ensure_role_exists(db: &DatabaseConnection, role_name: &str) -> Result<()> {
  let existing = role::Entity::load().filter_by_name(role_name).one(db).await?;
  if existing.is_none() {
    let mut model = role::ActiveModel::new();
    model.name = sea_orm::ActiveValue::Set(role_name.to_string());
    model.insert(db).await?;
    tracing::info!(role = %role_name, "Seeded default role");
  }
  Ok(())
}

async fn seed_default_roles(db: &DatabaseConnection) -> Result<()> {
  ensure_role_exists(db, "admin").await?;
  ensure_role_exists(db, "superadmin").await?;
  Ok(())
}

async fn prepare_database(db: &DatabaseConnection) -> Result<()> {
  tracing::info!("Running database migrations");
  db.get_schema_registry("voletu-core::entities::*")
    .sync(db)
    .await?;

  seed_default_roles(db).await
}

pub async fn preflight_startup(cfg: &AppConfig) -> Result<StartupPreflight> {
  tracing::info!("Running startup preflight");
  let db = init_database(&cfg.db).await?;
  prepare_database(&db).await?;

  let users = user::Entity::load().with(role::Entity).all(&db).await?;
  let has_superadmin_user = users.iter().any(|entry| {
    entry
      .role
      .as_ref()
      .is_some_and(|assigned_role| assigned_role.name == "superadmin")
  });

  Ok(StartupPreflight {
    has_superadmin_user,
  })
}

#[derive(Clone)]
pub struct AppState {
  pub cfg: AppConfig,
  pub db: Arc<DatabaseConnection>,
  pub jwt_service: Arc<JwtService>,
  pub auth_service: Arc<AuthService>,
  pub user_service: Arc<UserService>,
}

impl AppState {
  pub async fn new(cfg: AppConfig) -> Result<Self> {
    tracing::info!("Connecting to database: {}", cfg.db.connection_url());
    let db = Arc::new(init_database(&cfg.db).await?);

    tracing::info!("Database connected");
    prepare_database(db.as_ref()).await?;

    let jwt_service = Arc::new(JwtService::new(&cfg.jwt));
    let auth_service = Arc::new(AuthService::new(db.clone(), jwt_service.clone()));
    let user_service = Arc::new(UserService::new(db.clone()));

    Ok(Self {
      cfg,
      db,
      jwt_service,
      auth_service,
      user_service,
    })
  }
}

/// Initialise the API and return an `OpenApiRouter` ready to be served.
/// The caller is responsible for binding and running the server.
pub async fn init_api(cfg: AppConfig) -> Result<OpenApiRouter> {
  tracing::info!("Initialising API");
  let state = Arc::new(AppState::new(cfg).await?);

  // state
  //   .db
  //   .get_schema_registry("voletu::entities::*")
  //   .sync(&state.db)
  //   .await?;

  let cors = CorsLayer::new()
    .allow_methods(tower_http::cors::Any)
    .allow_headers(tower_http::cors::Any)
    .allow_origin(tower_http::cors::Any);

  let router = OpenApiRouter::new()
    .merge(endpoints::auth::auth_routes(state.clone()))
    .merge(endpoints::user::user_routes(state.clone()))
    .layer(cors);

  tracing::info!("API router ready");
  Ok(router)
}
