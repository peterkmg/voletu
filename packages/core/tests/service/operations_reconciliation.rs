use std::{str::FromStr, sync::Arc};

use chrono::{DateTime, Utc};
use sea_orm::prelude::Decimal;
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  dtos::{CreateInventoryAdjustmentRequest, CreateInventoryReconciliationRequest},
  enums::AdjustmentType,
  services::{audit::AuditService, document::DocumentService, ledger::LedgerService},
};

use crate::common::{
  catalog_seed::{seed_inventory_catalog, seed_ledger_balance},
  setup_db, test_config,
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
async fn reconciliation_adjustments_apply_on_execute_and_reverse_on_revert() {
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
