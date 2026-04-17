use std::{str::FromStr, sync::Arc};

use assert_json_diff::assert_json_eq;
use chrono::{DateTime, Utc};
use sea_orm::{prelude::Decimal, ActiveModelTrait, ActiveValue::Set, EntityTrait};
use uuid::Uuid;
use voletu_core::{
  api::ApiError,
  context::audit::with_audit_context,
  dtos::{
    AcceptanceItemCompositeRequest,
    CreateAcceptanceCompositeRequest,
    CreateAcceptanceItemRequest,
    CreateAcceptanceRequest,
    CreateDispatchCompositeRequest,
    CreateDispatchItemRequest,
    CreateDispatchRequest,
    DispatchItemCompositeRequest,
    DispatchMeasurementCompositeRequest,
    UpdateAcceptanceCompositeRequest,
    UpdateAcceptanceItemCompositeRequest,
    UpdateAcceptanceRequest,
    UpdateDispatchCompositeRequest,
    UpdateDispatchItemCompositeRequest,
    UpdateDispatchMeasurementCompositeRequest,
    UpdateDispatchRequest,
  },
  entities::{acceptance_document, audit_log, product_type, storage},
  enums::{self, ArrivalType, AuditTable, DispatchMethod, DispatchPurpose},
  services::{audit::AuditService, document::DocumentService, ledger::LedgerService},
};

use crate::common::{
  catalog_seed::{seed_inventory_catalog, seed_ledger_balance},
  setup_db,
  test_config,
};

fn ts(value: &str) -> DateTime<Utc> {
  DateTime::parse_from_rfc3339(value)
    .unwrap()
    .with_timezone(&Utc)
}

fn dec(value: &str) -> Decimal {
  Decimal::from_str(value).unwrap()
}

#[tokio::test]
async fn acceptance_execution_applies_items_to_ledger_and_emits_audit_rows() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger.clone(), audit);

    let doc = service
      .acceptance_document_create(&CreateAcceptanceRequest {
        document_number: "ACC-1".to_string(),
        date_accepted: ts("2026-01-01T00:00:00Z"),
        arrival_type: ArrivalType::Truck,
        source_entity: None,
        contractor_id: catalog.contractor_a_id,
        truck_waybill_id: None,
        rail_waybill_id: None,
        transit_dispatch_id: None,
      })
      .await
      .unwrap();

    let item = service
      .acceptance_item_create(&CreateAcceptanceItemRequest {
        acceptance_doc_id: doc.id,
        item: AcceptanceItemCompositeRequest {
          product_id: catalog.product_a_id,
          storage_id: catalog.storage_a_id,
          accepted_amount: dec("42.5"),
        },
      })
      .await
      .unwrap();

    assert_eq!(item.storage_id, catalog.storage_a_id);

    let created_doc_model = acceptance_document::Entity::find_by_id(doc.id)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();

    service
      .acceptance_document_execute(doc.id, Uuid::now_v7())
      .await
      .unwrap();

    let entry = ledger
      .by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(entry.current_amount, dec("42.5"));

    let audit_rows: Vec<audit_log::ModelEx> = audit_log::Entity::load().all(&*db).await.unwrap();
    assert!(!audit_rows.is_empty());

    let insert_log = audit_rows
      .iter()
      .find(|row| {
        row.table_name == AuditTable::AcceptanceDocuments
          && row.record_id == doc.id
          && row.action == enums::AuditAction::Insert
      })
      .unwrap();
    let expected_insert = serde_json::to_value(&created_doc_model).unwrap();
    assert_json_eq!(insert_log.new_values.as_ref().unwrap(), &expected_insert);

    let update_log = audit_rows
      .iter()
      .find(|row| {
        row.table_name == AuditTable::AcceptanceDocuments
          && row.record_id == doc.id
          && row.action == enums::AuditAction::Update
      })
      .unwrap();
    let updated_doc = acceptance_document::Entity::find_by_id(doc.id)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    let expected_old = serde_json::to_value(&created_doc_model).unwrap();
    let expected_new = serde_json::to_value(&updated_doc).unwrap();
    assert_json_eq!(update_log.old_values.as_ref().unwrap(), &expected_old);
    assert_json_eq!(update_log.new_values.as_ref().unwrap(), &expected_new);
  })
  .await;
}

