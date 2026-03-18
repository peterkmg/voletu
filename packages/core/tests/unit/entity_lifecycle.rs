use sea_orm::{ActiveModelBehavior, ActiveValue::Set, DatabaseBackend, MockDatabase};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  entities::{company, ownership_transfer, user},
  enums,
};

#[tokio::test]
async fn user_before_save_populates_id_and_timestamps_for_insert() {
  let db = MockDatabase::new(DatabaseBackend::Sqlite).into_connection();

  let am = user::ActiveModel {
    username: Set("tester".to_string()),
    password_hash: Set("hash".to_string()),
    role_id: Set(enums::RoleType::Admin.uuid()),
    ..Default::default()
  };

  let saved = with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    am.before_save(&db, true).await.unwrap()
  })
  .await;

  assert!(matches!(saved.id, Set(_)));
  assert!(matches!(saved.created_at, Set(_)));
  assert!(matches!(saved.updated_at, Set(_)));
  assert!(matches!(saved.origin_db_id, Set(_)));
}

#[tokio::test]
async fn company_before_save_populates_id_and_timestamps_for_insert() {
  let db = MockDatabase::new(DatabaseBackend::Sqlite).into_connection();

  let am = company::ActiveModel {
    common_name: Set("Company A".to_string()),
    legal_name: Set(Some("Company A LLC".to_string())),
    is_contractor: Set(true),
    is_exporter: Set(false),
    is_manufacturer: Set(false),
    is_sender: Set(true),
    ..Default::default()
  };

  let saved = with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    am.before_save(&db, true).await.unwrap()
  })
  .await;

  assert!(matches!(saved.id, Set(_)));
  assert!(matches!(saved.created_at, Set(_)));
  assert!(matches!(saved.updated_at, Set(_)));
  assert!(matches!(saved.origin_db_id, Set(_)));
}

#[tokio::test]
async fn database_instance_before_save_populates_uuid_and_timestamps_for_insert() {
  let db = MockDatabase::new(DatabaseBackend::Sqlite).into_connection();

  let am = voletu_core::entities::database_instance::ActiveModel {
    common_name: Set("local".to_string()),
    node_type: Set(enums::NodeType::Peripheral),
    base_id: Set(None),
    ..Default::default()
  };

  let saved = with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    am.before_save(&db, true).await.unwrap()
  })
  .await;

  assert!(matches!(saved.id, Set(_)));
  assert!(matches!(saved.created_at, Set(_)));
  assert!(matches!(saved.updated_at, Set(_)));
  assert!(matches!(saved.origin_db_id, Set(_)));
}

#[tokio::test]
async fn versioned_model_before_save_sets_version_on_insert() {
  let db = MockDatabase::new(DatabaseBackend::Sqlite).into_connection();

  let am = ownership_transfer::ActiveModel {
    date: Set(chrono::Utc::now()),
    status: Set(enums::DocumentStatus::Draft),
    executed_at: Set(None),
    executed_by: Set(None),
    reverted_at: Set(None),
    reverted_by: Set(None),
    ..Default::default()
  };

  let saved = with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    am.before_save(&db, true).await.unwrap()
  })
  .await;

  assert!(matches!(saved.version, Set(1)));
}

#[tokio::test]
async fn versioned_model_before_save_increments_version_on_update() {
  let db = MockDatabase::new(DatabaseBackend::Sqlite).into_connection();

  let am = ownership_transfer::ActiveModel {
    id: Set(Uuid::now_v7()),
    date: Set(chrono::Utc::now()),
    status: Set(enums::DocumentStatus::Draft),
    version: Set(7),
    executed_at: Set(None),
    executed_by: Set(None),
    reverted_at: Set(None),
    reverted_by: Set(None),
    ..Default::default()
  };
  let saved = with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    am.before_save(&db, false).await.unwrap()
  })
  .await;

  assert!(matches!(saved.version, Set(8)));
}
