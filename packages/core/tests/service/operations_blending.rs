use std::{str::FromStr, sync::Arc};

use chrono::{DateTime, Utc};
use sea_orm::{prelude::Decimal, EntityLoaderTrait};
use uuid::Uuid;
use voletu_core::{
  context::audit::with_audit_context,
  dtos::{
    BlendingComponentCompositeRequest,
    BlendingResultCompositeRequest,
    CreateBlendingComponentRequest,
    CreateBlendingRequest,
    CreateBlendingResultRequest,
  },
  entities::blending_document,
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
async fn blending_execution_requires_balanced_component_and_result_totals() {
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
      Decimal::from(50),
    )
    .await;

    let bad_doc = service
      .blending_document_create(&CreateBlendingRequest {
        document_number: "BLD-BAD".to_string(),
        date: ts("2026-01-02T00:00:00Z"),
        contractor_id: catalog.contractor_a_id,
        target_product_id: catalog.product_b_id,
      })
      .await
      .unwrap();
    service
      .blending_component_create(&CreateBlendingComponentRequest {
        blending_doc_id: bad_doc.id,
        component: BlendingComponentCompositeRequest {
          storage_id: catalog.storage_a_id,
          source_product_id: catalog.product_a_id,
          amount_used: dec("20.0"),
        },
      })
      .await
      .unwrap();
    service
      .blending_result_create(&CreateBlendingResultRequest {
        blending_doc_id: bad_doc.id,
        result: BlendingResultCompositeRequest {
          storage_id: catalog.storage_b_id,
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
        contractor_id: catalog.contractor_a_id,
        target_product_id: catalog.product_b_id,
      })
      .await
      .unwrap();
    service
      .blending_component_create(&CreateBlendingComponentRequest {
        blending_doc_id: good_doc.id,
        component: BlendingComponentCompositeRequest {
          storage_id: catalog.storage_a_id,
          source_product_id: catalog.product_a_id,
          amount_used: dec("15.0"),
        },
      })
      .await
      .unwrap();
    service
      .blending_result_create(&CreateBlendingResultRequest {
        blending_doc_id: good_doc.id,
        result: BlendingResultCompositeRequest {
          storage_id: catalog.storage_b_id,
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
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let dst = ledger
      .by_dimensions(
        catalog.storage_b_id,
        catalog.product_b_id,
        catalog.contractor_a_id,
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
    let catalog = seed_inventory_catalog(&db).await;
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
          contractor_id: catalog.contractor_a_id,
          target_product_id: catalog.product_b_id,
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
async fn blending_component_creation_rejects_posted_document_mutation() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    seed_ledger_balance(
      &db,
      catalog.storage_a_id,
      catalog.product_a_id,
      catalog.contractor_a_id,
      Decimal::from(20),
    )
    .await;

    let doc = service
      .blending_document_create(&CreateBlendingRequest {
        document_number: "BLD-MUT".to_string(),
        date: ts("2026-01-06T00:00:00Z"),
        contractor_id: catalog.contractor_a_id,
        target_product_id: catalog.product_b_id,
      })
      .await
      .unwrap();

    service
      .blending_component_create(&CreateBlendingComponentRequest {
        blending_doc_id: doc.id,
        component: BlendingComponentCompositeRequest {
          storage_id: catalog.storage_a_id,
          source_product_id: catalog.product_a_id,
          amount_used: dec("10.0"),
        },
      })
      .await
      .unwrap();

    service
      .blending_result_create(&CreateBlendingResultRequest {
        blending_doc_id: doc.id,
        result: BlendingResultCompositeRequest {
          storage_id: catalog.storage_b_id,
          produced_amount: dec("10.0"),
        },
      })
      .await
      .unwrap();

    service
      .blending_document_execute(doc.id, Uuid::now_v7())
      .await
      .unwrap();

    let posted = blending_document::Entity::load()
      .filter_by_id(doc.id)
      .one(db.as_ref())
      .await
      .unwrap()
      .unwrap();
    assert_eq!(posted.status, DocumentStatus::Executed);

    let err = service
      .blending_component_create(&CreateBlendingComponentRequest {
        blending_doc_id: doc.id,
        component: BlendingComponentCompositeRequest {
          storage_id: catalog.storage_a_id,
          source_product_id: catalog.product_a_id,
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
