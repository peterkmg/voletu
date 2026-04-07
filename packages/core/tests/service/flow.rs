use std::sync::Arc;

use chrono::NaiveDate;
use sea_orm::{prelude::Decimal, ActiveModelTrait, ActiveValue::Set};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  entities::{acceptance_document, acceptance_item, truck_waybill, truck_waybill_item},
  enums::{self, PipelineStatus},
  services::{audit::AuditService, document::DocumentService, ledger::LedgerService},
};

use crate::common::{catalog_seed::seed_inventory_catalog, setup_db};

fn date(s: &str) -> NaiveDate {
  NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}

#[tokio::test]
async fn truck_receipt_flow_returns_correct_pipeline_statuses() {
  let actor = Uuid::now_v7();
  let origin = Uuid::now_v7();

  with_audit_context(actor, origin, || async {
    let db = Arc::new(setup_db().await);
    let fix = seed_inventory_catalog(&db).await;
    let mut cfg = crate::common::test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let svc = DocumentService::new(db.clone(), ledger, audit);

    // Waybill 1: no acceptance → Pending
    let wb1 = truck_waybill::ActiveModel {
      document_number: Set("TWB-001".into()),
      date: Set(date("2026-04-01")),
      sender_id: Set(fix.sender_id),
      base_id: Set(fix.base_id),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    truck_waybill_item::ActiveModel {
      truck_waybill_id: Set(wb1.id),
      product_id: Set(fix.product_a_id),
      declared_amount: Set(Decimal::new(25000, 0)),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    // Waybill 2: with draft acceptance → Draft
    let wb2 = truck_waybill::ActiveModel {
      document_number: Set("TWB-002".into()),
      date: Set(date("2026-03-31")),
      sender_id: Set(fix.sender_id),
      base_id: Set(fix.base_id),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let acc2 = acceptance_document::ActiveModel {
      document_number: Set("ACC-001".into()),
      date_accepted: Set(chrono::Utc::now()),
      status: Set(enums::DocumentStatus::Draft),
      version: Set(1),
      arrival_type: Set(enums::ArrivalType::Truck),
      contractor_id: Set(fix.contractor_a_id),
      truck_waybill_id: Set(Some(wb2.id)),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    // Waybill 3: with posted acceptance + item → Executed
    let wb3 = truck_waybill::ActiveModel {
      document_number: Set("TWB-003".into()),
      date: Set(date("2026-03-30")),
      sender_id: Set(fix.sender_id),
      base_id: Set(fix.base_id),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let acc3 = acceptance_document::ActiveModel {
      document_number: Set("ACC-002".into()),
      date_accepted: Set(chrono::Utc::now()),
      status: Set(enums::DocumentStatus::Executed),
      version: Set(1),
      arrival_type: Set(enums::ArrivalType::Truck),
      contractor_id: Set(fix.contractor_a_id),
      truck_waybill_id: Set(Some(wb3.id)),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    acceptance_item::ActiveModel {
      acceptance_doc_id: Set(acc3.id),
      product_id: Set(fix.product_a_id),
      storage_id: Set(fix.storage_a_id),
      accepted_amount: Set(Decimal::new(24900, 0)),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    // --- Assertions ---

    // All rows
    let all = svc
      .truck_receipt_pipeline_query(None, None, Some(1), Some(50))
      .await
      .unwrap();
    assert_eq!(all.len(), 3);

    let pending = all
      .iter()
      .find(|r| r.basis_document_number == "TWB-001")
      .unwrap();
    assert_eq!(pending.pipeline_status, PipelineStatus::Pending);
    assert!(pending.action_id.is_none());
    assert_eq!(pending.expected_quantity, Some(Decimal::new(25000, 0)));

    let draft = all
      .iter()
      .find(|r| r.basis_document_number == "TWB-002")
      .unwrap();
    assert_eq!(draft.pipeline_status, PipelineStatus::Draft);
    assert_eq!(draft.action_id, Some(acc2.id));
    assert_eq!(draft.action_document_number.as_deref(), Some("ACC-001"));

    let executed = all
      .iter()
      .find(|r| r.basis_document_number == "TWB-003")
      .unwrap();
    assert_eq!(executed.pipeline_status, PipelineStatus::Executed);
    assert_eq!(executed.action_id, Some(acc3.id));
    assert_eq!(executed.actual_quantity, Some(Decimal::new(24900, 0)));

    // Filter by status
    let pending_only = svc
      .truck_receipt_pipeline_query(Some(PipelineStatus::Pending), None, Some(1), Some(50))
      .await
      .unwrap();
    assert_eq!(pending_only.len(), 1);
    assert_eq!(pending_only[0].basis_document_number, "TWB-001");

    let draft_only = svc
      .truck_receipt_pipeline_query(Some(PipelineStatus::Draft), None, Some(1), Some(50))
      .await
      .unwrap();
    assert_eq!(draft_only.len(), 1);
    assert_eq!(draft_only[0].basis_document_number, "TWB-002");

    let executed_only = svc
      .truck_receipt_pipeline_query(Some(PipelineStatus::Executed), None, Some(1), Some(50))
      .await
      .unwrap();
    assert_eq!(executed_only.len(), 1);
    assert_eq!(executed_only[0].basis_document_number, "TWB-003");

    // Filter by contractor
    let by_sender = svc
      .truck_receipt_pipeline_query(None, Some(fix.sender_id), Some(1), Some(50))
      .await
      .unwrap();
    assert_eq!(by_sender.len(), 3);
    let by_nobody = svc
      .truck_receipt_pipeline_query(None, Some(Uuid::now_v7()), Some(1), Some(50))
      .await
      .unwrap();
    assert_eq!(by_nobody.len(), 0);

    // Pagination
    let page1 = svc
      .truck_receipt_pipeline_query(None, None, Some(1), Some(2))
      .await
      .unwrap();
    assert_eq!(page1.len(), 2);
    let page2 = svc
      .truck_receipt_pipeline_query(None, None, Some(2), Some(2))
      .await
      .unwrap();
    assert_eq!(page2.len(), 1);
  })
  .await;
}

#[tokio::test]
async fn acceptance_flat_query_returns_one_row_per_item() {
  let actor = Uuid::now_v7();
  let origin = Uuid::now_v7();

  with_audit_context(actor, origin, || async {
    let db = Arc::new(setup_db().await);
    let fix = seed_inventory_catalog(&db).await;
    let mut cfg = crate::common::test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let svc = DocumentService::new(db.clone(), ledger, audit);

    // Create acceptance doc with 2 items
    let doc = acceptance_document::ActiveModel {
      document_number: Set("ACC-FLAT-001".into()),
      date_accepted: Set(chrono::Utc::now()),
      status: Set(enums::DocumentStatus::Draft),
      version: Set(1),
      arrival_type: Set(enums::ArrivalType::External),
      source_entity: Set(Some("Test Source".into())),
      contractor_id: Set(fix.contractor_a_id),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let _item1 = acceptance_item::ActiveModel {
      acceptance_doc_id: Set(doc.id),
      product_id: Set(fix.product_a_id),
      storage_id: Set(fix.storage_a_id),
      accepted_amount: Set(Decimal::new(1000, 0)),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    let _item2 = acceptance_item::ActiveModel {
      acceptance_doc_id: Set(doc.id),
      product_id: Set(fix.product_b_id),
      storage_id: Set(fix.storage_b_id),
      accepted_amount: Set(Decimal::new(2000, 0)),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    // Create a second doc with 1 item
    let doc2 = acceptance_document::ActiveModel {
      document_number: Set("ACC-FLAT-002".into()),
      date_accepted: Set(chrono::Utc::now()),
      status: Set(enums::DocumentStatus::Executed),
      version: Set(1),
      arrival_type: Set(enums::ArrivalType::External),
      contractor_id: Set(fix.contractor_a_id),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    acceptance_item::ActiveModel {
      acceptance_doc_id: Set(doc2.id),
      product_id: Set(fix.product_a_id),
      storage_id: Set(fix.storage_a_id),
      accepted_amount: Set(Decimal::new(500, 0)),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    // --- All rows: should get 3 (2 items from doc1 + 1 item from doc2) ---
    let all = svc
      .acceptance_flat_query(None, Some(1), Some(50))
      .await
      .unwrap();
    assert_eq!(all.len(), 3, "Expected 3 flat rows (2 + 1 items)");

    // Check document fields are repeated for items in same group
    let doc1_rows: Vec<_> = all
      .iter()
      .filter(|r| r.document_number == "ACC-FLAT-001")
      .collect();
    assert_eq!(doc1_rows.len(), 2, "Doc1 should have 2 rows (2 items)");
    assert_eq!(
      doc1_rows[0].document_id, doc1_rows[1].document_id,
      "Same document_id for grouped rows"
    );
    assert_eq!(doc1_rows[0].status, enums::DocumentStatus::Draft);

    // Check resolved names
    let has_product_a = doc1_rows.iter().any(|r| r.product_id_name == "Product A");
    let has_product_b = doc1_rows.iter().any(|r| r.product_id_name == "Product B");
    assert!(has_product_a, "Should resolve Product A name");
    assert!(has_product_b, "Should resolve Product B name");

    let has_tank_a = doc1_rows.iter().any(|r| r.storage_id_name == "Tank A");
    let has_tank_b = doc1_rows.iter().any(|r| r.storage_id_name == "Tank B");
    assert!(has_tank_a, "Should resolve Tank A name");
    assert!(has_tank_b, "Should resolve Tank B name");

    // Check doc2 rows
    let doc2_rows: Vec<_> = all
      .iter()
      .filter(|r| r.document_number == "ACC-FLAT-002")
      .collect();
    assert_eq!(doc2_rows.len(), 1, "Doc2 should have 1 row");
    assert_eq!(doc2_rows[0].status, enums::DocumentStatus::Executed);

    // --- Filter by status ---
    let drafts = svc
      .acceptance_flat_query(Some(enums::DocumentStatus::Draft), Some(1), Some(50))
      .await
      .unwrap();
    assert_eq!(drafts.len(), 2, "Only draft doc's 2 items");
    assert!(drafts.iter().all(|r| r.document_number == "ACC-FLAT-001"));

    let posted = svc
      .acceptance_flat_query(Some(enums::DocumentStatus::Executed), Some(1), Some(50))
      .await
      .unwrap();
    assert_eq!(posted.len(), 1, "Only posted doc's 1 item");
    assert_eq!(posted[0].document_number, "ACC-FLAT-002");
  })
  .await;
}
