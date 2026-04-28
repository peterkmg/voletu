use std::{str::FromStr, sync::Arc};

use chrono::{DateTime, Utc};
use sea_orm::prelude::Decimal;
use uuid::Uuid;
use voletu_core::{
  api::ApiError,
  context::audit::with_audit_context,
  dtos::{
    CreateInventoryAdjustmentRequest,
    CreateInventoryReconciliationCompositeRequest,
    CreateInventoryReconciliationRequest,
    InventoryAdjustmentCompositeRequest,
    UpdateInventoryAdjustmentCompositeRequest,
    UpdateInventoryReconciliationCompositeRequest,
    UpdateInventoryReconciliationRequest,
  },
  enums::AdjustmentType,
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
async fn adjustments_apply_on_execute_and_reverse_on_revert() {
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
      Decimal::from(5),
    )
    .await;

    let reconciliation = service
      .reconciliation_create(&CreateInventoryReconciliationRequest {
        document_number: "REC-1".to_string(),
        date: ts("2026-01-03T00:00:00Z"),
        contractor_id: catalog.contractor_a_id,
        warehouse_id: catalog.warehouse_id,
      })
      .await
      .unwrap();

    service
      .adjustment_create(&CreateInventoryAdjustmentRequest {
        reconciliation_id: reconciliation.id,
        storage_id: catalog.storage_a_id,
        product_id: catalog.product_a_id,
        adjustment_type: AdjustmentType::Surplus,
        amount: dec("3.0"),
        reason: Some("Counted extra".to_string()),
      })
      .await
      .unwrap();
    service
      .adjustment_create(&CreateInventoryAdjustmentRequest {
        reconciliation_id: reconciliation.id,
        storage_id: catalog.storage_a_id,
        product_id: catalog.product_a_id,
        adjustment_type: AdjustmentType::Loss,
        amount: dec("2.0"),
        reason: Some("Evaporation".to_string()),
      })
      .await
      .unwrap();

    service
      .reconciliation_execute(reconciliation.id, Uuid::now_v7())
      .await
      .unwrap();

    let final_entry = ledger
      .by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(final_entry.current_amount, dec("6.0"));

    service
      .reconciliation_revert(reconciliation.id, Uuid::now_v7())
      .await
      .unwrap();
    let reverted_entry = ledger
      .by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(reverted_entry.current_amount, dec("5.0"));
    assert_eq!(service.reconciliation_list(None).await.unwrap().len(), 1);
    assert_eq!(service.adjustment_list(None).await.unwrap().len(), 2);
  })
  .await;
}

#[tokio::test]
async fn inventory_composite_create_with_adjustments() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    let response = service
      .inventory_reconciliation_composite_create(&CreateInventoryReconciliationCompositeRequest {
        reconciliation: CreateInventoryReconciliationRequest {
          document_number: "REC-COMP-1".to_string(),
          date: ts("2026-01-03T00:00:00Z"),
          contractor_id: catalog.contractor_a_id,
          warehouse_id: catalog.warehouse_id,
        },
        adjustments: vec![
          InventoryAdjustmentCompositeRequest {
            storage_id: catalog.storage_a_id,
            product_id: catalog.product_a_id,
            adjustment_type: AdjustmentType::Surplus,
            amount: dec("3.0"),
            reason: Some("Counted extra".to_string()),
          },
          InventoryAdjustmentCompositeRequest {
            storage_id: catalog.storage_a_id,
            product_id: catalog.product_a_id,
            adjustment_type: AdjustmentType::Loss,
            amount: dec("2.0"),
            reason: Some("Evaporation".to_string()),
          },
          InventoryAdjustmentCompositeRequest {
            storage_id: catalog.storage_b_id,
            product_id: catalog.product_a_id,
            adjustment_type: AdjustmentType::Surplus,
            amount: dec("1.5"),
            reason: None,
          },
        ],
      })
      .await
      .unwrap();

    // All three adjustments must be persisted under the new reconciliation.
    assert_eq!(response.document.document_number, "REC-COMP-1");
    assert_eq!(response.adjustments.len(), 3);
    for adj in &response.adjustments {
      assert_eq!(adj.reconciliation_id, response.document.id);
    }

    // Spot-check values survive the round-trip.
    let surplus_a = response
      .adjustments
      .iter()
      .find(|a| {
        a.storage_id == catalog.storage_a_id && a.adjustment_type == AdjustmentType::Surplus
      })
      .expect("surplus adjustment on storage A should exist");
    assert_eq!(surplus_a.amount, dec("3.0"));

    let loss = response
      .adjustments
      .iter()
      .find(|a| a.adjustment_type == AdjustmentType::Loss)
      .expect("loss adjustment should exist");
    assert_eq!(loss.amount, dec("2.0"));

    let surplus_b = response
      .adjustments
      .iter()
      .find(|a| a.storage_id == catalog.storage_b_id)
      .expect("surplus adjustment on storage B should exist");
    assert_eq!(surplus_b.amount, dec("1.5"));

    // Confirm the adjustments are queryable via the plain adjustment API too.
    assert_eq!(service.adjustment_list(None).await.unwrap().len(), 3);
  })
  .await;
}