#[tokio::test]
async fn acceptance_item_creation_rejects_storage_with_incompatible_product_type() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let other_product_type = product_type::ActiveModel {
      common_name: Set("Lubes".to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let mismatched_storage = storage::ActiveModel {
      warehouse_id: Set(catalog.warehouse_id),
      common_name: Set("Restricted Tank".to_string()),
      long_name: Set(None),
      capacity: Set(None),
      is_type_specific: Set(true),
      product_type_id: Set(Some(other_product_type.id)),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger.clone(), audit);

    let doc = service
      .acceptance_document_create(&CreateAcceptanceRequest {
        document_number: "ACC-2".to_string(),
        date_accepted: ts("2026-01-01T00:00:00Z"),
        arrival_type: ArrivalType::Truck,
        source_entity: None,
        contractor_id: catalog.contractor_a_id,
        truck_waybill_id: None,
        rail_waybill_id: None,
        transit_dispatch_id: None,
      })
      .await
      .unwrap();
    let err = service
      .acceptance_item_create(&CreateAcceptanceItemRequest {
        acceptance_doc_id: doc.id,
        item: AcceptanceItemCompositeRequest {
          product_id: catalog.product_a_id,
          storage_id: mismatched_storage.id,
          accepted_amount: dec("10.0"),
        },
      })
      .await
      .unwrap_err();

    assert!(err
      .to_string()
      .contains("Storage type restriction violated"));
  })
  .await;
}

#[tokio::test]
async fn acceptance_execution_applies_multiple_items_to_corresponding_storages() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger.clone(), audit);

    let doc = service
      .acceptance_document_create(&CreateAcceptanceRequest {
        document_number: "ACC-UNBALANCED".to_string(),
        date_accepted: ts("2026-01-01T00:00:00Z"),
        arrival_type: ArrivalType::Truck,
        source_entity: None,
        contractor_id: catalog.contractor_a_id,
        truck_waybill_id: None,
        rail_waybill_id: None,
        transit_dispatch_id: None,
      })
      .await
      .unwrap();
    service
      .acceptance_item_create(&CreateAcceptanceItemRequest {
        acceptance_doc_id: doc.id,
        item: AcceptanceItemCompositeRequest {
          product_id: catalog.product_a_id,
          storage_id: catalog.storage_a_id,
          accepted_amount: dec("10.0"),
        },
      })
      .await
      .unwrap();

    service
      .acceptance_item_create(&CreateAcceptanceItemRequest {
        acceptance_doc_id: doc.id,
        item: AcceptanceItemCompositeRequest {
          product_id: catalog.product_a_id,
          storage_id: catalog.storage_b_id,
          accepted_amount: dec("6.0"),
        },
      })
      .await
      .unwrap();

    service
      .acceptance_document_execute(doc.id, Uuid::now_v7())
      .await
      .unwrap();

    let storage_a = ledger
      .by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let storage_b = ledger
      .by_dimensions(
        catalog.storage_b_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(storage_a.current_amount, dec("10.0"));
    assert_eq!(storage_b.current_amount, dec("6.0"));
  })
  .await;
}

#[tokio::test]
async fn acceptance_item_creation_rejects_posted_document_mutation() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    let doc = service
      .acceptance_document_create(&CreateAcceptanceRequest {
        document_number: "ACC-POSTED-MUT".to_string(),
        date_accepted: ts("2026-01-07T00:00:00Z"),
        arrival_type: ArrivalType::Truck,
        source_entity: None,
        contractor_id: catalog.contractor_a_id,
        truck_waybill_id: None,
        rail_waybill_id: None,
        transit_dispatch_id: None,
      })
      .await
      .unwrap();

    service
      .acceptance_item_create(&CreateAcceptanceItemRequest {
        acceptance_doc_id: doc.id,
        item: AcceptanceItemCompositeRequest {
          product_id: catalog.product_a_id,
          storage_id: catalog.storage_a_id,
          accepted_amount: dec("10.0"),
        },
      })
      .await
      .unwrap();

    service
      .acceptance_document_execute(doc.id, Uuid::now_v7())
      .await
      .unwrap();

    let err = service
      .acceptance_item_create(&CreateAcceptanceItemRequest {
        acceptance_doc_id: doc.id,
        item: AcceptanceItemCompositeRequest {
          product_id: catalog.product_a_id,
          storage_id: catalog.storage_a_id,
          accepted_amount: dec("1.0"),
        },
      })
      .await
      .unwrap_err();

    assert!(err
      .to_string()
      .contains("Only draft documents can be modified"));
  })
  .await;
}

