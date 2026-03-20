use std::{str::FromStr, sync::Arc};

use chrono::{DateTime, Utc};
use sea_orm::{prelude::Decimal, EntityTrait};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  dtos::{
    BlendingComponentCompositeRequest,
    BlendingResultCompositeRequest,
    CreateBlendingComponentRequest,
    CreateBlendingRequest,
    CreateBlendingResultRequest,
    CreateInventoryAdjustmentRequest,
    CreateInventoryReconciliationRequest,
    CreateOwnershipTransferRequest,
    CreatePhysicalTransferRequest,
    OwnershipTransferItemCompositeRequest,
    PhysicalTransferItemCompositeRequest,
  },
  entities::{blending_document, ownership_transfer, physical_storage_transfer},
  enums::{AdjustmentType, DocumentStatus},
  services::{audit::AuditService, document::DocumentService, ledger::LedgerService},
};

use crate::common::{
  fixtures::{seed_inventory_fixture, seed_ledger_balance},
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
    let fixture = seed_inventory_fixture(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger.clone(), audit);

    seed_ledger_balance(
      &db,
      fixture.storage_a_id,
      fixture.product_a_id,
      fixture.contractor_a_id,
      Decimal::from(100),
    )
    .await;

    let physical = service
      .physical_transfer_composite_create(&CreatePhysicalTransferRequest {
        document_number: "PT-1".to_string(),
        date: ts("2026-01-01T00:00:00Z"),
        start_cargo_ops: ts("2026-01-01T01:00:00Z"),
        end_cargo_ops: ts("2026-01-01T02:00:00Z"),
        items: vec![
          PhysicalTransferItemCompositeRequest {
            contractor_id: fixture.contractor_a_id,
            product_id: fixture.product_a_id,
            from_storage_id: fixture.storage_a_id,
            to_storage_id: fixture.storage_b_id,
            amount: dec("30.0"),
          },
          PhysicalTransferItemCompositeRequest {
            contractor_id: fixture.contractor_a_id,
            product_id: fixture.product_a_id,
            from_storage_id: fixture.storage_a_id,
            to_storage_id: fixture.storage_b_id,
            amount: dec("5.0"),
          },
        ],
      })
      .await
      .unwrap();

    let before_execute_from = ledger
      .by_dimensions(
        fixture.storage_a_id,
        fixture.product_a_id,
        fixture.contractor_a_id,
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
      .by_dimensions(
        fixture.storage_a_id,
        fixture.product_a_id,
        fixture.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let after_to = ledger
      .by_dimensions(
        fixture.storage_b_id,
        fixture.product_a_id,
        fixture.contractor_a_id,
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
            storage_id: fixture.storage_b_id,
            product_id: fixture.product_a_id,
            from_contractor_id: fixture.contractor_a_id,
            to_contractor_id: fixture.contractor_b_id,
            amount: dec("10.0"),
          },
          OwnershipTransferItemCompositeRequest {
            storage_id: fixture.storage_b_id,
            product_id: fixture.product_a_id,
            from_contractor_id: fixture.contractor_a_id,
            to_contractor_id: fixture.contractor_b_id,
            amount: dec("4.0"),
          },
        ],
      })
      .await
      .unwrap();

    let ownership_before_execute = ledger
      .by_dimensions(
        fixture.storage_b_id,
        fixture.product_a_id,
        fixture.contractor_a_id,
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
      .by_dimensions(
        fixture.storage_b_id,
        fixture.product_a_id,
        fixture.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let owner_b = ledger
      .by_dimensions(
        fixture.storage_b_id,
        fixture.product_a_id,
        fixture.contractor_b_id,
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
async fn blending_execution_requires_balanced_component_and_result_totals() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let fixture = seed_inventory_fixture(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger.clone(), audit);

    seed_ledger_balance(
      &db,
      fixture.storage_a_id,
      fixture.product_a_id,
      fixture.contractor_a_id,
      Decimal::from(50),
    )
    .await;

    let bad_doc = service
      .blending_document_create(&CreateBlendingRequest {
        document_number: "BLD-BAD".to_string(),
        date: ts("2026-01-02T00:00:00Z"),
        contractor_id: fixture.contractor_a_id,
        target_product_id: fixture.product_b_id,
      })
      .await
      .unwrap();
    service
      .blending_component_create(&CreateBlendingComponentRequest {
        blending_doc_id: bad_doc.id,
        component: BlendingComponentCompositeRequest {
          storage_id: fixture.storage_a_id,
          source_product_id: fixture.product_a_id,
          amount_used: dec("20.0"),
        },
      })
      .await
      .unwrap();
    service
      .blending_result_create(&CreateBlendingResultRequest {
        blending_doc_id: bad_doc.id,
        result: BlendingResultCompositeRequest {
          storage_id: fixture.storage_b_id,
          produced_amount: dec("10.0"),
        },
      })
      .await
      .unwrap();
    let err = service
      .blending_document_execute(bad_doc.id, Uuid::now_v7())
      .await
      .unwrap_err();
    assert!(err
      .to_string()
      .contains("Blending document components and results do not match"));

    let good_doc = service
      .blending_document_create(&CreateBlendingRequest {
        document_number: "BLD-GOOD".to_string(),
        date: ts("2026-01-02T01:00:00Z"),
        contractor_id: fixture.contractor_a_id,
        target_product_id: fixture.product_b_id,
      })
      .await
      .unwrap();
    service
      .blending_component_create(&CreateBlendingComponentRequest {
        blending_doc_id: good_doc.id,
        component: BlendingComponentCompositeRequest {
          storage_id: fixture.storage_a_id,
          source_product_id: fixture.product_a_id,
          amount_used: dec("15.0"),
        },
      })
      .await
      .unwrap();
    service
      .blending_result_create(&CreateBlendingResultRequest {
        blending_doc_id: good_doc.id,
        result: BlendingResultCompositeRequest {
          storage_id: fixture.storage_b_id,
          produced_amount: dec("15.0"),
        },
      })
      .await
      .unwrap();
    service
      .blending_document_execute(good_doc.id, Uuid::now_v7())
      .await
      .unwrap();

    let src = ledger
      .by_dimensions(
        fixture.storage_a_id,
        fixture.product_a_id,
        fixture.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let dst = ledger
      .by_dimensions(
        fixture.storage_b_id,
        fixture.product_b_id,
        fixture.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(src.current_amount, dec("35.0"));
    assert_eq!(dst.current_amount, dec("15.0"));
  })
  .await;
}

#[tokio::test]
async fn blending_simple_create_and_execute_requires_components_and_results() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let fixture = seed_inventory_fixture(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    let err = service
      .blending_document_create_and_execute(
        &CreateBlendingRequest {
          document_number: "BLD-SIMPLE-EXEC".to_string(),
          date: ts("2026-01-02T03:00:00Z"),
          contractor_id: fixture.contractor_a_id,
          target_product_id: fixture.product_b_id,
        },
        Uuid::now_v7(),
      )
      .await
      .unwrap_err();

    assert!(err
      .to_string()
      .contains("Cannot execute blending document without components"));
  })
  .await;
}