#[tokio::test]
async fn inventory_composite_update_inserts_updates_and_deletes_adjustments() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // 1. Seed: create a reconciliation composite with three adjustments.
    let initial = service
      .inventory_reconciliation_composite_create(&CreateInventoryReconciliationCompositeRequest {
        reconciliation: CreateInventoryReconciliationRequest {
          document_number: "REC-COMP-UPDATE-1".to_string(),
          date: ts("2026-01-03T00:00:00Z"),
          contractor_id: catalog.contractor_a_id,
          warehouse_id: catalog.warehouse_id,
        },
        adjustments: vec![
          InventoryAdjustmentCompositeRequest {
            storage_id: catalog.storage_a_id,
            product_id: catalog.product_a_id,
            adjustment_type: AdjustmentType::Surplus,
            amount: dec("1.0"),
            reason: None,
          },
          InventoryAdjustmentCompositeRequest {
            storage_id: catalog.storage_a_id,
            product_id: catalog.product_a_id,
            adjustment_type: AdjustmentType::Loss,
            amount: dec("2.0"),
            reason: None,
          },
          InventoryAdjustmentCompositeRequest {
            storage_id: catalog.storage_b_id,
            product_id: catalog.product_a_id,
            adjustment_type: AdjustmentType::Surplus,
            amount: dec("3.0"),
            reason: None,
          },
        ],
      })
      .await
      .unwrap();

    assert_eq!(initial.adjustments.len(), 3);
    let reconciliation_id = initial.document.id;

    // Capture each adjustment by its initial amount so the test does not
    // depend on response ordering.
    let pick = |amount: Decimal| -> (Uuid, Uuid, Uuid, AdjustmentType, Decimal) {
      let adj = initial
        .adjustments
        .iter()
        .find(|a| a.amount == amount)
        .unwrap();
      (
        adj.id,
        adj.storage_id,
        adj.product_id,
        adj.adjustment_type,
        adj.amount,
      )
    };
    let (unchanged_id, unchanged_storage, unchanged_product, unchanged_type, unchanged_amount) =
      pick(dec("1.0"));
    let (update_id, update_storage, update_product, update_type, _) = pick(dec("2.0"));
    let (delete_id, _, _, _, _) = pick(dec("3.0"));

    // 2. Apply a composite update:
    //    - keep the first adjustment as-is,
    //    - update the second adjustment's amount,
    //    - drop the third adjustment by omitting it,
    //    - insert one fresh adjustment with id: None.
    let updated = service
      .inventory_reconciliation_composite_update(
        reconciliation_id,
        &UpdateInventoryReconciliationCompositeRequest {
          reconciliation: UpdateInventoryReconciliationRequest {
            document_number: None,
            date: None,
            contractor_id: None,
            warehouse_id: None,
          },
          adjustments: vec![
            UpdateInventoryAdjustmentCompositeRequest {
              id: Some(unchanged_id),
              storage_id: unchanged_storage,
              product_id: unchanged_product,
              adjustment_type: unchanged_type,
              amount: unchanged_amount,
              reason: None,
            },
            UpdateInventoryAdjustmentCompositeRequest {
              id: Some(update_id),
              storage_id: update_storage,
              product_id: update_product,
              adjustment_type: update_type,
              amount: dec("9.5"),
              reason: Some("corrected".to_string()),
            },
            UpdateInventoryAdjustmentCompositeRequest {
              id: None,
              storage_id: catalog.storage_b_id,
              product_id: catalog.product_a_id,
              adjustment_type: AdjustmentType::Loss,
              amount: dec("4.25"),
              reason: None,
            },
          ],
        },
      )
      .await
      .unwrap();

    // 3. Assertions on the response.
    assert_eq!(updated.adjustments.len(), 3);

    let unchanged = updated
      .adjustments
      .iter()
      .find(|a| a.id == unchanged_id)
      .expect("the unchanged adjustment should still be present");
    assert_eq!(unchanged.amount, dec("1.0"));
    assert_eq!(unchanged.storage_id, unchanged_storage);

    let modified = updated
      .adjustments
      .iter()
      .find(|a| a.id == update_id)
      .expect("the updated adjustment should keep its original id");
    assert_eq!(modified.amount, dec("9.5"));
    assert_eq!(modified.reason.as_deref(), Some("corrected"));

    assert!(
      updated.adjustments.iter().all(|a| a.id != delete_id),
      "the omitted adjustment should be hard-deleted from the composite"
    );

    let fresh = updated
      .adjustments
      .iter()
      .find(|a| a.id != unchanged_id && a.id != update_id && a.amount == dec("4.25"))
      .expect("the inserted adjustment should appear with a freshly generated id");
    assert_eq!(fresh.storage_id, catalog.storage_b_id);
    assert_eq!(fresh.adjustment_type, AdjustmentType::Loss);
  })
  .await;
}

