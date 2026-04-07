use std::sync::Arc;

use assert_json_diff::assert_json_eq;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  entities::{audit_log, base},
  enums::{self, AuditAction},
  services::sync::SyncService,
};

use crate::common::{
  catalog_seed::{seed_inventory_catalog, seed_sync_node},
  setup_db,
  test_config,
};

const TEST_SYNC_NODE_ID: Uuid = Uuid::from_u128(11);
const INITIAL_AUDIT_CURSOR: Uuid = Uuid::from_u128(1);

fn ts(value: &str) -> DateTime<Utc> {
  DateTime::parse_from_rfc3339(value)
    .unwrap()
    .with_timezone(&Utc)
}

fn sync_service_with_node(db: Arc<sea_orm::DatabaseConnection>, node_id: Uuid) -> SyncService {
  let mut cfg = test_config();
  cfg.node.db_id = node_id;
  SyncService::new(db, Arc::new(cfg))
}

#[tokio::test]
async fn sync_pull_target_matching_is_delimiter_safe_for_base_ids() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);

    let base_target = Uuid::parse_str("00000000-0000-0000-0000-000000000012").unwrap();
    let base_other = Uuid::parse_str("00000000-0000-0000-0000-000000000312").unwrap();

    let _ = base::ActiveModel {
      id: Set(base_target),
      common_name: Set("Base Target".to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let _ = base::ActiveModel {
      id: Set(base_other),
      common_name: Set("Base Other".to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let _node_target = seed_sync_node(&db, base_target, "Node Target").await;

    audit_log::ActiveModel {
      id: Set(Uuid::now_v7()),
      table_name: Set("dispatch_documents".to_string()),
      record_id: Set(Uuid::now_v7()),
      action: Set(enums::AuditAction::Insert),
      old_values: Set(None),
      new_values: Set(None),
      target_base_ids: Set(base_other.to_string()),
      user_role_weight: Set(10),
      user_id: Set(Uuid::now_v7()),
      timestamp: Set(ts("2026-01-01T00:00:00Z")),
      origin_db_id: Set(Uuid::now_v7()),
    }
    .insert(&*db)
    .await
    .unwrap();

    let response = service
      .pull_logs(INITIAL_AUDIT_CURSOR, &[base_target], None)
      .await
      .unwrap();
    // Only targeted (non-global) logs for base_other should be excluded.
    // Global entries (bases, database_instances) may be present.
    let targeted_logs: Vec<_> = response
      .logs
      .iter()
      .filter(|l| l.table_name == "dispatch_documents")
      .collect();
    assert!(
      targeted_logs.is_empty(),
      "dispatch_documents targeted to base_other should not be pulled for base_target"
    );
  })
  .await;
}

#[tokio::test]
async fn sync_outbound_returns_push_payload_shape_with_json_strings() {
  let db = Arc::new(setup_db().await);
  let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);
  let log_id = Uuid::now_v7();

  audit_log::ActiveModel {
    id: Set(log_id),
    table_name: Set("companies".to_string()),
    record_id: Set(Uuid::now_v7()),
    action: Set(enums::AuditAction::Update),
    old_values: Set(Some(serde_json::json!({"name":"Old"}))),
    new_values: Set(Some(serde_json::json!({"name":"New"}))),
    target_base_ids: Set(",00000000-0000-0000-0000-000000000012,".to_string()),
    user_role_weight: Set(10),
    user_id: Set(Uuid::now_v7()),
    timestamp: Set(ts("2026-01-01T00:00:00Z")),
    origin_db_id: Set(Uuid::now_v7()),
  }
  .insert(&*db)
  .await
  .unwrap();

  let outbound = service
    .outbound_logs(INITIAL_AUDIT_CURSOR, Some(100))
    .await
    .unwrap();
  assert_eq!(outbound.len(), 1);
  assert_eq!(outbound[0].id, log_id);
  assert_eq!(outbound[0].action, AuditAction::Update);
  let old_values =
    serde_json::from_str::<serde_json::Value>(outbound[0].old_values_json.as_deref().unwrap())
      .unwrap();
  let new_values =
    serde_json::from_str::<serde_json::Value>(outbound[0].new_values_json.as_deref().unwrap())
      .unwrap();
  assert_json_eq!(old_values, serde_json::json!({ "name": "Old" }));
  assert_json_eq!(new_values, serde_json::json!({ "name": "New" }));
}

