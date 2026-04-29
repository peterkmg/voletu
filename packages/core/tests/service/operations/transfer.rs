use std::{str::FromStr, sync::Arc};

use chrono::{DateTime, Utc};
use sea_orm::{prelude::Decimal, EntityLoaderTrait};
use uuid::Uuid;
use voletu_core::{
  api::ApiError,
  context::audit::with_audit_context,
  dtos::{
    CreateOwnershipTransferRequest,
    CreatePhysicalTransferRequest,
    OwnershipTransferItemCompositeRequest,
    PhysicalTransferItemCompositeRequest,
    UpdateOwnershipTransferCompositeRequest,
    UpdateOwnershipTransferItemCompositeRequest,
    UpdateOwnershipTransferRequest,
    UpdatePhysicalTransferCompositeRequest,
    UpdatePhysicalTransferItemCompositeRequest,
    UpdatePhysicalTransferRequest,
  },
  entities::{ownership_transfer, physical_storage_transfer},
  enums::DocumentStatus,
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
async fn physical_and_ownership_transfers_apply_on_execute() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger.clone(), audit);

    seed_ledger_balance(
      &db,
      catalog.storage_a_id,
      catalog.product_a_id,
      catalog.contractor_a_id,
      Decimal::from(100),
    )
    .await;

    let physical = service
      .physical_transfer_composite_create(&CreatePhysicalTransferRequest {
        document_number: "PT-1".to_string(),
        date: ts("2026-01-01T00:00:00Z"),
        contractor_id: catalog.contractor_a_id,
        start_cargo_ops: ts("2026-01-01T01:00:00Z"),
        end_cargo_ops: ts("2026-01-01T02:00:00Z"),
        items: vec![
          PhysicalTransferItemCompositeRequest {
            product_id: catalog.product_a_id,
            from_storage_id: catalog.storage_a_id,
            to_storage_id: catalog.storage_b_id,
            amount: dec("30.0"),
          },
          PhysicalTransferItemCompositeRequest {
            product_id: catalog.product_a_id,
            from_storage_id: catalog.storage_a_id,
            to_storage_id: catalog.storage_b_id,
            amount: dec("5.0"),
          },
        ],
      })
      .await
      .unwrap();

    let before_execute_from = ledger
      .balance_by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(before_execute_from.current_amount, dec("100.0"));

    service
      .physical_transfer_execute(physical.id, Uuid::now_v7())
      .await
      .unwrap();

    let after_from = ledger
      .balance_by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let after_to = ledger
      .balance_by_dimensions(
        catalog.storage_b_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(after_from.current_amount, dec("65.0"));
    assert_eq!(after_to.current_amount, dec("35.0"));

    let ownership = service
      .ownership_transfer_composite_create(&CreateOwnershipTransferRequest {
        date: ts("2026-01-01T03:00:00Z"),
        items: vec![
          OwnershipTransferItemCompositeRequest {
            storage_id: catalog.storage_b_id,
            product_id: catalog.product_a_id,
            from_contractor_id: catalog.contractor_a_id,
            to_contractor_id: catalog.contractor_b_id,
            amount: dec("10.0"),
          },
          OwnershipTransferItemCompositeRequest {
            storage_id: catalog.storage_b_id,
            product_id: catalog.product_a_id,
            from_contractor_id: catalog.contractor_a_id,
            to_contractor_id: catalog.contractor_b_id,
            amount: dec("4.0"),
          },
        ],
      })
      .await
      .unwrap();

    let ownership_before_execute = ledger
      .balance_by_dimensions(
        catalog.storage_b_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(ownership_before_execute.current_amount, dec("35.0"));

    service
      .ownership_transfer_execute(ownership.id, Uuid::now_v7())
      .await
      .unwrap();

    let owner_a = ledger
      .balance_by_dimensions(
        catalog.storage_b_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let owner_b = ledger
      .balance_by_dimensions(
        catalog.storage_b_id,
        catalog.product_a_id,
        catalog.contractor_b_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(owner_a.current_amount, dec("21.0"));
    assert_eq!(owner_b.current_amount, dec("14.0"));
    assert_eq!(
      service
        .physical_transfer_composite_list()
        .await
        .unwrap()
        .len(),
      1
    );
    assert_eq!(
      service
        .ownership_transfer_composite_list()
        .await
        .unwrap()
        .len(),
      1
    );
  })
  .await;
}

#[tokio::test]
async fn shortcut_create_execute_posts_documents_and_applies_ledger_effects() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger.clone(), audit);

    seed_ledger_balance(
      &db,
      catalog.storage_a_id,
      catalog.product_a_id,
      catalog.contractor_a_id,
      Decimal::from(100),
    )
    .await;

    let physical = service
      .physical_transfer_composite_create_and_execute(
        &CreatePhysicalTransferRequest {
          document_number: "PT-EXEC".to_string(),
          date: ts("2026-01-05T00:00:00Z"),
          contractor_id: catalog.contractor_a_id,
          start_cargo_ops: ts("2026-01-05T01:00:00Z"),
          end_cargo_ops: ts("2026-01-05T02:00:00Z"),
          items: vec![PhysicalTransferItemCompositeRequest {
            product_id: catalog.product_a_id,
            from_storage_id: catalog.storage_a_id,
            to_storage_id: catalog.storage_b_id,
            amount: dec("30.0"),
          }],
        },
        Uuid::now_v7(),
      )
      .await
      .unwrap();

    let physical_model = physical_storage_transfer::Entity::load()
      .filter_by_id(physical.id)
      .one(db.as_ref())
      .await
      .unwrap()
      .unwrap();
    assert_eq!(physical_model.status, DocumentStatus::Executed);

    let ownership = service
      .ownership_transfer_composite_create_and_execute(
        &CreateOwnershipTransferRequest {
          date: ts("2026-01-05T03:00:00Z"),
          items: vec![OwnershipTransferItemCompositeRequest {
            storage_id: catalog.storage_b_id,
            product_id: catalog.product_a_id,
            from_contractor_id: catalog.contractor_a_id,
            to_contractor_id: catalog.contractor_b_id,
            amount: dec("10.0"),
          }],
        },
        Uuid::now_v7(),
      )
      .await
      .unwrap();

    let ownership_model = ownership_transfer::Entity::load()
      .filter_by_id(ownership.id)
      .one(db.as_ref())
      .await
      .unwrap()
      .unwrap();
    assert_eq!(ownership_model.status, DocumentStatus::Executed);

    let source = ledger
      .balance_by_dimensions(
        catalog.storage_b_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let target = ledger
      .balance_by_dimensions(
        catalog.storage_b_id,
        catalog.product_a_id,
        catalog.contractor_b_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(source.current_amount, dec("20.0"));
    assert_eq!(target.current_amount, dec("10.0"));
  })
  .await;
}

