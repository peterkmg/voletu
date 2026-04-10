use std::sync::Arc;

use chrono::{DateTime, Utc};
use sea_orm::{
  ActiveModelTrait,
  ActiveValue::Set,
  EntityLoaderTrait,
  EntityTrait,
  IntoActiveModel,
};
use uuid::Uuid;
use voletu_core::{
  api::ApiError,
  context::audit::with_audit_context,
  dtos::PushAuditLogRequest,
  entities::{audit_log, company, database_instance, local, node_base_assignment, sync_watermark},
  enums::{self, AuditAction, AuditTable, SyncDirection},
  services::sync::{specs::SyncStatusQuerySpec, SyncService},
};

use crate::common::{
  catalog_seed::{seed_inventory_catalog, seed_sync_node},
  setup_db,
  test_config,
};

const TEST_SYNC_NODE_ID: Uuid = Uuid::from_u128(11);

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
async fn sync_push_rejects_lower_role_update_when_newer_higher_role_log_exists() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);
    let record_id = Uuid::now_v7();

    audit_log::ActiveModel {
      id: Set(Uuid::now_v7()),
      table_name: Set(AuditTable::DispatchDocuments),
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
        table_name: AuditTable::DispatchDocuments,
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
        table_name: AuditTable::Companies,
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

    let all_logs: Vec<audit_log::ModelEx> = audit_log::Entity::load().all(&*db).await.unwrap();
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
    assert!(company::Entity::load()
      .filter_by_id(seeded_company.id)
      .one(&*db)
      .await
      .unwrap()
      .is_none());

    let restore_log_id = Uuid::now_v7();
    let payload = vec![PushAuditLogRequest {
      id: restore_log_id,
      table_name: AuditTable::Companies,
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

    let reconstructed = company::Entity::load()
      .filter_by_id(seeded_company.id)
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
    let restore_log = audit_log::Entity::load()
      .filter_by_id(restore_log_id)
      .one(&*db)
      .await
      .unwrap();
    assert!(restore_log.is_some(), "pushed restore log should exist");
  })
  .await;
}

#[tokio::test]
async fn sync_push_skips_duplicate_log_ids_within_same_batch() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);

    let company_id = Uuid::now_v7();
    let log_id = Uuid::now_v7();
    let payload = vec![
      PushAuditLogRequest {
        id: log_id,
        table_name: AuditTable::Companies,
        record_id: company_id,
        action: AuditAction::Insert,
        old_values_json: None,
        new_values_json: Some(format!(
          "{{\"id\":\"{company_id}\",\"common_name\":\"Batch Dup\",\"legal_name\":null,\
             \"is_contractor\":true,\"is_exporter\":false,\"is_manufacturer\":false,\
             \"is_sender\":false}}"
        )),
        target_base_ids: String::new(),
        user_role_weight: 10,
        user_id: Uuid::now_v7(),
        timestamp: ts("2026-01-05T00:00:00Z"),
        origin_db_id: Uuid::now_v7(),
      },
      PushAuditLogRequest {
        id: log_id,
        table_name: AuditTable::Companies,
        record_id: company_id,
        action: AuditAction::Insert,
        old_values_json: None,
        new_values_json: Some(format!(
          "{{\"id\":\"{company_id}\",\"common_name\":\"Batch Dup\",\"legal_name\":null,\
             \"is_contractor\":true,\"is_exporter\":false,\"is_manufacturer\":false,\
             \"is_sender\":false}}"
        )),
        target_base_ids: String::new(),
        user_role_weight: 10,
        user_id: Uuid::now_v7(),
        timestamp: ts("2026-01-05T00:00:00Z"),
        origin_db_id: Uuid::now_v7(),
      },
    ];

    let result = service.push_logs(&payload).await.unwrap();
    assert_eq!(result.accepted, 1);
    assert_eq!(result.rejected, 0);

    let persisted = audit_log::Entity::load()
      .filter_by_id(log_id)
      .all(&*db)
      .await
      .unwrap();
    assert_eq!(persisted.len(), 1);
  })
  .await;
}

#[tokio::test]
async fn sync_watermark_upsert_updates_existing_row_for_same_node_and_direction() {
  let db = Arc::new(setup_db().await);
  let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);
  let catalog = seed_inventory_catalog(&db).await;
  let node_id = seed_sync_node(&db, catalog.base_id, "Node WM").await;

  let first_log_id = Uuid::now_v7();
  let second_log_id = Uuid::now_v7();
  service
    .upsert_watermark(node_id, SyncDirection::Push, first_log_id, String::new())
    .await
    .unwrap();
  service
    .upsert_watermark(node_id, SyncDirection::Push, second_log_id, String::new())
    .await
    .unwrap();

  let rows: Vec<sync_watermark::ModelEx> = sync_watermark::Entity::load().all(&*db).await.unwrap();
  assert_eq!(rows.len(), 1);
  assert_eq!(rows[0].last_audit_log_id, second_log_id);
}