#[tokio::test]
async fn dispatch_item_creation_rejects_posted_document_mutation() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let dispatch = DocumentService::new(db.clone(), ledger.clone(), audit);

    seed_ledger_balance(
      &db,
      catalog.storage_a_id,
      catalog.product_a_id,
      catalog.contractor_a_id,
      Decimal::from(5),
    )
    .await;

    let doc = dispatch
      .dispatch_document_create(&CreateDispatchRequest {
        document_number: "DISP-POSTED-MUT".to_string(),
        date: ts("2026-01-07T00:00:00Z"),
        dispatch_purpose: DispatchPurpose::External,
        dispatch_method: DispatchMethod::Truck,
        contractor_id: catalog.contractor_a_id,
        destination_base_id: None,
        receiver_entity: None,
        start_cargo_ops: None,
        end_cargo_ops: None,
        bunker_type: None,
        exporter_id: None,
        port_id: None,
      })
      .await
      .unwrap();

    dispatch
      .dispatch_item_create(&CreateDispatchItemRequest {
        dispatch_doc_id: doc.id,
        item: DispatchItemCompositeRequest {
          product_id: catalog.product_a_id,
          storage_id: catalog.storage_a_id,
          dispatched_amount: dec("1.0"),
        },
      })
      .await
      .unwrap();

    dispatch
      .dispatch_document_execute(doc.id, Uuid::now_v7())
      .await
      .unwrap();

    let err = dispatch
      .dispatch_item_create(&CreateDispatchItemRequest {
        dispatch_doc_id: doc.id,
        item: DispatchItemCompositeRequest {
          product_id: catalog.product_a_id,
          storage_id: catalog.storage_a_id,
          dispatched_amount: dec("1.0"),
        },
      })
      .await
      .unwrap_err();

    assert!(err
      .to_string()
      .contains("Only draft documents can be modified"));
  })
  .await;
}

