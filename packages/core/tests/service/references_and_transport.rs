use std::{str::FromStr, sync::Arc};

use assert_json_diff::assert_json_eq;
use chrono::NaiveDate;
use sea_orm::{prelude::Decimal, EntityTrait};
use uuid::Uuid;
use voletu_core::{
  api::ApiError,
  context::audit::with_audit_context,
  dtos::{
    CreateBaseRequest,
    CreateCompanyRequest,
    CreatePortRequest,
    CreateProductGroupRequest,
    CreateProductRequest,
    CreateProductTypeRequest,
    CreateRailWagonManifestRequest,
    CreateRailWagonMeasurementRequest,
    CreateRailWagonWeightRequest,
    CreateRailWaybillRequest,
    CreateStorageRequest,
    CreateTruckWaybillItemRequest,
    CreateTruckWaybillRequest,
    CreateTruckWeightDocRequest,
    CreateWarehouseRequest,
    RailWagonManifestCompositeRequest,
    RailWagonMeasurementCompositeRequest,
    RailWagonWeightCompositeRequest,
    RailWaybillCompositeRequest,
    TruckWaybillCompositeRequest,
    TruckWaybillItemCompositeRequest,
    TruckWeightDocCompositeRequest,
    UpdateRailWagonManifestCompositeRequest,
    UpdateRailWagonMeasurementCompositeRequest,
    UpdateRailWagonWeightCompositeRequest,
    UpdateRailWaybillCompositeRequest,
    UpdateRailWaybillRequest,
    UpdateTruckWaybillCompositeRequest,
    UpdateTruckWaybillItemCompositeRequest,
    UpdateTruckWaybillRequest,
  },
  entities::audit_log,
  enums::{self, AuditTable},
  services::{
    audit::AuditService,
    catalog::CatalogService,
    document::DocumentService,
    ledger::LedgerService,
  },
};

use crate::common::{catalog_seed::seed_inventory_catalog, setup_db, test_config};

fn date(value: &str) -> NaiveDate {
  NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap()
}

fn dec(value: &str) -> Decimal {
  Decimal::from_str(value).unwrap()
}

