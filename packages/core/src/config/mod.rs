mod database;
mod jwt;
mod logging;
mod node;
mod sync;

pub use database::{DatabaseType, DbConfig, DbParams};
pub use jwt::JwtConfig;
pub use logging::LoggingConfig;
pub use node::NodeConfig;
pub use sync::SyncConfig;

#[derive(Clone)]
pub struct ApiConfig {
  pub node: NodeConfig,
  pub jwt: JwtConfig,
  pub db: DbConfig,
}

impl ApiConfig {
  pub fn new(node: NodeConfig, jwt: JwtConfig, db: DbConfig) -> Self {
    Self { node, jwt, db }
  }
}
