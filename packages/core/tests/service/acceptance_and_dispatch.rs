use std::{str::FromStr, sync::Arc};

use assert_json_diff::assert_json_eq;
use chrono::{DateTime, Utc};
use sea_orm::{prelude::Decimal, ActiveModelTrait, ActiveValue::Set, EntityLoaderTrait};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  dtos::{
    AcceptanceItemCompositeRequest,
    CreateAcceptanceItemRequest,
    CreateAcceptanceRequest,
    CreateDispatchItemRequest,
    CreateDispatchRequest,
    DispatchItemCompositeRequest,
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

    let created_doc_model = acceptance_document::Entity::load()
      .filter_by_id(doc.id)
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
    let updated_doc = acceptance_document::Entity::load()
      .filter_by_id(doc.id)
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
