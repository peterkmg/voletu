use sea_orm::{
  entity::prelude::ChronoUtc,
  ActiveModelTrait,
  ActiveValue::Set,
  ConnectOptions,
  Database,
  DatabaseConnection,
  EntityTrait,
  TransactionTrait,
};
use tracing::{debug, log::LevelFilter, trace};
use uuid::Uuid;

use crate::{
  config::{DatabaseType, DbConfig, NodeConfig},
  constants::{DEFAULT_ADMIN_PASSWORD, DEFAULT_ADMIN_USERNAME, DEFAULT_DATABASE_COMMON_NAME},
  entities::{database_instance, enums, local, role, user},
  utils::{jwt, password::hash_password, paths::ensure_parent_dir},
};

pub async fn seed_defaults(db: &DatabaseConnection) -> anyhow::Result<local::Model> {
  debug!("Bootstrapping default data...");
  let txn = db.begin().await?;
  let bootstrap_db_id = Uuid::now_v7();
  let bootstrap_admin_id = Uuid::now_v7();
  let now = ChronoUtc::now();

  for role_type in enums::RoleType::all() {
    let m = role::ActiveModel {
      id: Set(role_type.uuid()),
      common_name: Set(role_type.clone()),
    };
    m.insert(&txn).await?;
    trace!("Seeded {} role.", role_type.as_str());
  }
  debug!("Roles seeded.");

  let instance = database_instance::ActiveModel {
    id: Set(bootstrap_db_id),
    common_name: Set(DEFAULT_DATABASE_COMMON_NAME.to_string()),
    node_type: Set(enums::NodeType::Peripheral),
    base_id: Set(None),
    created_at: Set(now),
    updated_at: Set(now),
    deleted_at: Set(None),
    created_by: Set(bootstrap_admin_id),
    updated_by: Set(bootstrap_admin_id),
    deleted_by: Set(None),
    origin_db_id: Set(bootstrap_db_id),
  }
  .insert(&txn)
  .await?;

  debug!("Seeded default database instance with id {}.", instance.id);

  let local = local::ActiveModel {
    local_db_id: Set(instance.id),
    is_initialized: Set(false),
    jwt_secret: Set(jwt::generate_secret()),
    ..Default::default()
  }
  .insert(&txn)
  .await?;

  let _ = user::ActiveModel {
    id: Set(bootstrap_admin_id),
    username: Set(DEFAULT_ADMIN_USERNAME.to_string()),
    password_hash: Set(hash_password(DEFAULT_ADMIN_PASSWORD).await?),
    role_id: Set(enums::RoleType::Admin.uuid()),
    created_at: Set(now),
    updated_at: Set(now),
    deleted_at: Set(None),
    created_by: Set(bootstrap_admin_id),
    updated_by: Set(bootstrap_admin_id),
    deleted_by: Set(None),
    origin_db_id: Set(bootstrap_db_id),
    ..Default::default()
  }
  .insert(&txn)
  .await?;

  txn.commit().await?;

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

  let local = match local::Entity::find().one(&db).await? {
    Some(existing) => existing,
    None => seed_defaults(&db).await?,
  };

  Ok((db, NodeConfig {
    database_id: Uuid::from(local.local_db_id),
    jwt_secret: local.jwt_secret,
  }))
}