#[tokio::test]
async fn sync_status_reflects_local_topology_and_local_row_keeps_central_url() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;

    let instance_row = database_instance::ActiveModel {
      common_name: Set("Local Node".to_string()),
      node_type: Set(enums::NodeType::Central),
      base_id: Set(None),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let local_row = local::ActiveModel {
      id: Set(1),
      is_initialized: Set(false),
      local_db_id: Set(instance_row.id),
      jwt_secret: Set("test-secret".to_string()),
      central_api_url: Set(None),
    }
    .insert(&*db)
    .await
    .unwrap();

    let mut local_am = local_row.clone().into_active_model();
    local_am.central_api_url = Set(Some("http://central.local:3030".to_string()));
    local_am.update(&*db).await.unwrap();

    let mut instance_am = instance_row.into_active_model();
    instance_am.base_id = Set(Some(catalog.base_id));
    instance_am.node_type = Set(enums::NodeType::Peripheral);
    instance_am.update(&*db).await.unwrap();

    let service = sync_service_with_node(db.clone(), local_row.local_db_id);

    let status = service
      .sync_status(SyncStatusQuerySpec::default())
      .await
      .unwrap();
    assert_eq!(status.node_type, "PERIPHERAL");

    let updated_local_row = local::Entity::load()
      .filter_by_id(1)
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
async fn apply_pulled_logs_advances_watermark_within_same_transaction() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let central_id = Uuid::now_v7();
    let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);

    // Apply an empty batch under the empty discriminant (catalog-only scope
    // with no assignments). The method should succeed and write a watermark
    // row with the given last_audit_log_id and base_discriminant="".
    let target_last_id = Uuid::now_v7();
    service
      .apply_pulled_logs(&[], central_id, target_last_id, String::new())
      .await
      .expect("apply_pulled_logs should succeed with empty batch");

    let (last_id, discriminant) = service
      .load_pull_watermark(central_id, SyncDirection::Pull)
      .await
      .expect("watermark load");
    assert_eq!(last_id, target_last_id);
    assert_eq!(discriminant, "");
  })
  .await;
}

#[tokio::test]
async fn apply_pulled_logs_aborts_when_discriminant_drifted() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    // Real database_instance row so the node_base_assignment FK is satisfied.
    let local_node_id = seed_sync_node(&db, catalog.base_id, "PA drift test").await;
    let central_id = Uuid::now_v7();
    let service = sync_service_with_node(db.clone(), local_node_id);

    // Precondition: the local node has at least one base assignment, so its
    // current discriminant is NOT empty.
    node_base_assignment::ActiveModel {
      node_id: Set(local_node_id),
      base_id: Set(catalog.base_id),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    // Caller passes expected_discriminant="" (as if the pull was issued before
    // the assignment was added). apply_pulled_logs must abort with Conflict.
    let err = service
      .apply_pulled_logs(&[], central_id, Uuid::now_v7(), String::new())
      .await
      .expect_err("apply_pulled_logs should abort on discriminant drift");

    match err {
      ApiError::Conflict(msg) => {
        assert!(msg.contains("discriminant"), "unexpected message: {msg}");
      }
      other => panic!("expected Conflict, got {other:?}"),
    }

    // Watermark must NOT be written — the transaction rolled back.
    let (last_id, disc) = service
      .load_pull_watermark(central_id, SyncDirection::Pull)
      .await
      .unwrap();
    assert_eq!(last_id, Uuid::nil());
    assert_eq!(disc, "");
  })
  .await;
}

#[tokio::test]
async fn apply_pulled_logs_rejects_lower_role_update_after_earlier_same_batch_apply() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let central_id = Uuid::now_v7();
    let service = sync_service_with_node(db.clone(), TEST_SYNC_NODE_ID);

    let seeded_company = company::ActiveModel {
      common_name: Set("Batch Apply Original".to_string()),
      legal_name: Set(None),
      is_contractor: Set(true),
      is_exporter: Set(false),
      is_manufacturer: Set(false),
      is_sender: Set(false),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let mut first_update = seeded_company.clone();
    first_update.common_name = "Batch Apply High".to_string();

    let mut second_update = seeded_company.clone();
    second_update.common_name = "Batch Apply Low".to_string();

    let payload = vec![
      PushAuditLogRequest {
        id: Uuid::now_v7(),
        table_name: AuditTable::Companies,
        record_id: seeded_company.id,
        action: AuditAction::Update,
        old_values_json: Some(serde_json::to_string(&seeded_company).unwrap()),
        new_values_json: Some(serde_json::to_string(&first_update).unwrap()),
        target_base_ids: String::new(),
        user_role_weight: 100,
        user_id: Uuid::now_v7(),
        timestamp: ts("2030-01-06T00:00:00Z"),
        origin_db_id: Uuid::now_v7(),
      },
      PushAuditLogRequest {
        id: Uuid::now_v7(),
        table_name: AuditTable::Companies,
        record_id: seeded_company.id,
        action: AuditAction::Update,
        old_values_json: Some(serde_json::to_string(&first_update).unwrap()),
        new_values_json: Some(serde_json::to_string(&second_update).unwrap()),
        target_base_ids: String::new(),
        user_role_weight: 10,
        user_id: Uuid::now_v7(),
        timestamp: ts("2030-01-05T00:00:00Z"),
        origin_db_id: Uuid::now_v7(),
      },
    ];

    let result = service
      .apply_pulled_logs(&payload, central_id, Uuid::now_v7(), String::new())
      .await
      .unwrap();
    assert_eq!(result.accepted, 1);
    assert_eq!(result.rejected, 1);

    let reconstructed = company::Entity::load()
      .filter_by_id(seeded_company.id)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(reconstructed.common_name, "Batch Apply High");
  })
  .await;
}
