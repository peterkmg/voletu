use std::{str::FromStr, sync::Arc};

use assert_json_diff::assert_json_eq;
use chrono::NaiveDate;
use sea_orm::{prelude::Decimal, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use voletu_core::{
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
    TruckWaybillItemCompositeRequest,
    TruckWeightDocCompositeRequest,
  },
  entities::{audit_log, product_group, rail_wagon_manifest, rail_waybill, truck_waybill},
  enums,
  services::{
    audit::AuditService,
    catalog::CatalogService,
    document::DocumentService,
    ledger::LedgerService,
  },
};

use crate::common::{fixtures::seed_inventory_fixture, setup_db, test_config};

fn date(value: &str) -> NaiveDate {
  NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap()
}

fn dec(value: &str) -> Decimal {
  Decimal::from_str(value).unwrap()
}

#[tokio::test]
async fn reference_catalog_and_topology_services_create_entities_and_return_them_in_lists() {
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

    assert_eq!(catalog.company_list().await.unwrap().len(), 1);
    assert_eq!(catalog.product_type_list().await.unwrap().len(), 1);
    assert_eq!(catalog.product_group_list().await.unwrap().len(), 1);
    assert_eq!(catalog.product_list().await.unwrap().len(), 1);
    assert_eq!(catalog.base_list().await.unwrap().len(), 1);
    assert_eq!(catalog.warehouse_list().await.unwrap().len(), 1);
    assert_eq!(catalog.storage_list().await.unwrap().len(), 1);
    assert_eq!(catalog.port_list().await.unwrap().len(), 1);
    assert_eq!(storage.product_type_id, Some(ptype.id));
    assert_eq!(product.product_group_id, pgroup.id);
    assert_eq!(port.common_name, "Port A");

    let logs = audit_log::Entity::find().all(&*db).await.unwrap();
    assert!(logs.len() >= 8);

    let company_insert_log = logs
      .iter()
      .find(|row| {
        row.table_name == "companies"
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
async fn transport_services_create_truck_and_rail_documents_and_list_them() {
  with_audit_context(Uuid::now_v7(), Uuid::now_v7(), || async {
    let db = Arc::new(setup_db().await);
    let fixture = seed_inventory_fixture(&db).await;
    let mut cfg = test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let doc_service = DocumentService::new(db.clone(), ledger, audit);

    let truck_waybill_row = doc_service
      .truck_waybill_create(&CreateTruckWaybillRequest {
        document_number: "TW-1".to_string(),
        date: date("2026-01-01"),
        sender_id: fixture.sender_id,
      })
      .await
      .unwrap();
    doc_service
      .truck_waybill_item_create(&CreateTruckWaybillItemRequest {
        truck_waybill_id: truck_waybill_row.id,
        item: TruckWaybillItemCompositeRequest {
          product_id: fixture.product_a_id,
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
        sender_id: fixture.sender_id,
      })
      .await
      .unwrap();
    let manifest = doc_service
      .rail_manifest_create(&CreateRailWagonManifestRequest {
        rail_waybill_id: rail_waybill_row.id,
        manifest: RailWagonManifestCompositeRequest {
          wagon_number: "WAGON-001".to_string(),
          product_id: fixture.product_a_id,
          declared_volume: dec("20.0"),
          declared_density: dec("0.8"),
          declared_mass: dec("16.0"),
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

    assert_eq!(doc_service.truck_waybill_list().await.unwrap().len(), 1);
    assert_eq!(
      doc_service.truck_waybill_item_list().await.unwrap().len(),
      1
    );
    assert_eq!(doc_service.truck_weight_doc_list().await.unwrap().len(), 1);
    assert_eq!(doc_service.rail_waybill_list().await.unwrap().len(), 1);
    assert_eq!(doc_service.rail_manifest_list().await.unwrap().len(), 1);
    assert_eq!(doc_service.rail_measurement_list().await.unwrap().len(), 1);
    assert_eq!(doc_service.rail_weight_list().await.unwrap().len(), 1);

    // Ensure helper data is actually reused by checking records are linked to seeded IDs.
    let saved_truck_waybill = truck_waybill::Entity::find()
      .filter(truck_waybill::Column::DocumentNumber.eq("TW-1"))
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    let saved_rail_waybill = rail_waybill::Entity::find()
      .filter(rail_waybill::Column::DocumentNumber.eq("RW-1"))
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    let saved_manifest = rail_wagon_manifest::Entity::find()
      .filter(rail_wagon_manifest::Column::RailWaybillId.eq(saved_rail_waybill.id))
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    let saved_group = product_group::Entity::find_by_id(fixture.product_group_id)
      .one(&*db)
      .await
      .unwrap();
    assert_eq!(saved_truck_waybill.sender_id, fixture.sender_id);
    assert_eq!(saved_manifest.product_id, fixture.product_a_id);
    assert!(saved_group.is_some());

    let logs = audit_log::Entity::find().all(&*db).await.unwrap();
    assert!(logs.len() >= 7);
  })
  .await;
}