#[tokio::test]
async fn physical_composite_update_inserts_updates_and_deletes_items() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // 1. Seed: create a physical transfer with three items.
    let initial = service
      .physical_transfer_composite_create(&CreatePhysicalTransferRequest {
        document_number: "PT-COMP-UPDATE-1".to_string(),
        date: ts("2026-01-01T00:00:00Z"),
        contractor_id: catalog.contractor_a_id,
        start_cargo_ops: ts("2026-01-01T01:00:00Z"),
        end_cargo_ops: ts("2026-01-01T02:00:00Z"),
        items: vec![
          PhysicalTransferItemCompositeRequest {
            product_id: catalog.product_a_id,
            from_storage_id: catalog.storage_a_id,
            to_storage_id: catalog.storage_b_id,
            amount: dec("1.0"),
          },
          PhysicalTransferItemCompositeRequest {
            product_id: catalog.product_a_id,
            from_storage_id: catalog.storage_a_id,
            to_storage_id: catalog.storage_b_id,
            amount: dec("2.0"),
          },
          PhysicalTransferItemCompositeRequest {
            product_id: catalog.product_a_id,
            from_storage_id: catalog.storage_a_id,
            to_storage_id: catalog.storage_b_id,
            amount: dec("3.0"),
          },
        ],
      })
      .await
      .unwrap();

    assert_eq!(initial.items.len(), 3);
    let physical_id = initial.id;

    // Capture each item id by its initial amount so the test does not depend on row order.
    let pick = |amount: Decimal| -> (Uuid, Uuid, Uuid, Uuid, Decimal) {
      let item = initial
        .items
        .iter()
        .find(|item| item.amount == amount)
        .unwrap();
      (
        item.id,
        item.product_id,
        item.from_storage_id,
        item.to_storage_id,
        item.amount,
      )
    };
    let (unchanged_id, unchanged_product, unchanged_from, unchanged_to, unchanged_amount) =
      pick(dec("1.0"));
    let (update_id, update_product, update_from, update_to, _) = pick(dec("2.0"));
    let (delete_id, _, _, _, _) = pick(dec("3.0"));

    // 2. Apply a composite update:
    //    - keep item_unchanged as-is,
    //    - update item_to_update.amount,
    //    - drop item_to_delete by omitting it,
    //    - insert one fresh item with id: None.
    let updated = service
      .physical_transfer_composite_update(physical_id, &UpdatePhysicalTransferCompositeRequest {
        physical_transfer: UpdatePhysicalTransferRequest {
          document_number: None,
          date: None,
          contractor_id: None,
          start_cargo_ops: None,
          end_cargo_ops: None,
        },
        items: vec![
          UpdatePhysicalTransferItemCompositeRequest {
            id: Some(unchanged_id),
            product_id: unchanged_product,
            from_storage_id: unchanged_from,
            to_storage_id: unchanged_to,
            amount: unchanged_amount,
          },
          UpdatePhysicalTransferItemCompositeRequest {
            id: Some(update_id),
            product_id: update_product,
            from_storage_id: update_from,
            to_storage_id: update_to,
            amount: dec("9.5"),
          },
          UpdatePhysicalTransferItemCompositeRequest {
            id: None,
            product_id: catalog.product_a_id,
            from_storage_id: catalog.storage_a_id,
            to_storage_id: catalog.storage_b_id,
            amount: dec("4.25"),
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
    assert_eq!(unchanged.amount, dec("1.0"));
    assert_eq!(unchanged.from_storage_id, unchanged_from);
    assert_eq!(unchanged.to_storage_id, unchanged_to);

    let modified = updated
      .items
      .iter()
      .find(|item| item.id == update_id)
      .expect("the updated item should still be present with its original id");
    assert_eq!(modified.amount, dec("9.5"));

    assert!(
      updated.items.iter().all(|item| item.id != delete_id),
      "the omitted item should be hard-deleted from the composite"
    );

    let fresh = updated
      .items
      .iter()
      .find(|item| item.id != unchanged_id && item.id != update_id && item.amount == dec("4.25"))
      .expect("the inserted item should appear with a freshly generated id");
    assert_eq!(fresh.product_id, catalog.product_a_id);
    assert_eq!(fresh.from_storage_id, catalog.storage_a_id);
    assert_eq!(fresh.to_storage_id, catalog.storage_b_id);
  })
  .await;
}

#[tokio::test]
async fn physical_composite_update_rejects_duplicate_item_ids() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    let initial = service
      .physical_transfer_composite_create(&CreatePhysicalTransferRequest {
        document_number: "PT-COMP-UPDATE-DUP".to_string(),
        date: ts("2026-01-01T00:00:00Z"),
        contractor_id: catalog.contractor_a_id,
        start_cargo_ops: ts("2026-01-01T01:00:00Z"),
        end_cargo_ops: ts("2026-01-01T02:00:00Z"),
        items: vec![PhysicalTransferItemCompositeRequest {
          product_id: catalog.product_a_id,
          from_storage_id: catalog.storage_a_id,
          to_storage_id: catalog.storage_b_id,
          amount: dec("1.0"),
        }],
      })
      .await
      .unwrap();

    let physical_id = initial.id;
    let existing = &initial.items[0];
    let dup_id = existing.id;

    let err = service
      .physical_transfer_composite_update(physical_id, &UpdatePhysicalTransferCompositeRequest {
        physical_transfer: UpdatePhysicalTransferRequest {
          document_number: None,
          date: None,
          contractor_id: None,
          start_cargo_ops: None,
          end_cargo_ops: None,
        },
        items: vec![
          UpdatePhysicalTransferItemCompositeRequest {
            id: Some(dup_id),
            product_id: existing.product_id,
            from_storage_id: existing.from_storage_id,
            to_storage_id: existing.to_storage_id,
            amount: dec("1.0"),
          },
          UpdatePhysicalTransferItemCompositeRequest {
            id: Some(dup_id),
            product_id: existing.product_id,
            from_storage_id: existing.from_storage_id,
            to_storage_id: existing.to_storage_id,
            amount: dec("2.0"),
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
async fn ownership_composite_update_inserts_updates_and_deletes_items() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // 1. Seed: create an ownership transfer with three items.
    let initial = service
      .ownership_transfer_composite_create(&CreateOwnershipTransferRequest {
        date: ts("2026-01-01T00:00:00Z"),
        items: vec![
          OwnershipTransferItemCompositeRequest {
            storage_id: catalog.storage_a_id,
            product_id: catalog.product_a_id,
            from_contractor_id: catalog.contractor_a_id,
            to_contractor_id: catalog.contractor_b_id,
            amount: dec("1.0"),
          },
          OwnershipTransferItemCompositeRequest {
            storage_id: catalog.storage_a_id,
            product_id: catalog.product_a_id,
            from_contractor_id: catalog.contractor_a_id,
            to_contractor_id: catalog.contractor_b_id,
            amount: dec("2.0"),
          },
          OwnershipTransferItemCompositeRequest {
            storage_id: catalog.storage_a_id,
            product_id: catalog.product_a_id,
            from_contractor_id: catalog.contractor_a_id,
            to_contractor_id: catalog.contractor_b_id,
            amount: dec("3.0"),
          },
        ],
      })
      .await
      .unwrap();

    assert_eq!(initial.items.len(), 3);
    let ownership_id = initial.id;

    // Capture each item id by its initial amount so the test does not depend on row order.
    let pick = |amount: Decimal| -> (Uuid, Uuid, Uuid, Uuid, Uuid, Decimal) {
      let item = initial
        .items
        .iter()
        .find(|item| item.amount == amount)
        .unwrap();
      (
        item.id,
        item.storage_id,
        item.product_id,
        item.from_contractor_id,
        item.to_contractor_id,
        item.amount,
      )
    };
    let (
      unchanged_id,
      unchanged_storage,
      unchanged_product,
      unchanged_from,
      unchanged_to,
      unchanged_amount,
    ) = pick(dec("1.0"));
    let (update_id, update_storage, update_product, update_from, update_to, _) = pick(dec("2.0"));
    let (delete_id, _, _, _, _, _) = pick(dec("3.0"));

    // 2. Apply a composite update:
    //    - keep item_unchanged as-is,
    //    - update item_to_update.amount,
    //    - drop item_to_delete by omitting it,
    //    - insert one fresh item with id: None.
    let updated = service
      .ownership_transfer_composite_update(ownership_id, &UpdateOwnershipTransferCompositeRequest {
        ownership_transfer: UpdateOwnershipTransferRequest { date: None },
        items: vec![
          UpdateOwnershipTransferItemCompositeRequest {
            id: Some(unchanged_id),
            storage_id: unchanged_storage,
            product_id: unchanged_product,
            from_contractor_id: unchanged_from,
            to_contractor_id: unchanged_to,
            amount: unchanged_amount,
          },
          UpdateOwnershipTransferItemCompositeRequest {
            id: Some(update_id),
            storage_id: update_storage,
            product_id: update_product,
            from_contractor_id: update_from,
            to_contractor_id: update_to,
            amount: dec("9.5"),
          },
          UpdateOwnershipTransferItemCompositeRequest {
            id: None,
            storage_id: catalog.storage_a_id,
            product_id: catalog.product_a_id,
            from_contractor_id: catalog.contractor_a_id,
            to_contractor_id: catalog.contractor_b_id,
            amount: dec("4.25"),
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
    assert_eq!(unchanged.amount, dec("1.0"));
    assert_eq!(unchanged.storage_id, unchanged_storage);
    assert_eq!(unchanged.from_contractor_id, unchanged_from);
    assert_eq!(unchanged.to_contractor_id, unchanged_to);

    let modified = updated
      .items
      .iter()
      .find(|item| item.id == update_id)
      .expect("the updated item should still be present with its original id");
    assert_eq!(modified.amount, dec("9.5"));

    assert!(
      updated.items.iter().all(|item| item.id != delete_id),
      "the omitted item should be hard-deleted from the composite"
    );

    let fresh = updated
      .items
      .iter()
      .find(|item| item.id != unchanged_id && item.id != update_id && item.amount == dec("4.25"))
      .expect("the inserted item should appear with a freshly generated id");
    assert_eq!(fresh.storage_id, catalog.storage_a_id);
    assert_eq!(fresh.product_id, catalog.product_a_id);
    assert_eq!(fresh.from_contractor_id, catalog.contractor_a_id);
    assert_eq!(fresh.to_contractor_id, catalog.contractor_b_id);
  })
  .await;
}

#[tokio::test]
async fn ownership_composite_update_rejects_duplicate_item_ids() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    let initial = service
      .ownership_transfer_composite_create(&CreateOwnershipTransferRequest {
        date: ts("2026-01-01T00:00:00Z"),
        items: vec![OwnershipTransferItemCompositeRequest {
          storage_id: catalog.storage_a_id,
          product_id: catalog.product_a_id,
          from_contractor_id: catalog.contractor_a_id,
          to_contractor_id: catalog.contractor_b_id,
          amount: dec("1.0"),
        }],
      })
      .await
      .unwrap();

    let ownership_id = initial.id;
    let existing = &initial.items[0];
    let dup_id = existing.id;

    let err = service
      .ownership_transfer_composite_update(ownership_id, &UpdateOwnershipTransferCompositeRequest {
        ownership_transfer: UpdateOwnershipTransferRequest { date: None },
        items: vec![
          UpdateOwnershipTransferItemCompositeRequest {
            id: Some(dup_id),
            storage_id: existing.storage_id,
            product_id: existing.product_id,
            from_contractor_id: existing.from_contractor_id,
            to_contractor_id: existing.to_contractor_id,
            amount: dec("1.0"),
          },
          UpdateOwnershipTransferItemCompositeRequest {
            id: Some(dup_id),
            storage_id: existing.storage_id,
            product_id: existing.product_id,
            from_contractor_id: existing.from_contractor_id,
            to_contractor_id: existing.to_contractor_id,
            amount: dec("2.0"),
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
