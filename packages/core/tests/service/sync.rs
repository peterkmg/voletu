use std::sync::Arc;

use assert_json_diff::assert_json_eq;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  dtos::PushAuditLogRequest,
  entities::{audit_log, base, company, database_instance, local, sync_watermark},
  enums::{self, AuditAction, SyncDirection},
  services::sync::SyncService,
};

use crate::common::{
  fixtures::{seed_inventory_fixture, seed_sync_node},
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

    let node_target = seed_sync_node(&db, base_target, "Node Target").await;

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
async fn sync_status_reflects_local_topology_and_local_row_keeps_central_url() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let fixture = seed_inventory_fixture(&db).await;

    let local_db_id = database_instance::ActiveModel {
      common_name: Set("Local Node".to_string()),
      node_type: Set(enums::NodeType::Central),
      base_id: Set(None),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap()
    .id;

    local::ActiveModel {
      id: Set(1),
      is_initialized: Set(false),
      local_db_id: Set(local_db_id),
      jwt_secret: Set("test-secret".to_string()),
      central_api_url: Set(None),
    }
    .insert(&*db)
    .await
    .unwrap();

    let local_row = local::Entity::find_by_id(1)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    let mut local_am = local_row.clone().into_active_model();
    local_am.central_api_url = Set(Some("http://central.local:3030".to_string()));
    local_am.update(&*db).await.unwrap();

    let instance_row = database_instance::Entity::find_by_id(local_row.local_db_id)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    let mut instance_am = instance_row.into_active_model();
    instance_am.base_id = Set(Some(fixture.base_id));
    instance_am.node_type = Set(enums::NodeType::Peripheral);
    instance_am.update(&*db).await.unwrap();

    let service = sync_service_with_node(db.clone(), local_db_id);

    let status = service.sync_status().await.unwrap();
    assert_eq!(status.node_type, "PERIPHERAL");

    let updated_local_row = local::Entity::find_by_id(1)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(
      updated_local_row.central_api_url.as_deref(),
      Some("http://central.local:3030")
    );
  })
  .await;
}

#[tokio::test]
async fn sync_watermark_upsert_updates_existing_row_for_same_node_and_direction() {
  let db = Arc::new(setup_db().await);
  let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);
  let fixture = seed_inventory_fixture(&db).await;
  let node_id = seed_sync_node(&db, fixture.base_id, "Node WM").await;

  let first_log_id = Uuid::now_v7();
  let second_log_id = Uuid::now_v7();
  service
    .upsert_watermark(node_id, SyncDirection::Push, first_log_id)
    .await
    .unwrap();
  service
    .upsert_watermark(node_id, SyncDirection::Push, second_log_id)
    .await
    .unwrap();

  let rows = sync_watermark::Entity::find().all(&*db).await.unwrap();
  assert_eq!(rows.len(), 1);
  assert_eq!(rows[0].last_audit_log_id, second_log_id);
}

#[tokio::test]
async fn sync_push_rejects_lower_role_update_when_newer_higher_role_log_exists() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);
    let record_id = Uuid::now_v7();

    audit_log::ActiveModel {
      id: Set(Uuid::now_v7()),
      table_name: Set("dispatch_documents".to_string()),
      record_id: Set(record_id),
      action: Set(enums::AuditAction::Update),
      old_values: Set(None),
      new_values: Set(None),
      target_base_ids: Set(String::new()),
      user_role_weight: Set(100),
      user_id: Set(Uuid::now_v7()),
      timestamp: Set(ts("2026-01-02T00:00:00Z")),
      origin_db_id: Set(Uuid::now_v7()),
    }
    .insert(&*db)
    .await
    .unwrap();

    let payload = vec![
      PushAuditLogRequest {
        id: Uuid::now_v7(),
        table_name: "dispatch_documents".to_string(),
        record_id,
        action: AuditAction::Update,
        old_values_json: None,
        new_values_json: Some("{\"status\":\"old\"}".to_string()),
        target_base_ids: String::new(),
        user_role_weight: 10,
        user_id: Uuid::now_v7(),
        timestamp: ts("2026-01-01T00:00:00Z"),
        origin_db_id: Uuid::now_v7(),
      },
      PushAuditLogRequest {
        id: Uuid::now_v7(),
        table_name: "companies".to_string(),
        record_id: Uuid::now_v7(),
        action: AuditAction::Insert,
        old_values_json: None,
        new_values_json: Some("{\"name\":\"ACME\"}".to_string()),
        target_base_ids: String::new(),
        user_role_weight: 10,
        user_id: Uuid::now_v7(),
        timestamp: ts("2026-01-03T00:00:00Z"),
        origin_db_id: Uuid::now_v7(),
      },
    ];

    let result = service.push_logs(&payload).await.unwrap();
    assert_eq!(result.accepted, 1);
    assert_eq!(result.rejected, 1);

    let all_logs = audit_log::Entity::find().all(&*db).await.unwrap();
    assert_eq!(all_logs.len(), 2);
  })
  .await;
}

