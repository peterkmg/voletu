use std::{str::FromStr, sync::Arc};

use chrono::{DateTime, Utc};
use sea_orm::{prelude::Decimal, EntityLoaderTrait};
use uuid::Uuid;
use voletu_core::{
  api::ApiError,
  context::audit::with_audit_context,
  dtos::{
    BlendingComponentCompositeRequest,
    BlendingResultCompositeRequest,
    CreateBlendingComponentRequest,
    CreateBlendingCompositeRequest,
    CreateBlendingRequest,
    CreateBlendingResultRequest,
    UpdateBlendingComponentCompositeRequest,
    UpdateBlendingCompositeRequest,
    UpdateBlendingRequest,
    UpdateBlendingResultCompositeRequest,
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
async fn execution_requires_balanced_component_and_result_totals() {
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
      .balance_by_dimensions(
        catalog.storage_a_id,
        catalog.product_a_id,
        catalog.contractor_a_id,
      )
      .await
      .unwrap()
      .unwrap();
    let dst = ledger
      .balance_by_dimensions(
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
async fn create_fails_without_required_components_and_results() {
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
async fn component_creation_rejects_posted_document_mutation() {
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

#[tokio::test]
async fn composite_update_mutates_components_and_results() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // 1. Seed: create a blending composite with three components and three
    //    results so we can exercise insert / update / delete on both child
    //    collections within a single update call. The document stays in
    //    Draft, so the per-row update / create paths run their normal
    //    draft-only mutation guards.
    let initial = service
      .blending_composite_create(&CreateBlendingCompositeRequest {
        document_number: "BLD-COMP-UPDATE-1".to_string(),
        date: ts("2026-01-02T00:00:00Z"),
        contractor_id: catalog.contractor_a_id,
        target_product_id: catalog.product_b_id,
        components: vec![
          BlendingComponentCompositeRequest {
            storage_id: catalog.storage_a_id,
            source_product_id: catalog.product_a_id,
            amount_used: dec("1.0"),
          },
          BlendingComponentCompositeRequest {
            storage_id: catalog.storage_a_id,
            source_product_id: catalog.product_a_id,
            amount_used: dec("2.0"),
          },
          BlendingComponentCompositeRequest {
            storage_id: catalog.storage_a_id,
            source_product_id: catalog.product_a_id,
            amount_used: dec("3.0"),
          },
        ],
        results: vec![
          BlendingResultCompositeRequest {
            storage_id: catalog.storage_b_id,
            produced_amount: dec("10.0"),
          },
          BlendingResultCompositeRequest {
            storage_id: catalog.storage_b_id,
            produced_amount: dec("20.0"),
          },
          BlendingResultCompositeRequest {
            storage_id: catalog.storage_b_id,
            produced_amount: dec("30.0"),
          },
        ],
      })
      .await
      .unwrap();

    assert_eq!(initial.components.len(), 3);
    assert_eq!(initial.results.len(), 3);

    let blending_id = initial.document.id;

    // Capture each component / result by its initial amount so the test does
    // not depend on a specific row ordering of the response.
    let pick_component = |amount: Decimal| -> (Uuid, Uuid, Uuid, Decimal) {
      let row = initial
        .components
        .iter()
        .find(|c| c.amount_used == amount)
        .unwrap();
      (
        row.id,
        row.storage_id,
        row.source_product_id,
        row.amount_used,
      )
    };
    let (c_unchanged_id, c_unchanged_storage, c_unchanged_product, c_unchanged_amount) =
      pick_component(dec("1.0"));
    let (c_update_id, c_update_storage, c_update_product, _) = pick_component(dec("2.0"));
    let (c_delete_id, _, _, _) = pick_component(dec("3.0"));

    let pick_result = |amount: Decimal| -> (Uuid, Uuid, Decimal) {
      let row = initial
        .results
        .iter()
        .find(|r| r.produced_amount == amount)
        .unwrap();
      (row.id, row.storage_id, row.produced_amount)
    };
    let (r_unchanged_id, r_unchanged_storage, r_unchanged_amount) = pick_result(dec("10.0"));
    let (r_update_id, r_update_storage, _) = pick_result(dec("20.0"));
    let (r_delete_id, _, _) = pick_result(dec("30.0"));

    // 2. Apply a composite update covering both child collections:
    //    - keep the unchanged component / result,
    //    - update one component's amount and one result's produced amount,
    //    - drop one component and one result by omitting them,
    //    - insert one fresh component and one fresh result with id: None.
    let updated = service
      .blending_composite_update(blending_id, &UpdateBlendingCompositeRequest {
        blending: UpdateBlendingRequest {
          document_number: None,
          date: None,
          contractor_id: None,
          target_product_id: None,
        },
        components: vec![
          UpdateBlendingComponentCompositeRequest {
            id: Some(c_unchanged_id),
            storage_id: c_unchanged_storage,
            source_product_id: c_unchanged_product,
            amount_used: c_unchanged_amount,
          },
          UpdateBlendingComponentCompositeRequest {
            id: Some(c_update_id),
            storage_id: c_update_storage,
            source_product_id: c_update_product,
            amount_used: dec("9.5"),
          },
          UpdateBlendingComponentCompositeRequest {
            id: None,
            storage_id: catalog.storage_a_id,
            source_product_id: catalog.product_a_id,
            amount_used: dec("4.25"),
          },
        ],
        results: vec![
          UpdateBlendingResultCompositeRequest {
            id: Some(r_unchanged_id),
            storage_id: r_unchanged_storage,
            produced_amount: r_unchanged_amount,
          },
          UpdateBlendingResultCompositeRequest {
            id: Some(r_update_id),
            storage_id: r_update_storage,
            produced_amount: dec("99.0"),
          },
          UpdateBlendingResultCompositeRequest {
            id: None,
            storage_id: catalog.storage_b_id,
            produced_amount: dec("44.0"),
          },
        ],
      })
      .await
      .unwrap();

    // 3. Component-side assertions on the response.
    assert_eq!(updated.components.len(), 3);

    let unchanged_c = updated
      .components
      .iter()
      .find(|c| c.id == c_unchanged_id)
      .expect("the unchanged component should still be present");
    assert_eq!(unchanged_c.amount_used, dec("1.0"));
    assert_eq!(unchanged_c.storage_id, c_unchanged_storage);

    let modified_c = updated
      .components
      .iter()
      .find(|c| c.id == c_update_id)
      .expect("the updated component should still be present with its original id");
    assert_eq!(modified_c.amount_used, dec("9.5"));

    assert!(
      updated.components.iter().all(|c| c.id != c_delete_id),
      "the omitted component should be hard-deleted from the composite"
    );

    let fresh_c = updated
      .components
      .iter()
      .find(|c| c.id != c_unchanged_id && c.id != c_update_id && c.amount_used == dec("4.25"))
      .expect("the inserted component should appear with a freshly generated id");
    assert_eq!(fresh_c.storage_id, catalog.storage_a_id);
    assert_eq!(fresh_c.source_product_id, catalog.product_a_id);

    // 4. Result-side assertions on the response.
    assert_eq!(updated.results.len(), 3);

    let unchanged_r = updated
      .results
      .iter()
      .find(|r| r.id == r_unchanged_id)
      .expect("the unchanged result should still be present");
    assert_eq!(unchanged_r.produced_amount, dec("10.0"));

    let modified_r = updated
      .results
      .iter()
      .find(|r| r.id == r_update_id)
      .expect("the updated result should still be present with its original id");
    assert_eq!(modified_r.produced_amount, dec("99.0"));

    assert!(
      updated.results.iter().all(|r| r.id != r_delete_id),
      "the omitted result should be hard-deleted from the composite"
    );

    let fresh_r = updated
      .results
      .iter()
      .find(|r| r.id != r_unchanged_id && r.id != r_update_id && r.produced_amount == dec("44.0"))
      .expect("the inserted result should appear with a freshly generated id");
    assert_eq!(fresh_r.storage_id, catalog.storage_b_id);
  })
  .await;
}

#[tokio::test]
async fn composite_update_rejects_duplicate_component_ids() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // Seed: create a blending composite with a single component and a single
    // result so the request below can refer to the existing component id.
    let initial = service
      .blending_composite_create(&CreateBlendingCompositeRequest {
        document_number: "BLD-COMP-UPDATE-DUP".to_string(),
        date: ts("2026-01-02T00:00:00Z"),
        contractor_id: catalog.contractor_a_id,
        target_product_id: catalog.product_b_id,
        components: vec![BlendingComponentCompositeRequest {
          storage_id: catalog.storage_a_id,
          source_product_id: catalog.product_a_id,
          amount_used: dec("1.0"),
        }],
        results: vec![BlendingResultCompositeRequest {
          storage_id: catalog.storage_b_id,
          produced_amount: dec("1.0"),
        }],
      })
      .await
      .unwrap();

    let blending_id = initial.document.id;
    let existing_component = &initial.components[0];
    let existing_result = &initial.results[0];
    let dup_id = existing_component.id;

    // Build a request that references the same existing component id twice.
    let err = service
      .blending_composite_update(blending_id, &UpdateBlendingCompositeRequest {
        blending: UpdateBlendingRequest {
          document_number: None,
          date: None,
          contractor_id: None,
          target_product_id: None,
        },
        components: vec![
          UpdateBlendingComponentCompositeRequest {
            id: Some(dup_id),
            storage_id: existing_component.storage_id,
            source_product_id: existing_component.source_product_id,
            amount_used: dec("1.0"),
          },
          UpdateBlendingComponentCompositeRequest {
            id: Some(dup_id),
            storage_id: existing_component.storage_id,
            source_product_id: existing_component.source_product_id,
            amount_used: dec("2.0"),
          },
        ],
        results: vec![UpdateBlendingResultCompositeRequest {
          id: Some(existing_result.id),
          storage_id: existing_result.storage_id,
          produced_amount: existing_result.produced_amount,
        }],
      })
      .await
      .expect_err("duplicate component ids must be rejected");

    match err {
      ApiError::BadRequest(msg) => {
        assert!(
          msg.contains("duplicate blending component id in request"),
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