#[tokio::test]
async fn update_acceptance_composite_inserts_updates_and_deletes_items() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // 1. Seed: create an acceptance composite with three items.
    let initial = service
      .acceptance_composite_create(&CreateAcceptanceCompositeRequest {
        acceptance: CreateAcceptanceRequest {
          document_number: "ACC-COMP-UPDATE-1".to_string(),
          date_accepted: ts("2026-01-01T00:00:00Z"),
          arrival_type: ArrivalType::Truck,
          source_entity: None,
          contractor_id: catalog.contractor_a_id,
          truck_waybill_id: None,
          rail_waybill_id: None,
          transit_dispatch_id: None,
        },
        items: vec![
          AcceptanceItemCompositeRequest {
            product_id: catalog.product_a_id,
            storage_id: catalog.storage_a_id,
            accepted_amount: dec("1.0"),
          },
          AcceptanceItemCompositeRequest {
            product_id: catalog.product_a_id,
            storage_id: catalog.storage_a_id,
            accepted_amount: dec("2.0"),
          },
          AcceptanceItemCompositeRequest {
            product_id: catalog.product_a_id,
            storage_id: catalog.storage_b_id,
            accepted_amount: dec("3.0"),
          },
        ],
      })
      .await
      .unwrap();

    assert_eq!(initial.items.len(), 3);

    let acceptance_id = initial.document.id;
    // We capture each id by its initial amount so the test does not depend on
    // a specific row ordering of the response. AcceptanceItemResponse is not Clone,
    // so we extract owned copies of just the fields we need below.
    let pick = |amount: Decimal| -> (Uuid, Uuid, Uuid, Decimal) {
      let item = initial
        .items
        .iter()
        .find(|item| item.accepted_amount == amount)
        .unwrap();
      (
        item.id,
        item.product_id,
        item.storage_id,
        item.accepted_amount,
      )
    };
    let (unchanged_id, unchanged_product, unchanged_storage, unchanged_amount) = pick(dec("1.0"));
    let (update_id, update_product, update_storage, _) = pick(dec("2.0"));
    let (delete_id, _, _, _) = pick(dec("3.0"));

    // 2. Apply a composite update:
    //    - keep item_unchanged as-is,
    //    - update item_to_update.accepted_amount,
    //    - drop item_to_delete by omitting it,
    //    - insert one fresh item with id: None.
    let updated = service
      .acceptance_composite_update(acceptance_id, &UpdateAcceptanceCompositeRequest {
        acceptance: UpdateAcceptanceRequest {
          document_number: None,
          date_accepted: None,
          arrival_type: None,
          source_entity: None,
          contractor_id: None,
          truck_waybill_id: None,
          rail_waybill_id: None,
          transit_dispatch_id: None,
        },
        items: vec![
          UpdateAcceptanceItemCompositeRequest {
            id: Some(unchanged_id),
            product_id: unchanged_product,
            storage_id: unchanged_storage,
            accepted_amount: unchanged_amount,
          },
          UpdateAcceptanceItemCompositeRequest {
            id: Some(update_id),
            product_id: update_product,
            storage_id: update_storage,
            accepted_amount: dec("9.5"),
          },
          UpdateAcceptanceItemCompositeRequest {
            id: None,
            product_id: catalog.product_a_id,
            storage_id: catalog.storage_b_id,
            accepted_amount: dec("4.25"),
          },
        ],
      })
      .await
      .unwrap();

    // 3. Assertions on the response.
    assert_eq!(updated.items.len(), 3);

    let unchanged = updated
      .items
      .iter()
      .find(|item| item.id == unchanged_id)
      .expect("the unchanged item should still be present");
    assert_eq!(unchanged.accepted_amount, dec("1.0"));
    assert_eq!(unchanged.storage_id, unchanged_storage);

    let modified = updated
      .items
      .iter()
      .find(|item| item.id == update_id)
      .expect("the updated item should still be present with its original id");
    assert_eq!(modified.accepted_amount, dec("9.5"));

    assert!(
      updated.items.iter().all(|item| item.id != delete_id),
      "the omitted item should be hard-deleted from the composite"
    );

    let fresh = updated
      .items
      .iter()
      .find(|item| {
        item.id != unchanged_id && item.id != update_id && item.accepted_amount == dec("4.25")
      })
      .expect("the inserted item should appear with a freshly generated id");
    assert_eq!(fresh.product_id, catalog.product_a_id);
    assert_eq!(fresh.storage_id, catalog.storage_b_id);
  })
  .await;
}

#[tokio::test]
async fn update_acceptance_composite_rejects_duplicate_item_ids() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // Seed: create an acceptance composite with a single item.
    let initial = service
      .acceptance_composite_create(&CreateAcceptanceCompositeRequest {
        acceptance: CreateAcceptanceRequest {
          document_number: "ACC-COMP-UPDATE-DUP".to_string(),
          date_accepted: ts("2026-01-01T00:00:00Z"),
          arrival_type: ArrivalType::Truck,
          source_entity: None,
          contractor_id: catalog.contractor_a_id,
          truck_waybill_id: None,
          rail_waybill_id: None,
          transit_dispatch_id: None,
        },
        items: vec![AcceptanceItemCompositeRequest {
          product_id: catalog.product_a_id,
          storage_id: catalog.storage_a_id,
          accepted_amount: dec("1.0"),
        }],
      })
      .await
      .unwrap();

    let acceptance_id = initial.document.id;
    let existing = &initial.items[0];
    let dup_id = existing.id;

    // Build a request that references the same existing id twice.
    let err = service
      .acceptance_composite_update(acceptance_id, &UpdateAcceptanceCompositeRequest {
        acceptance: UpdateAcceptanceRequest {
          document_number: None,
          date_accepted: None,
          arrival_type: None,
          source_entity: None,
          contractor_id: None,
          truck_waybill_id: None,
          rail_waybill_id: None,
          transit_dispatch_id: None,
        },
        items: vec![
          UpdateAcceptanceItemCompositeRequest {
            id: Some(dup_id),
            product_id: existing.product_id,
            storage_id: existing.storage_id,
            accepted_amount: dec("1.0"),
          },
          UpdateAcceptanceItemCompositeRequest {
            id: Some(dup_id),
            product_id: existing.product_id,
            storage_id: existing.storage_id,
            accepted_amount: dec("2.0"),
          },
        ],
      })
      .await
      .expect_err("duplicate item ids must be rejected");

    match err {
      ApiError::BadRequest(msg) => {
        assert!(
          msg.contains("duplicate item id in request"),
          "expected duplicate-id error, got: {msg}"
        );
        assert!(
          msg.contains(&dup_id.to_string()),
          "error should name the offending id, got: {msg}"
        );
      }
      other => panic!("expected ApiError::BadRequest, got: {other:?}"),
    }
  })
  .await;
}