#[tokio::test]
async fn catalog_and_topology_services_return_created_entities_in_lists() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let catalog = CatalogService::new(db.clone(), audit.clone());

    let company = catalog
      .company_create(&CreateCompanyRequest {
        common_name: "Sender Inc".to_string(),
        legal_name: Some("Sender Incorporated".to_string()),
        is_contractor: true,
        is_exporter: false,
        is_manufacturer: true,
        is_sender: true,
      })
      .await
      .unwrap();
    let ptype = catalog
      .product_type_create(&CreateProductTypeRequest {
        common_name: "Fuel".to_string(),
        long_name: Some("Fuel products".to_string()),
      })
      .await
      .unwrap();
    let pgroup = catalog
      .product_group_create(&CreateProductGroupRequest {
        product_type_id: ptype.id,
        common_name: "Diesel".to_string(),
        long_name: None,
      })
      .await
      .unwrap();
    let product = catalog
      .product_create(&CreateProductRequest {
        product_group_id: pgroup.id,
        manufacturer_id: Some(company.id),
        common_name: "Diesel X".to_string(),
        long_name: None,
        add_identification: Some("DX".to_string()),
        is_component: Some(true),
      })
      .await
      .unwrap();

    let base = catalog
      .base_create(&CreateBaseRequest {
        common_name: "Base Alpha".to_string(),
        long_name: None,
      })
      .await
      .unwrap();
    let warehouse = catalog
      .warehouse_create(&CreateWarehouseRequest {
        base_id: base.id,
        common_name: "WH-A".to_string(),
        long_name: None,
      })
      .await
      .unwrap();
    let storage = catalog
      .storage_create(&CreateStorageRequest {
        warehouse_id: warehouse.id,
        common_name: "Tank-1".to_string(),
        long_name: None,
        capacity: Some(dec("1000.0")),
        is_type_specific: Some(true),
        product_type_id: Some(ptype.id),
      })
      .await
      .unwrap();
    let port = catalog
      .port_create(&CreatePortRequest {
        common_name: "Port A".to_string(),
        country: Some("EE".to_string()),
      })
      .await
      .unwrap();

    assert_eq!(catalog.company_list(None).await.unwrap().len(), 1);
    assert_eq!(catalog.product_type_list(None).await.unwrap().len(), 1);
    assert_eq!(catalog.product_group_list(None).await.unwrap().len(), 1);
    assert_eq!(catalog.product_list(None).await.unwrap().len(), 1);
    assert_eq!(catalog.base_list(None).await.unwrap().len(), 1);
    assert_eq!(catalog.warehouse_list(None).await.unwrap().len(), 1);
    assert_eq!(catalog.storage_list(None).await.unwrap().len(), 1);
    assert_eq!(catalog.port_list(None).await.unwrap().len(), 1);
    assert_eq!(storage.product_type_id, Some(ptype.id));
    assert_eq!(product.product_group_id, pgroup.id);
    assert_eq!(port.common_name, "Port A");

    let logs: Vec<audit_log::ModelEx> = audit_log::Entity::load().all(&*db).await.unwrap();
    assert!(logs.len() >= 8);

    let company_insert_log = logs
      .iter()
      .find(|row| {
        row.table_name == AuditTable::Companies
          && row.record_id == company.id
          && row.action == enums::AuditAction::Insert
      })
      .unwrap();
    let company_model = voletu_core::entities::company::Entity::find_by_id(company.id)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    let expected_company_snapshot = serde_json::to_value(&company_model).unwrap();
    assert_json_eq!(
      company_insert_log.new_values.as_ref().unwrap(),
      &expected_company_snapshot
    );
  })
  .await;
}

#[tokio::test]
async fn creates_truck_and_rail_documents_returned_in_list() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let doc_service = DocumentService::new(db.clone(), ledger, audit);

    let truck_waybill_row = doc_service
      .truck_waybill_create(&CreateTruckWaybillRequest {
        document_number: "TW-1".to_string(),
        date: date("2026-01-01"),
        sender_id: catalog.sender_id,
        base_id: catalog.base_id,
      })
      .await
      .unwrap();
    doc_service
      .truck_waybill_item_create(&CreateTruckWaybillItemRequest {
        truck_waybill_id: truck_waybill_row.id,
        item: TruckWaybillItemCompositeRequest {
          product_id: catalog.product_a_id,
          declared_amount: dec("12.5"),
        },
      })
      .await
      .unwrap();
    doc_service
      .truck_weight_doc_create(&CreateTruckWeightDocRequest {
        truck_waybill_id: truck_waybill_row.id,
        weight_doc: TruckWeightDocCompositeRequest {
          total_weight: dec("13.0"),
        },
      })
      .await
      .unwrap();

    let rail_waybill_row = doc_service
      .rail_waybill_create(&CreateRailWaybillRequest {
        document_number: "RW-1".to_string(),
        date: date("2026-01-01"),
        sender_id: catalog.sender_id,
        base_id: catalog.base_id,
      })
      .await
      .unwrap();
    let manifest = doc_service
      .rail_manifest_create(&CreateRailWagonManifestRequest {
        rail_waybill_id: rail_waybill_row.id,
        manifest: RailWagonManifestCompositeRequest {
          wagon_number: "WAGON-001".to_string(),
          product_id: catalog.product_a_id,
          declared_volume: dec("20.0"),
          declared_density: dec("0.8"),
          declared_mass: dec("16.0"),
          measurements: None,
          weights: None,
        },
      })
      .await
      .unwrap();
    doc_service
      .rail_measurement_create(&CreateRailWagonMeasurementRequest {
        wagon_manifest_id: manifest.id,
        measured_height: dec("2.0"),
        lab_density: Some(dec("0.79")),
        calculated_mass: dec("15.8"),
      })
      .await
      .unwrap();
    doc_service
      .rail_weight_create(&CreateRailWagonWeightRequest {
        wagon_manifest_id: manifest.id,
        gross_weight: dec("40.0"),
        tare_weight: dec("20.0"),
        net_product_weight: dec("20.0"),
      })
      .await
      .unwrap();

    assert_eq!(doc_service.truck_waybill_list(None).await.unwrap().len(), 1);
    assert_eq!(
      doc_service
        .truck_waybill_item_list(None)
        .await
        .unwrap()
        .len(),
      1
    );
    assert_eq!(
      doc_service.truck_weight_doc_list(None).await.unwrap().len(),
      1
    );
    assert_eq!(doc_service.rail_waybill_list(None).await.unwrap().len(), 1);
    assert_eq!(doc_service.rail_manifest_list(None).await.unwrap().len(), 1);
    assert_eq!(
      doc_service.rail_measurement_list(None).await.unwrap().len(),
      1
    );
    assert_eq!(doc_service.rail_weight_list(None).await.unwrap().len(), 1);

    // The create helpers should preserve the seeded linkage without extra lookup hops.
    assert_eq!(truck_waybill_row.sender_id, catalog.sender_id);
    assert_eq!(truck_waybill_row.base_id, catalog.base_id);
    assert_eq!(rail_waybill_row.sender_id, catalog.sender_id);
    assert_eq!(manifest.rail_waybill_id, rail_waybill_row.id);
    assert_eq!(manifest.product_id, catalog.product_a_id);

    let logs: Vec<audit_log::ModelEx> = audit_log::Entity::load().all(&*db).await.unwrap();
    assert!(logs.len() >= 7);
  })
  .await;
}