#[tokio::test]
async fn sync_pull_empty_scope_advances_to_highest_evaluated_id() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);

    let base_a = base::ActiveModel {
      id: Set(Uuid::parse_str("00000000-0000-0000-0000-000000000012").unwrap()),
      common_name: Set("Base A".to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();
    let base_b = base::ActiveModel {
      id: Set(Uuid::parse_str("00000000-0000-0000-0000-000000000013").unwrap()),
      common_name: Set("Base B".to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let _node_a = seed_sync_node(&db, base_a.id, "Node A").await;

    let _log = audit_log::ActiveModel {
      id: Set(Uuid::now_v7()),
      table_name: Set("dispatch_documents".to_string()),
      record_id: Set(Uuid::now_v7()),
      action: Set(enums::AuditAction::Insert),
      old_values: Set(None),
      new_values: Set(None),
      target_base_ids: Set(base_b.id.to_string()),
      user_role_weight: Set(10),
      user_id: Set(Uuid::now_v7()),
      timestamp: Set(ts("2026-01-05T00:00:00Z")),
      origin_db_id: Set(Uuid::now_v7()),
    }
    .insert(&*db)
    .await
    .unwrap();

    let response = service
      .pull_logs(INITIAL_AUDIT_CURSOR, &[base_a.id], None)
      .await
      .unwrap();
    // Only targeted (non-global) logs for base_b should be excluded.
    let targeted_logs: Vec<_> = response
      .logs
      .iter()
      .filter(|l| l.table_name == "dispatch_documents")
      .collect();
    assert!(targeted_logs.is_empty());
    assert!(
      response.highest_evaluated_id > INITIAL_AUDIT_CURSOR,
      "watermark should advance from initial cursor"
    );
  })
  .await;
}

#[tokio::test]
async fn sync_pull_returns_global_and_targeted_logs_for_requesting_base_scope() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);
    let catalog = seed_inventory_catalog(&db).await;
    let base_a_id = catalog.base_id;
    let base_b = base::ActiveModel {
      common_name: Set("Base B".to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let _node_a_id = seed_sync_node(&db, base_a_id, "Node A").await;

    let global_log_record = Uuid::now_v7();
    let target_a_record = Uuid::now_v7();
    let target_b_record = Uuid::now_v7();
    let global_storage_record = Uuid::now_v7();

    audit_log::ActiveModel {
      id: Set(Uuid::now_v7()),
      table_name: Set("companies".to_string()),
      record_id: Set(global_log_record),
      action: Set(enums::AuditAction::Insert),
      old_values: Set(None),
      new_values: Set(None),
      target_base_ids: Set(String::new()),
      user_role_weight: Set(10),
      user_id: Set(Uuid::now_v7()),
      timestamp: Set(ts("2026-01-01T00:00:00Z")),
      origin_db_id: Set(Uuid::now_v7()),
    }
    .insert(&*db)
    .await
    .unwrap();

    audit_log::ActiveModel {
      id: Set(Uuid::now_v7()),
      table_name: Set("dispatch_documents".to_string()),
      record_id: Set(target_a_record),
      action: Set(enums::AuditAction::Insert),
      old_values: Set(None),
      new_values: Set(None),
      target_base_ids: Set(base_a_id.to_string()),
      user_role_weight: Set(10),
      user_id: Set(Uuid::now_v7()),
      timestamp: Set(ts("2026-01-01T00:01:00Z")),
      origin_db_id: Set(Uuid::now_v7()),
    }
    .insert(&*db)
    .await
    .unwrap();

    audit_log::ActiveModel {
      id: Set(Uuid::now_v7()),
      table_name: Set("dispatch_documents".to_string()),
      record_id: Set(target_b_record),
      action: Set(enums::AuditAction::Insert),
      old_values: Set(None),
      new_values: Set(None),
      target_base_ids: Set(base_b.id.to_string()),
      user_role_weight: Set(10),
      user_id: Set(Uuid::now_v7()),
      timestamp: Set(ts("2026-01-01T00:02:00Z")),
      origin_db_id: Set(Uuid::now_v7()),
    }
    .insert(&*db)
    .await
    .unwrap();

    audit_log::ActiveModel {
      id: Set(Uuid::now_v7()),
      table_name: Set("storages".to_string()),
      record_id: Set(global_storage_record),
      action: Set(enums::AuditAction::Update),
      old_values: Set(None),
      new_values: Set(None),
      target_base_ids: Set(base_b.id.to_string()),
      user_role_weight: Set(10),
      user_id: Set(Uuid::now_v7()),
      timestamp: Set(ts("2026-01-01T00:03:00Z")),
      origin_db_id: Set(Uuid::now_v7()),
    }
    .insert(&*db)
    .await
    .unwrap();

    let response = service
      .pull_logs(INITIAL_AUDIT_CURSOR, &[base_a_id], None)
      .await
      .unwrap();
    let pulled_ids = response
      .logs
      .iter()
      .map(|log| log.record_id)
      .collect::<Vec<_>>();

    assert!(pulled_ids.contains(&global_log_record));
    assert!(pulled_ids.contains(&target_a_record));
    assert!(pulled_ids.contains(&global_storage_record));
    assert!(!pulled_ids.contains(&target_b_record));
    assert_eq!(
      response.highest_evaluated_id,
      response.logs.last().unwrap().id
    );
  })
  .await;
}