#[tokio::test]
async fn dispatch_composite_update_inserts_updates_and_deletes_items_and_measurements() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // Seed ledger balances so the dispatch composite create's draft-only
    // balance check has stock to draw against. Both storages need enough
    // headroom to cover every item line we insert across the create + update.
    seed_ledger_balance(
      &db,
      catalog.storage_a_id,
      catalog.product_a_id,
      catalog.contractor_a_id,
      Decimal::from(100),
    )
    .await;
    seed_ledger_balance(
      &db,
      catalog.storage_b_id,
      catalog.product_a_id,
      catalog.contractor_a_id,
      Decimal::from(100),
    )
    .await;

    // 1. Seed: create a dispatch composite with three items and three storage
    //    measurement rows so we can exercise insert / update / delete on both
    //    child collections within a single update call.
    let initial = service
      .dispatch_composite_create(&CreateDispatchCompositeRequest {
        dispatch: CreateDispatchRequest {
          document_number: "DISP-COMP-UPDATE-1".to_string(),
          date: ts("2026-01-02T00:00:00Z"),
          dispatch_purpose: DispatchPurpose::External,
          dispatch_method: DispatchMethod::Truck,
          contractor_id: catalog.contractor_a_id,
          destination_base_id: None,
          receiver_entity: None,
          start_cargo_ops: None,
          end_cargo_ops: None,
          bunker_type: None,
          exporter_id: None,
          port_id: None,
        },
        items: vec![
          DispatchItemCompositeRequest {
            product_id: catalog.product_a_id,
            storage_id: catalog.storage_a_id,
            dispatched_amount: dec("1.0"),
          },
          DispatchItemCompositeRequest {
            product_id: catalog.product_a_id,
            storage_id: catalog.storage_a_id,
            dispatched_amount: dec("2.0"),
          },
          DispatchItemCompositeRequest {
            product_id: catalog.product_a_id,
            storage_id: catalog.storage_b_id,
            dispatched_amount: dec("3.0"),
          },
        ],
        storage_measurements: Some(vec![
          DispatchMeasurementCompositeRequest {
            storage_id: catalog.storage_a_id,
            before_height: None,
            before_volume: None,
            before_density: None,
            before_mass: dec("100.0"),
            after_height: None,
            after_volume: None,
            after_density: None,
            after_mass: dec("90.0"),
          },
          DispatchMeasurementCompositeRequest {
            storage_id: catalog.storage_a_id,
            before_height: None,
            before_volume: None,
            before_density: None,
            before_mass: dec("200.0"),
            after_height: None,
            after_volume: None,
            after_density: None,
            after_mass: dec("180.0"),
          },
          DispatchMeasurementCompositeRequest {
            storage_id: catalog.storage_b_id,
            before_height: None,
            before_volume: None,
            before_density: None,
            before_mass: dec("300.0"),
            after_height: None,
            after_volume: None,
            after_density: None,
            after_mass: dec("270.0"),
          },
        ]),
      })
      .await
      .unwrap();

    assert_eq!(initial.items.len(), 3);
    assert_eq!(initial.storage_measurements.len(), 3);

    let dispatch_id = initial.document.id;

    // Capture each item by its initial dispatched_amount so the test does not
    // depend on a specific row ordering of the response.
    let pick_item = |amount: Decimal| -> (Uuid, Uuid, Uuid, Decimal) {
      let item = initial
        .items
        .iter()
        .find(|item| item.dispatched_amount == amount)
        .unwrap();
      (
        item.id,
        item.product_id,
        item.storage_id,
        item.dispatched_amount,
      )
    };
    let (item_unchanged_id, item_unchanged_product, item_unchanged_storage, item_unchanged_amount) =
      pick_item(dec("1.0"));
    let (item_update_id, item_update_product, item_update_storage, _) = pick_item(dec("2.0"));
    let (item_delete_id, _, _, _) = pick_item(dec("3.0"));

    // Capture each measurement by its initial before_mass.
    let pick_measurement = |before: Decimal| -> (Uuid, Uuid, Decimal, Decimal) {
      let row = initial
        .storage_measurements
        .iter()
        .find(|row| row.before_mass == before)
        .unwrap();
      (row.id, row.storage_id, row.before_mass, row.after_mass)
    };
    let (m_unchanged_id, m_unchanged_storage, m_unchanged_before, m_unchanged_after) =
      pick_measurement(dec("100.0"));
    let (m_update_id, m_update_storage, _, _) = pick_measurement(dec("200.0"));
    let (m_delete_id, _, _, _) = pick_measurement(dec("300.0"));

    // 2. Apply a composite update covering both child collections:
    //    - keep the unchanged item / measurement,
    //    - update one item's amount and one measurement's after_mass,
    //    - drop one item and one measurement by omitting them,
    //    - insert one fresh item and one fresh measurement with id: None.
    let updated = service
      .dispatch_composite_update(dispatch_id, &UpdateDispatchCompositeRequest {
        dispatch: UpdateDispatchRequest {
          document_number: None,
          date: None,
          dispatch_purpose: None,
          dispatch_method: None,
          contractor_id: None,
          destination_base_id: None,
          receiver_entity: None,
          start_cargo_ops: None,
          end_cargo_ops: None,
          bunker_type: None,
          exporter_id: None,
          port_id: None,
        },
        items: vec![
          UpdateDispatchItemCompositeRequest {
            id: Some(item_unchanged_id),
            product_id: item_unchanged_product,
            storage_id: item_unchanged_storage,
            dispatched_amount: item_unchanged_amount,
          },
          UpdateDispatchItemCompositeRequest {
            id: Some(item_update_id),
            product_id: item_update_product,
            storage_id: item_update_storage,
            dispatched_amount: dec("9.5"),
          },
          UpdateDispatchItemCompositeRequest {
            id: None,
            product_id: catalog.product_a_id,
            storage_id: catalog.storage_b_id,
            dispatched_amount: dec("4.25"),
          },
        ],
        storage_measurements: Some(vec![
          UpdateDispatchMeasurementCompositeRequest {
            id: Some(m_unchanged_id),
            storage_id: m_unchanged_storage,
            before_height: None,
            before_volume: None,
            before_density: None,
            before_mass: m_unchanged_before,
            after_height: None,
            after_volume: None,
            after_density: None,
            after_mass: m_unchanged_after,
          },
          UpdateDispatchMeasurementCompositeRequest {
            id: Some(m_update_id),
            storage_id: m_update_storage,
            before_height: None,
            before_volume: None,
            before_density: None,
            before_mass: dec("200.0"),
            after_height: None,
            after_volume: None,
            after_density: None,
            after_mass: dec("150.0"),
          },
          UpdateDispatchMeasurementCompositeRequest {
            id: None,
            storage_id: catalog.storage_b_id,
            before_height: None,
            before_volume: None,
            before_density: None,
            before_mass: dec("400.0"),
            after_height: None,
            after_volume: None,
            after_density: None,
            after_mass: dec("360.0"),
          },
        ]),
      })
      .await
      .unwrap();

    // 3. Item-side assertions on the response.
    assert_eq!(updated.items.len(), 3);

    let unchanged_item = updated
      .items
      .iter()
      .find(|item| item.id == item_unchanged_id)
      .expect("the unchanged item should still be present");
    assert_eq!(unchanged_item.dispatched_amount, dec("1.0"));
    assert_eq!(unchanged_item.storage_id, item_unchanged_storage);

    let modified_item = updated
      .items
      .iter()
      .find(|item| item.id == item_update_id)
      .expect("the updated item should still be present with its original id");
    assert_eq!(modified_item.dispatched_amount, dec("9.5"));

    assert!(
      updated.items.iter().all(|item| item.id != item_delete_id),
      "the omitted item should be hard-deleted from the composite"
    );

    let fresh_item = updated
      .items
      .iter()
      .find(|item| {
        item.id != item_unchanged_id
          && item.id != item_update_id
          && item.dispatched_amount == dec("4.25")
      })
      .expect("the inserted item should appear with a freshly generated id");
    assert_eq!(fresh_item.product_id, catalog.product_a_id);
    assert_eq!(fresh_item.storage_id, catalog.storage_b_id);

    // 4. Measurement-side assertions on the response.
    assert_eq!(updated.storage_measurements.len(), 3);

    let unchanged_m = updated
      .storage_measurements
      .iter()
      .find(|row| row.id == m_unchanged_id)
      .expect("the unchanged measurement should still be present");
    assert_eq!(unchanged_m.before_mass, dec("100.0"));
    assert_eq!(unchanged_m.after_mass, dec("90.0"));

    let modified_m = updated
      .storage_measurements
      .iter()
      .find(|row| row.id == m_update_id)
      .expect("the updated measurement should still be present with its original id");
    assert_eq!(modified_m.after_mass, dec("150.0"));

    assert!(
      updated
        .storage_measurements
        .iter()
        .all(|row| row.id != m_delete_id),
      "the omitted measurement should be hard-deleted from the composite"
    );

    let fresh_m = updated
      .storage_measurements
      .iter()
      .find(|row| {
        row.id != m_unchanged_id && row.id != m_update_id && row.before_mass == dec("400.0")
      })
      .expect("the inserted measurement should appear with a freshly generated id");
    assert_eq!(fresh_m.storage_id, catalog.storage_b_id);
    assert_eq!(fresh_m.after_mass, dec("360.0"));
  })
  .await;
}