#[tokio::test]
async fn truck_waybill_composite_update_inserts_updates_and_deletes_items() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // 1. Seed: create a truck waybill composite with three items.
    let initial = service
      .truck_waybill_composite_create(&TruckWaybillCompositeRequest {
        document_number: "TW-COMP-UPDATE-1".to_string(),
        date: date("2026-01-01"),
        sender_id: catalog.sender_id,
        base_id: catalog.base_id,
        items: Some(vec![
          TruckWaybillItemCompositeRequest {
            product_id: catalog.product_a_id,
            declared_amount: dec("1.0"),
          },
          TruckWaybillItemCompositeRequest {
            product_id: catalog.product_a_id,
            declared_amount: dec("2.0"),
          },
          TruckWaybillItemCompositeRequest {
            product_id: catalog.product_a_id,
            declared_amount: dec("3.0"),
          },
        ]),
        weight_docs: None,
      })
      .await
      .unwrap();

    let initial_items = initial.items.as_ref().expect("items present");
    assert_eq!(initial_items.len(), 3);

    let waybill_id = initial.waybill.id;
    // Capture each id by initial declared_amount so the test does not depend
    // on a specific row ordering of the response.
    let pick = |amount: Decimal| -> (Uuid, Uuid, Decimal) {
      let item = initial_items
        .iter()
        .find(|item| item.declared_amount == amount)
        .unwrap();
      (item.id, item.product_id, item.declared_amount)
    };
    let (unchanged_id, unchanged_product, unchanged_amount) = pick(dec("1.0"));
    let (update_id, update_product, _) = pick(dec("2.0"));
    let (delete_id, _, _) = pick(dec("3.0"));

    // 2. Apply a composite update:
    //    - keep item_unchanged as-is,
    //    - update item_to_update.declared_amount,
    //    - drop item_to_delete by omitting it,
    //    - insert one fresh item with id: None.
    let updated = service
      .truck_waybill_composite_update(waybill_id, &UpdateTruckWaybillCompositeRequest {
        waybill: UpdateTruckWaybillRequest {
          document_number: None,
          date: None,
          sender_id: None,
          base_id: None,
        },
        items: vec![
          UpdateTruckWaybillItemCompositeRequest {
            id: Some(unchanged_id),
            product_id: unchanged_product,
            declared_amount: unchanged_amount,
          },
          UpdateTruckWaybillItemCompositeRequest {
            id: Some(update_id),
            product_id: update_product,
            declared_amount: dec("9.5"),
          },
          UpdateTruckWaybillItemCompositeRequest {
            id: None,
            product_id: catalog.product_a_id,
            declared_amount: dec("4.25"),
          },
        ],
      })
      .await
      .unwrap();

    // 3. Assertions on the response.
    let updated_items = updated.items.as_ref().expect("items present after update");
    assert_eq!(updated_items.len(), 3);

    let unchanged = updated_items
      .iter()
      .find(|item| item.id == unchanged_id)
      .expect("the unchanged item should still be present");
    assert_eq!(unchanged.declared_amount, dec("1.0"));

    let modified = updated_items
      .iter()
      .find(|item| item.id == update_id)
      .expect("the updated item should still be present with its original id");
    assert_eq!(modified.declared_amount, dec("9.5"));

    assert!(
      updated_items.iter().all(|item| item.id != delete_id),
      "the omitted item should be hard-deleted from the composite"
    );

    let fresh = updated_items
      .iter()
      .find(|item| {
        item.id != unchanged_id && item.id != update_id && item.declared_amount == dec("4.25")
      })
      .expect("the inserted item should appear with a freshly generated id");
    assert_eq!(fresh.product_id, catalog.product_a_id);
  })
  .await;
}