#[tokio::test]
async fn sync_pull_excludes_roles_and_local_tables_from_scope() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);
    let catalog = seed_inventory_catalog(&db).await;
    let _node_a_id = seed_sync_node(&db, catalog.base_id, "Node A").await;

    let allowed_record = Uuid::now_v7();
    let roles_record = Uuid::now_v7();
    let local_record = Uuid::now_v7();

    audit_log::ActiveModel {
      id: Set(Uuid::now_v7()),
      table_name: Set("companies".to_string()),
      record_id: Set(allowed_record),
      action: Set(enums::AuditAction::Insert),
      old_values: Set(None),
      new_values: Set(None),
      target_base_ids: Set(String::new()),
      user_role_weight: Set(10),
      user_id: Set(Uuid::now_v7()),
      timestamp: Set(ts("2026-01-02T00:00:00Z")),
      origin_db_id: Set(Uuid::now_v7()),
    }
    .insert(&*db)
    .await
    .unwrap();

    audit_log::ActiveModel {
      id: Set(Uuid::now_v7()),
      table_name: Set("roles".to_string()),
      record_id: Set(roles_record),
      action: Set(enums::AuditAction::Update),
      old_values: Set(None),
      new_values: Set(None),
      target_base_ids: Set(catalog.base_id.to_string()),
      user_role_weight: Set(10),
      user_id: Set(Uuid::now_v7()),
      timestamp: Set(ts("2026-01-02T00:01:00Z")),
      origin_db_id: Set(Uuid::now_v7()),
    }
    .insert(&*db)
    .await
    .unwrap();

    audit_log::ActiveModel {
      id: Set(Uuid::now_v7()),
      table_name: Set("local".to_string()),
      record_id: Set(local_record),
      action: Set(enums::AuditAction::Update),
      old_values: Set(None),
      new_values: Set(None),
      target_base_ids: Set(catalog.base_id.to_string()),
      user_role_weight: Set(10),
      user_id: Set(Uuid::now_v7()),
      timestamp: Set(ts("2026-01-02T00:02:00Z")),
      origin_db_id: Set(Uuid::now_v7()),
    }
    .insert(&*db)
    .await
    .unwrap();

    let response = service
      .pull_logs(INITIAL_AUDIT_CURSOR, &[catalog.base_id], None)
      .await
      .unwrap();

    let pulled_ids = response
      .logs
      .iter()
      .map(|log| log.record_id)
      .collect::<Vec<_>>();

    assert!(pulled_ids.contains(&allowed_record));
    assert!(!pulled_ids.contains(&roles_record));
    assert!(!pulled_ids.contains(&local_record));
  })
  .await;
}
