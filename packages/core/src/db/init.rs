use sea_orm::{
  entity::prelude::ChronoUtc, ActiveModelTrait, ActiveValue::Set, ConnectOptions, Database,
  DatabaseConnection, TransactionTrait,
};
use strum::VariantArray;
use tracing::{debug, log::LevelFilter, trace};
use uuid::Uuid;

use crate::{
  config::{DatabaseType, DbConfig, NodeConfig},
  constants::{DEFAULT_ADMIN_PASSWORD, DEFAULT_ADMIN_USERNAME, DEFAULT_DATABASE_COMMON_NAME},
  context::audit::with_audit_context,
  entities::{database_instance, local, role, user},
  enums,
  services::system::{
    database_instance::load_active_database_instance, local::load_local_bootstrap,
  },
  utils::{jwt, password::hash_password, paths::ensure_parent_dir},
};

pub async fn seed_defaults(db: &DatabaseConnection) -> anyhow::Result<local::Model> {
  debug!("Bootstrapping default data...");
  let txn = db.begin().await?;

  for role_type in enums::RoleType::VARIANTS {
    let m = role::ActiveModel {
      id: Set(role_type.uuid()),
      common_name: Set(*role_type),
    };
    m.insert(&txn).await?;
    trace!("Seeded {} role.", role_type);
  }
  debug!("Roles seeded.");

  let bootstrap_db_id = Uuid::now_v7();
  let bootstrap_admin_id = Uuid::now_v7();
  let now = ChronoUtc::now();

  // Wrap syncable entity inserts in an audit context so they produce audit logs.
  // Without this, database_instance and user records would never sync to other nodes.
  let (_instance, local) = with_audit_context(bootstrap_admin_id, bootstrap_db_id, || async {
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

    // local is not synced (excluded from audit), but must be inserted in same transaction
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

    Ok::<_, anyhow::Error>((instance, local))
  })
  .await?;

  txn.commit().await?;

  debug!("Seeded local settings.");

  Ok(local)
}

pub async fn init_database(cfg: &DbConfig) -> anyhow::Result<(DatabaseConnection, NodeConfig)> {
  trace!("Initializing database options...");

  let connection_url = cfg
    .connection_url()
    .map_err(|err| anyhow::anyhow!("Invalid database configuration: {err}"))?;
  let mut options = ConnectOptions::new(connection_url);
  options.sqlx_logging(true);
  options.sqlx_logging_level(LevelFilter::Trace);

  if matches!(cfg.params.db_type, DatabaseType::SQLite) {
    if let Some(file) = &cfg.params.file {
      ensure_parent_dir(file)?;
    }
    if !cfg.params.sqlite_in_memory && cfg.params.sqlite_shared_memory_name.is_none() {
      options.sqlcipher_key(cfg.password.clone());
    }
    // For in-memory / shared-memory SQLite, constrain the connection pool
    // to a single connection. SQLite's shared-cache mode has known
    // cross-connection visibility quirks where a commit on one pooled
    // connection isn't consistently seen by SELECTs on another pooled
    // connection — readers can remain pinned to an earlier snapshot.
    // Serializing all access through one connection eliminates that class
    // of races. File-based SQLite under WAL gets normal snapshot semantics
    // and keeps the default pool size.
    if cfg.params.sqlite_in_memory || cfg.params.sqlite_shared_memory_name.is_some() {
      options.max_connections(1);
      options.min_connections(1);
    }
  }

  trace!("Connecting to database...");
  let db = Database::connect(options).await?;
  trace!("Database connection established.");

  trace!("Synchronizing database schema...");
  db.get_schema_registry("voletu-core::entities::*")
    .sync(&db)
    .await?;
  trace!("Database schema synchronized.");

  let local = match load_local_bootstrap(&db).await {
    Ok(existing) => existing,
    Err(crate::api::ApiError::NotFound(_)) => seed_defaults(&db).await?,
    Err(other) => return Err(anyhow::anyhow!(other.to_string())),
  };

  let instance = load_active_database_instance(&db, local.local_db_id)
    .await
    .map_err(|err| anyhow::anyhow!(err.to_string()))?;

  Ok((
    db,
    NodeConfig::new(
      local.local_db_id,
      instance.node_type.to_string(),
      local.jwt_secret,
      local
        .central_api_url
        .as_ref()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty()),
    ),
  ))
}