#[tokio::test]
async fn dispatch_composite_update_rejects_duplicate_item_ids() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // Seed ledger balance so the dispatch composite create has stock available.
    seed_ledger_balance(
      &db,
      catalog.storage_a_id,
      catalog.product_a_id,
      catalog.contractor_a_id,
      Decimal::from(10),
    )
    .await;

    // Seed: create a dispatch composite with a single item.
    let initial = service
      .dispatch_composite_create(&CreateDispatchCompositeRequest {
        dispatch: CreateDispatchRequest {
          document_number: "DISP-COMP-UPDATE-DUP".to_string(),
          date: ts("2026-01-02T00:00:00Z"),
          dispatch_purpose: DispatchPurpose::External,
          dispatch_method: DispatchMethod::Truck,
          contractor_id: catalog.contractor_a_id,
          destination_base_id: None,
          receiver_entity: None,
          start_cargo_ops: None,
          end_cargo_ops: None,
          bunker_type: None,
          exporter_id: None,
          port_id: None,
        },
        items: vec![DispatchItemCompositeRequest {
          product_id: catalog.product_a_id,
          storage_id: catalog.storage_a_id,
          dispatched_amount: dec("1.0"),
        }],
        storage_measurements: None,
      })
      .await
      .unwrap();

    let dispatch_id = initial.document.id;
    let existing = &initial.items[0];
    let dup_id = existing.id;

    // Build a request that references the same existing id twice.
    let err = service
      .dispatch_composite_update(dispatch_id, &UpdateDispatchCompositeRequest {
        dispatch: UpdateDispatchRequest {
          document_number: None,
          date: None,
          dispatch_purpose: None,
          dispatch_method: None,
          contractor_id: None,
          destination_base_id: None,
          receiver_entity: None,
          start_cargo_ops: None,
          end_cargo_ops: None,
          bunker_type: None,
          exporter_id: None,
          port_id: None,
        },
        items: vec![
          UpdateDispatchItemCompositeRequest {
            id: Some(dup_id),
            product_id: existing.product_id,
            storage_id: existing.storage_id,
            dispatched_amount: dec("1.0"),
          },
          UpdateDispatchItemCompositeRequest {
            id: Some(dup_id),
            product_id: existing.product_id,
            storage_id: existing.storage_id,
            dispatched_amount: dec("2.0"),
          },
        ],
        storage_measurements: None,
      })
      .await
      .expect_err("duplicate item ids must be rejected");

    match err {
      ApiError::BadRequest(msg) => {
        assert!(
          msg.contains("duplicate dispatch item id in request"),
          "expected duplicate-id error, got: {msg}"
        );
        assert!(
          msg.contains(&dup_id.to_string()),
          "error should name the offending id, got: {msg}"
        );
      }
      other => panic!("expected ApiError::BadRequest, got: {other:?}"),
    }
  })
  .await;
}