#[tokio::test]
async fn sync_push_restores_company_from_snapshot_and_is_idempotent_on_reapply() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);

    let seeded_company = company::ActiveModel {
      common_name: Set("Restore Company".to_string()),
      legal_name: Set(Some("Restore Company LLC".to_string())),
      is_contractor: Set(true),
      is_exporter: Set(false),
      is_manufacturer: Set(false),
      is_sender: Set(true),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let snapshot_json = serde_json::to_string(&seeded_company).unwrap();
    company::Entity::delete_by_id(seeded_company.id)
      .exec(&*db)
      .await
      .unwrap();
    assert!(company::Entity::find_by_id(seeded_company.id)
      .one(&*db)
      .await
      .unwrap()
      .is_none());

    let restore_log_id = Uuid::now_v7();
    let payload = vec![PushAuditLogRequest {
      id: restore_log_id,
      table_name: "companies".to_string(),
      record_id: seeded_company.id,
      action: AuditAction::Insert,
      old_values_json: None,
      new_values_json: Some(snapshot_json.clone()),
      target_base_ids: String::new(),
      user_role_weight: 10,
      user_id: Uuid::now_v7(),
      timestamp: ts("2026-01-04T00:00:00Z"),
      origin_db_id: Uuid::now_v7(),
    }];

    let first_apply = service.push_logs(&payload).await.unwrap();
    assert_eq!(first_apply.accepted, 1);
    assert_eq!(first_apply.rejected, 0);

    let reconstructed = company::Entity::find_by_id(seeded_company.id)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(reconstructed.common_name, seeded_company.common_name);
    assert_eq!(reconstructed.legal_name, seeded_company.legal_name);

    let second_apply = service.push_logs(&payload).await.unwrap();
    assert_eq!(second_apply.accepted, 0);
    assert_eq!(second_apply.rejected, 0);

    // Verify the pushed restore log exists (other audit logs may be present from entity hooks)
    let restore_log = audit_log::Entity::find_by_id(restore_log_id)
      .one(&*db)
      .await
      .unwrap();
    assert!(restore_log.is_some(), "pushed restore log should exist");
  })
  .await;
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

    let node_a = seed_sync_node(&db, base_a.id, "Node A").await;

    let log = audit_log::ActiveModel {
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
    let fixture = seed_inventory_fixture(&db).await;
    let base_a_id = fixture.base_id;
    let base_b = base::ActiveModel {
      common_name: Set("Base B".to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let node_a_id = seed_sync_node(&db, base_a_id, "Node A").await;

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
    let fixture = seed_inventory_fixture(&db).await;
    let node_a_id = seed_sync_node(&db, fixture.base_id, "Node A").await;

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
      target_base_ids: Set(fixture.base_id.to_string()),
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
      target_base_ids: Set(fixture.base_id.to_string()),
      user_role_weight: Set(10),
      user_id: Set(Uuid::now_v7()),
      timestamp: Set(ts("2026-01-02T00:02:00Z")),
      origin_db_id: Set(Uuid::now_v7()),
    }
    .insert(&*db)
    .await
    .unwrap();

    let response = service
      .pull_logs(INITIAL_AUDIT_CURSOR, &[fixture.base_id], None)
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
