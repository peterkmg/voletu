use std::{str::FromStr, sync::Arc};

use chrono::{DateTime, Utc};
use sea_orm::{prelude::Decimal, EntityTrait};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  dtos::{
    CreateOwnershipTransferRequest,
    CreatePhysicalTransferRequest,
    OwnershipTransferItemCompositeRequest,
    PhysicalTransferItemCompositeRequest,
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
      .by_dimensions(
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
      .by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let after_to = ledger
      .by_dimensions(
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
      .by_dimensions(
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
      .by_dimensions(
        catalog.storage_b_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let owner_b = ledger
      .by_dimensions(
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
async fn operations_create_and_execute_shortcuts_post_documents_and_apply_ledger_effects() {
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

    let physical_model = physical_storage_transfer::Entity::find_by_id(physical.id)
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

    let ownership_model = ownership_transfer::Entity::find_by_id(ownership.id)
      .one(db.as_ref())
      .await
      .unwrap()
      .unwrap();
    assert_eq!(ownership_model.status, DocumentStatus::Executed);

    let source = ledger
      .by_dimensions(
        catalog.storage_b_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let target = ledger
      .by_dimensions(
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
