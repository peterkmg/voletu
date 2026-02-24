use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ConnectOptions, Database, DatabaseConnection, EntityTrait,
};
use tracing::{debug, log::LevelFilter, trace};
use uuid::Uuid;

use crate::{
  config::{DatabaseType, DbConfig, NodeConfig},
  constants::{DEFAULT_ADMIN_USERNAME, DEFAULT_DATABASE_COMMON_NAME, DEFAULT_DATABASE_NODE_TYPE},
  entities::{database_instance, local, role, user},
  utils::{jwt, password::hash_password, paths::ensure_parent_dir},
};

pub async fn seed_defaults(db: &DatabaseConnection) -> anyhow::Result<local::Model> {
  debug!("Bootstrapping default data...");
  for role_type in role::RoleType::all() {
    let m = role::ActiveModel {
      id: Set(role_type.uuid()),
      common_name: Set(role_type.clone()),
    };
    m.insert(db).await?;
    trace!("Seeded {} role.", role_type.as_str());
  }
  debug!("Roles seeded.");

  let _ = user::ActiveModel {
    username: Set(DEFAULT_ADMIN_USERNAME.to_string()),
    password_hash: Set(hash_password("admin").await?),
    role_id: Set(role::RoleType::Admin.uuid()),
    ..Default::default()
  }
  .insert(db)
  .await?;

  let instance = database_instance::ActiveModel {
    common_name: Set(DEFAULT_DATABASE_COMMON_NAME.to_string()),
    node_type: Set(DEFAULT_DATABASE_NODE_TYPE.to_string()),
    base_id: Set(None),
    ..Default::default()
  }
  .insert(db)
  .await?;

  debug!("Seeded default database instance with id {}.", instance.id);

  let local = local::ActiveModel {
    local_db_id: Set(instance.id),
    is_initialized: Set(false),
    jwt_secret: Set(jwt::generate_secret()),
    ..Default::default()
  }
  .insert(db)
  .await?;

  debug!("Seeded local settings.");

  Ok(local)
}

pub async fn init_database(cfg: &DbConfig) -> anyhow::Result<(DatabaseConnection, NodeConfig)> {
  trace!("Initializing database options...");

  let mut options = ConnectOptions::new(cfg.connection_url());
  options.sqlx_logging(true);
  options.sqlx_logging_level(LevelFilter::Trace);

  if matches!(cfg.params.db_type, DatabaseType::SQLite) {
    if let Some(file) = &cfg.params.file {
      ensure_parent_dir(file)?;
    }
    options.sqlcipher_key(cfg.password.clone());
  }

  trace!("Connecting to database...");
  let db = Database::connect(options).await?;
  trace!("Database connection established.");

  trace!("Synchronizing database schema...");
  db.get_schema_registry("voletu-core::entities::*")
    .sync(&db)
    .await?;
  trace!("Database schema synchronized.");

  let local = local::Entity::find()
    .one(&db)
    .await?
    .unwrap_or(seed_defaults(&db).await?);

  Ok((db, NodeConfig {
    database_id: Uuid::from(local.local_db_id),
    jwt_secret: local.jwt_secret,
  }))
}
