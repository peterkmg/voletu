use std::{str::FromStr, sync::Arc};

use assert_json_diff::assert_json_eq;
use sea_orm::{
  entity::prelude::Decimal,
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  QueryFilter,
};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  entities::{audit_log, company},
  enums::{self, AuditTable},
  services::{audit::AuditService, ledger::LedgerService},
};

use crate::common::{catalog_seed::seed_inventory_catalog, setup_db, test_config};

fn dec(value: &str) -> Decimal {
  Decimal::from_str(value).unwrap()
}

#[tokio::test]
async fn audit_service_insert_and_update_persist_expected_fields_and_payloads() {
  let actor_id = Uuid::now_v7();
  let context_origin_db_id = Uuid::now_v7();

  with_audit_context(actor_id, context_origin_db_id, || async {
    let db = Arc::new(setup_db().await);
    let origin_db_id = Uuid::now_v7();
    let mut cfg = test_config();
    cfg.node.db_id = origin_db_id;
    let service = AuditService::new(Arc::new(cfg));

    let saved = company::ActiveModel {
      common_name: Set("ACME".to_string()),
      legal_name: Set(Some("ACME Ltd".to_string())),
      is_contractor: Set(true),
      is_exporter: Set(false),
      is_manufacturer: Set(true),
      is_sender: Set(true),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    // INSERT audit log is created automatically by the entity's after_save hook.

    let updated = company::ActiveModel {
      id: Set(saved.id),
      common_name: Set("ACME 2".to_string()),
      ..Default::default()
    }
    .update(&*db)
    .await
    .unwrap();

    service
      .register_update(&*db, saved.id, &saved, &updated)
      .await
      .unwrap();

    let rows: Vec<audit_log::ModelEx> = audit_log::Entity::load()
      .filter(audit_log::Column::RecordId.eq(saved.id))
      .all(&*db)
      .await
      .unwrap();
    assert_eq!(rows.len(), 2);

    let insert_row = rows
      .iter()
      .find(|r| r.action == enums::AuditAction::Insert)
      .unwrap();
    assert_eq!(insert_row.table_name, AuditTable::Companies);
    assert_eq!(insert_row.origin_db_id, context_origin_db_id);
    assert_eq!(insert_row.old_values, None);
    let expected_insert = serde_json::to_value(&saved).unwrap();
    assert_json_eq!(insert_row.new_values.as_ref().unwrap(), &expected_insert);

    let update_row = rows
      .iter()
      .find(|r| r.action == enums::AuditAction::Update)
      .unwrap();
    assert_eq!(update_row.origin_db_id, context_origin_db_id);
    let expected_old = serde_json::to_value(&saved).unwrap();
    let expected_new = serde_json::to_value(&updated).unwrap();
    assert_json_eq!(update_row.old_values.as_ref().unwrap(), &expected_old);
    assert_json_eq!(update_row.new_values.as_ref().unwrap(), &expected_new);
  })
  .await;
}

#[tokio::test]
async fn audit_service_model_methods_capture_full_inserts_and_field_level_update_diffs() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let origin_db_id = Uuid::now_v7();
    let mut cfg = test_config();
    cfg.node.db_id = origin_db_id;
    let service = AuditService::new(Arc::new(cfg));

    let saved = company::ActiveModel {
      common_name: Set("ACME".to_string()),
      legal_name: Set(Some("ACME Ltd".to_string())),
      is_contractor: Set(true),
      is_exporter: Set(false),
      is_manufacturer: Set(true),
      is_sender: Set(true),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    // INSERT audit log is created automatically by the entity's after_save hook.

    let updated = company::ActiveModel {
      id: Set(saved.id),
      common_name: Set("ACME 2".to_string()),
      ..Default::default()
    }
    .update(&*db)
    .await
    .unwrap();

    service
      .register_update(&*db, saved.id, &saved, &updated)
      .await
      .unwrap();

    service
      .register_update(&*db, saved.id, &updated, &updated)
      .await
      .unwrap();

    let rows: Vec<audit_log::ModelEx> = audit_log::Entity::load()
      .filter(audit_log::Column::RecordId.eq(saved.id))
      .all(&*db)
      .await
      .unwrap();
    assert_eq!(rows.len(), 2);

    let insert_row = rows
      .iter()
      .find(|r| r.action == enums::AuditAction::Insert)
      .unwrap();
    let expected_insert = serde_json::to_value(&saved).unwrap();
    assert_json_eq!(insert_row.new_values.as_ref().unwrap(), &expected_insert);

    let changed_update_row = rows
      .iter()
      .find(|r| r.action == enums::AuditAction::Update && r.new_values.is_some())
      .unwrap();
    let expected_old = serde_json::to_value(&saved).unwrap();
    let expected_new = serde_json::to_value(&updated).unwrap();
    assert_json_eq!(
      changed_update_row.old_values.as_ref().unwrap(),
      &expected_old
    );
    assert_json_eq!(
      changed_update_row.new_values.as_ref().unwrap(),
      &expected_new
    );

    assert_eq!(
      rows
        .iter()
        .filter(|r| r.action == enums::AuditAction::Update)
        .count(),
      1
    );
  })
  .await;
}

#[tokio::test]
async fn ledger_service_apply_delta_creates_updates_and_allows_negative_balances() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let service = LedgerService::new(db.clone());

    // Positive delta creates a new entry.
    service
      .apply_delta(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
        Decimal::new(5, 0),
      )
      .await
      .unwrap();
    let after_create = service
      .by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(after_create.current_amount, dec("5.0"));

    // Delta update on existing entry.
    service
      .apply_delta(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
        Decimal::new(-2, 0),
      )
      .await
      .unwrap();
    let after_update = service
      .by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(after_update.current_amount, dec("3.0"));

    // Negative deltas are allowed (balance can go below zero).
    service
      .apply_delta(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
        Decimal::new(-4, 0),
      )
      .await
      .unwrap();
    let after_negative = service
      .by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(after_negative.current_amount, dec("-1.0"));

    // Creating a missing entry with a negative delta is also allowed.
    service
      .apply_delta(
        catalog.storage_b_id,
        catalog.product_b_id,
        catalog.contractor_b_id,
        Decimal::new(-1, 0),
      )
      .await
      .unwrap();
    let negative_new = service
      .by_dimensions(
        catalog.storage_b_id,
        catalog.product_b_id,
        catalog.contractor_b_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(negative_new.current_amount, dec("-1.0"));

    // Delta update can move an existing entry to an exact target.
    service
      .apply_delta(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
        Decimal::new(8, 0),
      )
      .await
      .unwrap();
    let after_target = service
      .by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(after_target.current_amount, dec("7.0"));
  })
  .await;
}