#[tokio::test]
async fn reconciliation_adjustments_apply_on_execute_and_reverse_on_revert() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let fixture = seed_inventory_fixture(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger.clone(), audit);

    seed_ledger_balance(
      &db,
      fixture.storage_a_id,
      fixture.product_a_id,
      fixture.contractor_a_id,
      Decimal::from(5),
    )
    .await;

    let reconciliation = service
      .reconciliation_create(&CreateInventoryReconciliationRequest {
        document_number: "REC-1".to_string(),
        date: ts("2026-01-03T00:00:00Z"),
        warehouse_id: fixture.warehouse_id,
      })
      .await
      .unwrap();

    service
      .adjustment_create(&CreateInventoryAdjustmentRequest {
        reconciliation_id: reconciliation.id,
        storage_id: fixture.storage_a_id,
        product_id: fixture.product_a_id,
        contractor_id: fixture.contractor_a_id,
        adjustment_type: AdjustmentType::Surplus,
        amount: dec("3.0"),
        reason: Some("Counted extra".to_string()),
      })
      .await
      .unwrap();
    service
      .adjustment_create(&CreateInventoryAdjustmentRequest {
        reconciliation_id: reconciliation.id,
        storage_id: fixture.storage_a_id,
        product_id: fixture.product_a_id,
        contractor_id: fixture.contractor_a_id,
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
        fixture.storage_a_id,
        fixture.product_a_id,
        fixture.contractor_a_id,
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
        fixture.storage_a_id,
        fixture.product_a_id,
        fixture.contractor_a_id,
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
async fn operations_create_and_execute_shortcuts_post_documents_and_apply_ledger_effects() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let fixture = seed_inventory_fixture(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger.clone(), audit);

    seed_ledger_balance(
      &db,
      fixture.storage_a_id,
      fixture.product_a_id,
      fixture.contractor_a_id,
      Decimal::from(100),
    )
    .await;

    let physical = service
      .physical_transfer_composite_create_and_execute(
        &CreatePhysicalTransferRequest {
          document_number: "PT-EXEC".to_string(),
          date: ts("2026-01-05T00:00:00Z"),
          start_cargo_ops: ts("2026-01-05T01:00:00Z"),
          end_cargo_ops: ts("2026-01-05T02:00:00Z"),
          items: vec![PhysicalTransferItemCompositeRequest {
            contractor_id: fixture.contractor_a_id,
            product_id: fixture.product_a_id,
            from_storage_id: fixture.storage_a_id,
            to_storage_id: fixture.storage_b_id,
            amount: dec("30.0"),
          }],
        },
        Uuid::now_v7(),
      )
      .await
      .unwrap();

    let physical_model = physical_storage_transfer::Entity::find_by_id(physical.id)
      .one(db.as_ref())
      .await
      .unwrap()
      .unwrap();
    assert_eq!(physical_model.status, DocumentStatus::Posted);

    let ownership = service
      .ownership_transfer_composite_create_and_execute(
        &CreateOwnershipTransferRequest {
          date: ts("2026-01-05T03:00:00Z"),
          items: vec![OwnershipTransferItemCompositeRequest {
            storage_id: fixture.storage_b_id,
            product_id: fixture.product_a_id,
            from_contractor_id: fixture.contractor_a_id,
            to_contractor_id: fixture.contractor_b_id,
            amount: dec("10.0"),
          }],
        },
        Uuid::now_v7(),
      )
      .await
      .unwrap();

    let ownership_model = ownership_transfer::Entity::find_by_id(ownership.id)
      .one(db.as_ref())
      .await
      .unwrap()
      .unwrap();
    assert_eq!(ownership_model.status, DocumentStatus::Posted);

    let source = ledger
      .by_dimensions(
        fixture.storage_b_id,
        fixture.product_a_id,
        fixture.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let target = ledger
      .by_dimensions(
        fixture.storage_b_id,
        fixture.product_a_id,
        fixture.contractor_b_id,
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
async fn blending_component_creation_rejects_posted_document_mutation() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let fixture = seed_inventory_fixture(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    seed_ledger_balance(
      &db,
      fixture.storage_a_id,
      fixture.product_a_id,
      fixture.contractor_a_id,
      Decimal::from(20),
    )
    .await;

    let doc = service
      .blending_document_create(&CreateBlendingRequest {
        document_number: "BLD-MUT".to_string(),
        date: ts("2026-01-06T00:00:00Z"),
        contractor_id: fixture.contractor_a_id,
        target_product_id: fixture.product_b_id,
      })
      .await
      .unwrap();

    service
      .blending_component_create(&CreateBlendingComponentRequest {
        blending_doc_id: doc.id,
        component: BlendingComponentCompositeRequest {
          storage_id: fixture.storage_a_id,
          source_product_id: fixture.product_a_id,
          amount_used: dec("10.0"),
        },
      })
      .await
      .unwrap();

    service
      .blending_result_create(&CreateBlendingResultRequest {
        blending_doc_id: doc.id,
        result: BlendingResultCompositeRequest {
          storage_id: fixture.storage_b_id,
          produced_amount: dec("10.0"),
        },
      })
      .await
      .unwrap();

    service
      .blending_document_execute(doc.id, Uuid::now_v7())
      .await
      .unwrap();

    let posted = blending_document::Entity::find_by_id(doc.id)
      .one(db.as_ref())
      .await
      .unwrap()
      .unwrap();
    assert_eq!(posted.status, DocumentStatus::Posted);

    let err = service
      .blending_component_create(&CreateBlendingComponentRequest {
        blending_doc_id: doc.id,
        component: BlendingComponentCompositeRequest {
          storage_id: fixture.storage_a_id,
          source_product_id: fixture.product_a_id,
          amount_used: dec("1.0"),
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
