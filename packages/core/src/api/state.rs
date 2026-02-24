use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
  config::ApiConfig,
  services::{auth::AuthService, token::TokenService, user::UserService},
};

#[derive(Clone)]
pub struct ApiState {
  pub cfg: ApiConfig,
  pub db: Arc<DatabaseConnection>,
  pub jwt_service: Arc<TokenService>,
  pub auth_service: Arc<AuthService>,
  pub user_service: Arc<UserService>,
}

impl ApiState {
  pub(crate) fn build(cfg: ApiConfig, db: Arc<DatabaseConnection>) -> Self {
    let token_service = Arc::new(TokenService::new(&cfg));
    let auth_service = Arc::new(AuthService::new(db.clone(), token_service.clone()));
    let user_service = Arc::new(UserService::new(db.clone()));
    Self {
      cfg,
      db,
      jwt_service: token_service,
      auth_service,
      user_service,
    }
  }
}