#[tokio::test]
async fn truck_waybill_composite_update_rejects_duplicate_item_ids() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // Seed: create a truck waybill composite with a single item.
    let initial = service
      .truck_waybill_composite_create(&TruckWaybillCompositeRequest {
        document_number: "TW-COMP-UPDATE-DUP".to_string(),
        date: date("2026-01-01"),
        sender_id: catalog.sender_id,
        base_id: catalog.base_id,
        items: Some(vec![TruckWaybillItemCompositeRequest {
          product_id: catalog.product_a_id,
          declared_amount: dec("1.0"),
        }]),
        weight_docs: None,
      })
      .await
      .unwrap();

    let waybill_id = initial.waybill.id;
    let initial_items = initial.items.as_ref().expect("items present");
    let existing = &initial_items[0];
    let dup_id = existing.id;

    // Build a request that references the same existing id twice.
    let err = service
      .truck_waybill_composite_update(waybill_id, &UpdateTruckWaybillCompositeRequest {
        waybill: UpdateTruckWaybillRequest {
          document_number: None,
          date: None,
          sender_id: None,
          base_id: None,
        },
        items: vec![
          UpdateTruckWaybillItemCompositeRequest {
            id: Some(dup_id),
            product_id: existing.product_id,
            declared_amount: dec("1.0"),
          },
          UpdateTruckWaybillItemCompositeRequest {
            id: Some(dup_id),
            product_id: existing.product_id,
            declared_amount: dec("2.0"),
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

/// Cover the recursive composite update for rail waybills.
///
/// The seed creates three manifests:
///   - manifest A: with 1 measurement + 1 weight (will be left UNCHANGED at the manifest level)
///   - manifest B: with 1 weight, 0 measurements (will UPDATE its declared_mass; ADD a measurement)
///   - manifest C: with 1 measurement + 1 weight (will be DELETED entirely; children must go too)
///
/// The update payload also INSERTS a brand-new manifest D with one measurement
/// and one weight, exercising the create-with-children branch.
///
/// Note: rail measurement / weight rows enforce a `UNIQUE(wagon_manifest_id)`
/// constraint at the entity level, so each manifest holds at most one of each.
/// This keeps the nested-diff scenario realistic for the actual schema while
/// still exercising insert / update / delete / unchanged at both levels.
#[tokio::test]
async fn rail_waybill_composite_update_inserts_updates_and_deletes_nested_children() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let service = DocumentService::new(db.clone(), ledger, audit);

    // 1. Seed: create the rail waybill composite with three manifests.
    let initial = service
      .rail_waybill_composite_create(&RailWaybillCompositeRequest {
        document_number: "RW-COMP-UPDATE-1".to_string(),
        date: date("2026-01-01"),
        sender_id: catalog.sender_id,
        base_id: catalog.base_id,
        manifests: Some(vec![
          // Manifest A - kept unchanged at every level.
          RailWagonManifestCompositeRequest {
            wagon_number: "WAG-A".to_string(),
            product_id: catalog.product_a_id,
            declared_volume: dec("10.0"),
            declared_density: dec("0.85"),
            declared_mass: dec("8.5"),
            measurements: Some(vec![RailWagonMeasurementCompositeRequest {
              wagon_number: "WAG-A".to_string(),
              measured_height: dec("1.20"),
              lab_density: Some(dec("0.85")),
              calculated_mass: dec("8.4"),
            }]),
            weights: Some(vec![RailWagonWeightCompositeRequest {
              wagon_number: "WAG-A".to_string(),
              gross_weight: dec("12.0"),
              tare_weight: dec("3.5"),
              net_product_weight: dec("8.5"),
            }]),
          },
          // Manifest B - will have its declared_mass updated and gain a measurement.
          RailWagonManifestCompositeRequest {
            wagon_number: "WAG-B".to_string(),
            product_id: catalog.product_a_id,
            declared_volume: dec("12.0"),
            declared_density: dec("0.84"),
            declared_mass: dec("10.0"),
            measurements: None,
            weights: Some(vec![RailWagonWeightCompositeRequest {
              wagon_number: "WAG-B".to_string(),
              gross_weight: dec("13.0"),
              tare_weight: dec("3.0"),
              net_product_weight: dec("10.0"),
            }]),
          },
          // Manifest C - will be deleted entirely with both children.
          RailWagonManifestCompositeRequest {
            wagon_number: "WAG-C".to_string(),
            product_id: catalog.product_a_id,
            declared_volume: dec("11.0"),
            declared_density: dec("0.86"),
            declared_mass: dec("9.46"),
            measurements: Some(vec![RailWagonMeasurementCompositeRequest {
              wagon_number: "WAG-C".to_string(),
              measured_height: dec("1.10"),
              lab_density: Some(dec("0.86")),
              calculated_mass: dec("9.4"),
            }]),
            weights: Some(vec![RailWagonWeightCompositeRequest {
              wagon_number: "WAG-C".to_string(),
              gross_weight: dec("14.0"),
              tare_weight: dec("4.5"),
              net_product_weight: dec("9.5"),
            }]),
          },
        ]),
      })
      .await
      .unwrap();

    let waybill_id = initial.waybill.id;
    let initial_manifests = initial.wagon_manifests.as_ref().expect("manifests present");
    assert_eq!(initial_manifests.len(), 3);

    let pick_manifest = |wagon: &str| -> _ {
      initial_manifests
        .iter()
        .find(|m| m.wagon_number == wagon)
        .unwrap_or_else(|| panic!("manifest {wagon} missing in seed response"))
    };
    let manifest_a = pick_manifest("WAG-A");
    let manifest_b = pick_manifest("WAG-B");
    let manifest_c = pick_manifest("WAG-C");

    // Capture child ids for the update payload (round-tripped so the
    // backend treats them as updates, not deletes-and-inserts).
    let manifest_a_measurement_id = manifest_a.measurements.as_ref().unwrap()[0].id;
    let manifest_a_weight_id = manifest_a.weights.as_ref().unwrap()[0].id;
    let manifest_b_weight_id = manifest_b.weights.as_ref().unwrap()[0].id;

    // 2. Apply the composite update:
    //    - Manifest A: unchanged at every level (round-trip ids and values).
    //    - Manifest B: scalar update (declared_mass) + INSERT one measurement
    //      (id: None) + keep its weight unchanged (id round-tripped).
    //    - Manifest C: omitted entirely => delete manifest + cascade children.
    //    - Manifest D: brand-new manifest with one measurement and one weight.
    let updated = service
      .rail_waybill_composite_update(waybill_id, &UpdateRailWaybillCompositeRequest {
        waybill: UpdateRailWaybillRequest {
          document_number: None,
          date: None,
          sender_id: None,
          base_id: None,
        },
        manifests: vec![
          UpdateRailWagonManifestCompositeRequest {
            id: Some(manifest_a.id),
            wagon_number: manifest_a.wagon_number.clone(),
            product_id: manifest_a.product_id,
            declared_volume: manifest_a.declared_volume,
            declared_density: manifest_a.declared_density,
            declared_mass: manifest_a.declared_mass,
            measurements: vec![UpdateRailWagonMeasurementCompositeRequest {
              id: Some(manifest_a_measurement_id),
              measured_height: dec("1.20"),
              lab_density: Some(dec("0.85")),
              calculated_mass: dec("8.4"),
            }],
            weights: vec![UpdateRailWagonWeightCompositeRequest {
              id: Some(manifest_a_weight_id),
              gross_weight: dec("12.0"),
              tare_weight: dec("3.5"),
              net_product_weight: dec("8.5"),
            }],
          },
          UpdateRailWagonManifestCompositeRequest {
            id: Some(manifest_b.id),
            wagon_number: manifest_b.wagon_number.clone(),
            product_id: manifest_b.product_id,
            declared_volume: manifest_b.declared_volume,
            declared_density: manifest_b.declared_density,
            declared_mass: dec("11.5"),
            measurements: vec![UpdateRailWagonMeasurementCompositeRequest {
              id: None,
              measured_height: dec("1.30"),
              lab_density: Some(dec("0.84")),
              calculated_mass: dec("11.3"),
            }],
            weights: vec![UpdateRailWagonWeightCompositeRequest {
              id: Some(manifest_b_weight_id),
              gross_weight: dec("13.0"),
              tare_weight: dec("3.0"),
              net_product_weight: dec("10.0"),
            }],
          },
          UpdateRailWagonManifestCompositeRequest {
            id: None,
            wagon_number: "WAG-D".to_string(),
            product_id: catalog.product_a_id,
            declared_volume: dec("9.0"),
            declared_density: dec("0.83"),
            declared_mass: dec("7.47"),
            measurements: vec![UpdateRailWagonMeasurementCompositeRequest {
              id: None,
              measured_height: dec("0.95"),
              lab_density: None,
              calculated_mass: dec("7.4"),
            }],
            weights: vec![UpdateRailWagonWeightCompositeRequest {
              id: None,
              gross_weight: dec("11.0"),
              tare_weight: dec("3.6"),
              net_product_weight: dec("7.4"),
            }],
          },
        ],
      })
      .await
      .unwrap();

    // 3. Assertions.
    let updated_manifests = updated
      .wagon_manifests
      .as_ref()
      .expect("manifests present after update");
    assert_eq!(updated_manifests.len(), 3, "manifest C should be deleted");

    // Manifest C must be gone.
    assert!(
      updated_manifests.iter().all(|m| m.id != manifest_c.id),
      "manifest C should be hard-deleted from the composite"
    );

    // Manifest A: id preserved, scalar values unchanged, child ids preserved.
    let returned_a = updated_manifests
      .iter()
      .find(|m| m.id == manifest_a.id)
      .expect("manifest A should still be present");
    assert_eq!(returned_a.declared_mass, dec("8.5"));
    assert_eq!(
      returned_a.measurements.as_ref().unwrap()[0].id,
      manifest_a_measurement_id
    );
    assert_eq!(
      returned_a.weights.as_ref().unwrap()[0].id,
      manifest_a_weight_id
    );

    // Manifest B: id preserved, scalar update applied, weight id preserved,
    // measurement INSERTED with a fresh id.
    let returned_b = updated_manifests
      .iter()
      .find(|m| m.id == manifest_b.id)
      .expect("manifest B should still be present");
    assert_eq!(returned_b.declared_mass, dec("11.5"));
    assert_eq!(
      returned_b.weights.as_ref().unwrap()[0].id,
      manifest_b_weight_id
    );
    let b_measurements = returned_b
      .measurements
      .as_ref()
      .expect("manifest B should now have a measurement");
    assert_eq!(b_measurements.len(), 1);
    assert_eq!(b_measurements[0].calculated_mass, dec("11.3"));

    // Manifest D: brand-new id, both children present with fresh ids.
    let returned_d = updated_manifests
      .iter()
      .find(|m| m.wagon_number == "WAG-D")
      .expect("manifest D should be inserted");
    assert!(returned_d.id != manifest_a.id && returned_d.id != manifest_b.id);
    let d_measurements = returned_d
      .measurements
      .as_ref()
      .expect("manifest D should have its measurement persisted");
    assert_eq!(d_measurements.len(), 1);
    assert_eq!(d_measurements[0].measured_height, dec("0.95"));
    let d_weights = returned_d
      .weights
      .as_ref()
      .expect("manifest D should have its weight persisted");
    assert_eq!(d_weights.len(), 1);
    assert_eq!(d_weights[0].net_product_weight, dec("7.4"));
  })
  .await;
}

/// Composite update must reject duplicate manifest ids before any write,
/// matching the truck-waybill guard.
#[tokio::test]
async fn rail_waybill_composite_update_rejects_duplicate_manifest_ids() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let catalog = seed_inventory_catalog(&db).await;
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let service = DocumentService::new(db.clone(), ledger, audit);

    let initial = service
      .rail_waybill_composite_create(&RailWaybillCompositeRequest {
        document_number: "RW-COMP-UPDATE-DUP".to_string(),
        date: date("2026-01-01"),
        sender_id: catalog.sender_id,
        base_id: catalog.base_id,
        manifests: Some(vec![RailWagonManifestCompositeRequest {
          wagon_number: "WAG-DUP".to_string(),
          product_id: catalog.product_a_id,
          declared_volume: dec("10.0"),
          declared_density: dec("0.85"),
          declared_mass: dec("8.5"),
          measurements: None,
          weights: None,
        }]),
      })
      .await
      .unwrap();

    let waybill_id = initial.waybill.id;
    let manifest = &initial.wagon_manifests.as_ref().unwrap()[0];
    let dup_id = manifest.id;

    let err = service
      .rail_waybill_composite_update(waybill_id, &UpdateRailWaybillCompositeRequest {
        waybill: UpdateRailWaybillRequest {
          document_number: None,
          date: None,
          sender_id: None,
          base_id: None,
        },
        manifests: vec![
          UpdateRailWagonManifestCompositeRequest {
            id: Some(dup_id),
            wagon_number: manifest.wagon_number.clone(),
            product_id: manifest.product_id,
            declared_volume: manifest.declared_volume,
            declared_density: manifest.declared_density,
            declared_mass: manifest.declared_mass,
            measurements: vec![],
            weights: vec![],
          },
          UpdateRailWagonManifestCompositeRequest {
            id: Some(dup_id),
            wagon_number: manifest.wagon_number.clone(),
            product_id: manifest.product_id,
            declared_volume: manifest.declared_volume,
            declared_density: manifest.declared_density,
            declared_mass: dec("9.9"),
            measurements: vec![],
            weights: vec![],
          },
        ],
      })
      .await
      .expect_err("duplicate manifest ids must be rejected");

    match err {
      ApiError::BadRequest(msg) => {
        assert!(
          msg.contains("duplicate manifest id in request"),
          "expected duplicate-manifest-id error, got: {msg}"
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