#[tokio::test]
async fn inventory_composite_update_rejects_duplicate_adjustment_ids() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // Seed: create a reconciliation composite with a single adjustment.
    let initial = service
      .inventory_reconciliation_composite_create(&CreateInventoryReconciliationCompositeRequest {
        reconciliation: CreateInventoryReconciliationRequest {
          document_number: "REC-COMP-UPDATE-DUP".to_string(),
          date: ts("2026-01-03T00:00:00Z"),
          contractor_id: catalog.contractor_a_id,
          warehouse_id: catalog.warehouse_id,
        },
        adjustments: vec![InventoryAdjustmentCompositeRequest {
          storage_id: catalog.storage_a_id,
          product_id: catalog.product_a_id,
          adjustment_type: AdjustmentType::Surplus,
          amount: dec("1.0"),
          reason: None,
        }],
      })
      .await
      .unwrap();

    let reconciliation_id = initial.document.id;
    let existing = &initial.adjustments[0];
    let dup_id = existing.id;

    // Build a request that references the same existing id twice.
    let err = service
      .inventory_reconciliation_composite_update(
        reconciliation_id,
        &UpdateInventoryReconciliationCompositeRequest {
          reconciliation: UpdateInventoryReconciliationRequest {
            document_number: None,
            date: None,
            contractor_id: None,
            warehouse_id: None,
          },
          adjustments: vec![
            UpdateInventoryAdjustmentCompositeRequest {
              id: Some(dup_id),
              storage_id: existing.storage_id,
              product_id: existing.product_id,
              adjustment_type: existing.adjustment_type,
              amount: dec("1.0"),
              reason: None,
            },
            UpdateInventoryAdjustmentCompositeRequest {
              id: Some(dup_id),
              storage_id: existing.storage_id,
              product_id: existing.product_id,
              adjustment_type: existing.adjustment_type,
              amount: dec("2.0"),
              reason: None,
            },
          ],
        },
      )
      .await
      .expect_err("duplicate adjustment ids must be rejected");

    match err {
      ApiError::BadRequest(msg) => {
        assert!(
          msg.contains("duplicate adjustment id in request"),
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
