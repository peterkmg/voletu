use std::{str::FromStr, sync::Arc};

use assert_json_diff::assert_json_eq;
use sea_orm::{
  entity::prelude::Decimal,
  ActiveModelTrait,
  ActiveValue::Set,
  ColumnTrait,
  QueryFilter,
  SqlErr,
};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  entities::{audit_log, company, inventory_ledger_entry},
  enums::{self, AuditTable},
  services::{
    audit::AuditService,
    ledger::{LedgerDelta, LedgerService},
  },
};

use crate::common::{catalog_seed::seed_inventory_catalog, setup_db, test_config};

fn dec(value: &str) -> Decimal {
  Decimal::from_str(value).unwrap()
}

async fn append_manual_adjustment(
  service: &LedgerService,
  db: &sea_orm::DatabaseConnection,
  storage_id: Uuid,
  product_id: Uuid,
  contractor_id: Uuid,
  quantity_delta: Decimal,
) {
  service
    .append_delta_on(db, LedgerDelta {
      storage_id,
      product_id,
      contractor_id,
      quantity_delta,
      source_kind: enums::LedgerEntrySourceKind::ManualAdjustment,
      source_id: Uuid::now_v7(),
      source_event: enums::LedgerEntrySourceEvent::Execution,
      reverses_entry_id: None,
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn insert_and_update_persist_expected_fields_in_audit_log() {
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
async fn model_methods_capture_full_inserts_and_field_level_update_diffs() {
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
async fn ledger_deltas_append_rows_and_balance_is_derived() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let service = LedgerService::new(db.clone());

    append_manual_adjustment(
      &service,
      &db,
      catalog.storage_a_id,
      catalog.product_a_id,
      catalog.contractor_a_id,
      Decimal::new(5, 0),
    )
    .await;
    let after_create = service
      .balance_by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(after_create.current_amount, dec("5.0"));

    append_manual_adjustment(
      &service,
      &db,
      catalog.storage_a_id,
      catalog.product_a_id,
      catalog.contractor_a_id,
      Decimal::new(-2, 0),
    )
    .await;
    append_manual_adjustment(
      &service,
      &db,
      catalog.storage_a_id,
      catalog.product_a_id,
      catalog.contractor_a_id,
      Decimal::new(-4, 0),
    )
    .await;

    let rows: Vec<inventory_ledger_entry::ModelEx> = inventory_ledger_entry::Entity::load()
      .filter(inventory_ledger_entry::Column::StorageId.eq(catalog.storage_a_id))
      .filter(inventory_ledger_entry::Column::ProductId.eq(catalog.product_a_id))
      .filter(inventory_ledger_entry::Column::ContractorId.eq(catalog.contractor_a_id))
      .all(&*db)
      .await
      .unwrap();
    assert_eq!(rows.len(), 3);
    let amounts: Vec<_> = rows.iter().map(|row| row.quantity_delta).collect();
    assert!(amounts.contains(&dec("5.0")));
    assert!(amounts.contains(&dec("-2.0")));
    assert!(amounts.contains(&dec("-4.0")));

    let balance = service
      .balance_by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(balance.current_amount, dec("-1.0"));
  })
  .await;
}

#[tokio::test]
async fn reversal_append_is_idempotent_for_already_reversed_rows() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let service = LedgerService::new(db.clone());
    let source_id = Uuid::now_v7();

    service
      .append_delta_on(&*db, voletu_core::services::ledger::LedgerDelta {
        storage_id: catalog.storage_a_id,
        product_id: catalog.product_a_id,
        contractor_id: catalog.contractor_a_id,
        quantity_delta: Decimal::new(5, 0),
        source_kind: enums::LedgerEntrySourceKind::ManualAdjustment,
        source_id,
        source_event: enums::LedgerEntrySourceEvent::Execution,
        reverses_entry_id: None,
      })
      .await
      .unwrap();

    service
      .append_reversal_deltas_on(
        &*db,
        enums::LedgerEntrySourceKind::ManualAdjustment,
        source_id,
      )
      .await
      .unwrap();

    service
      .append_reversal_deltas_on(
        &*db,
        enums::LedgerEntrySourceKind::ManualAdjustment,
        source_id,
      )
      .await
      .unwrap();

    let rows: Vec<inventory_ledger_entry::ModelEx> = inventory_ledger_entry::Entity::load()
      .filter(
        inventory_ledger_entry::Column::SourceKind
          .eq(enums::LedgerEntrySourceKind::ManualAdjustment),
      )
      .filter(inventory_ledger_entry::Column::SourceId.eq(source_id))
      .all(&*db)
      .await
      .unwrap();

    assert_eq!(rows.len(), 2);
    assert_eq!(
      rows
        .iter()
        .filter(|row| row.source_event == enums::LedgerEntrySourceEvent::Reversion)
        .count(),
      1
    );
  })
  .await;
}

#[tokio::test]
async fn reversal_entries_are_unique_per_original_ledger_entry() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let service = LedgerService::new(db.clone());

    let original = service
      .append_delta_on(&*db, voletu_core::services::ledger::LedgerDelta {
        storage_id: catalog.storage_a_id,
        product_id: catalog.product_a_id,
        contractor_id: catalog.contractor_a_id,
        quantity_delta: Decimal::new(5, 0),
        source_kind: enums::LedgerEntrySourceKind::ManualAdjustment,
        source_id: Uuid::now_v7(),
        source_event: enums::LedgerEntrySourceEvent::Execution,
        reverses_entry_id: None,
      })
      .await
      .unwrap();

    inventory_ledger_entry::ActiveModel {
      storage_id: Set(original.storage_id),
      product_id: Set(original.product_id),
      contractor_id: Set(original.contractor_id),
      quantity_delta: Set(-original.quantity_delta),
      source_kind: Set(original.source_kind),
      source_id: Set(original.source_id),
      source_event: Set(enums::LedgerEntrySourceEvent::Reversion),
      reverses_entry_id: Set(Some(original.id)),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let duplicate = inventory_ledger_entry::ActiveModel {
      storage_id: Set(original.storage_id),
      product_id: Set(original.product_id),
      contractor_id: Set(original.contractor_id),
      quantity_delta: Set(-original.quantity_delta),
      source_kind: Set(original.source_kind),
      source_id: Set(original.source_id),
      source_event: Set(enums::LedgerEntrySourceEvent::Reversion),
      reverses_entry_id: Set(Some(original.id)),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap_err();

    assert!(matches!(
      duplicate.sql_err(),
      Some(SqlErr::UniqueConstraintViolation(_))
    ));
  })
  .await;
}

#[tokio::test]
async fn manual_adjustment_deltas_allow_missing_negative_balances_and_exact_targets() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let service = LedgerService::new(db.clone());

    append_manual_adjustment(
      &service,
      &db,
      catalog.storage_b_id,
      catalog.product_b_id,
      catalog.contractor_b_id,
      Decimal::new(-1, 0),
    )
    .await;
    let negative_new = service
      .balance_by_dimensions(
        catalog.storage_b_id,
        catalog.product_b_id,
        catalog.contractor_b_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(negative_new.current_amount, dec("-1.0"));

    append_manual_adjustment(
      &service,
      &db,
      catalog.storage_a_id,
      catalog.product_a_id,
      catalog.contractor_a_id,
      Decimal::new(-1, 0),
    )
    .await;
    append_manual_adjustment(
      &service,
      &db,
      catalog.storage_a_id,
      catalog.product_a_id,
      catalog.contractor_a_id,
      Decimal::new(8, 0),
    )
    .await;
    let after_target = service
      .balance_by_dimensions(
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
