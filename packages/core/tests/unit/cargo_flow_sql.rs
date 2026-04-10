use std::{collections::BTreeMap, sync::Arc};

use chrono::{TimeZone, Utc};
use sea_orm::{entity::prelude::Decimal, DatabaseBackend, MockDatabase, Value};
use uuid::Uuid;
use voletu_core::{
  enums,
  services::{AuditService, DocumentService, LedgerService},
};

fn cargo_flow_count_row(total: i64) -> BTreeMap<String, Value> {
  BTreeMap::from([("total".to_string(), total.into())])
}

fn cargo_flow_data_row() -> BTreeMap<String, Value> {
  BTreeMap::from([
    ("id".to_string(), Uuid::now_v7().into()),
    ("document_id".to_string(), Uuid::now_v7().into()),
    (
      "document_number".to_string(),
      Some("CF-1".to_string()).into(),
    ),
    (
      "date".to_string(),
      Utc.with_ymd_and_hms(2026, 4, 10, 12, 0, 0).unwrap().into(),
    ),
    ("flow_type".to_string(), "Incoming".into()),
    ("operation".to_string(), "Acceptance".into()),
    (
      "contractor_name".to_string(),
      Some("Contoso".to_string()).into(),
    ),
    (
      "ownership_from_contractor_name".to_string(),
      Option::<String>::None.into(),
    ),
    (
      "ownership_to_contractor_name".to_string(),
      Option::<String>::None.into(),
    ),
    ("status".to_string(), enums::DocumentStatus::Draft.into()),
    ("flow_route".to_string(), "/incoming/external".into()),
    ("product_name".to_string(), "Diesel".into()),
    ("storage_name".to_string(), "Tank A".into()),
    ("quantity".to_string(), Decimal::new(10, 0).into()),
    ("item_type".to_string(), Option::<String>::None.into()),
  ])
}

#[tokio::test]
async fn cargo_flow_queries_use_backend_specific_placeholders() {
  for backend in [
    DatabaseBackend::Sqlite,
    DatabaseBackend::Postgres,
    DatabaseBackend::MySql,
  ] {
    let db = Arc::new(
      MockDatabase::new(backend)
        .append_query_results([[cargo_flow_count_row(1)]])
        .append_query_results([[cargo_flow_data_row()]])
        .into_connection(),
    );

    let mut cfg = crate::common::test_config();
    cfg.node.db_id = Uuid::now_v7();
    let audit = Arc::new(AuditService::new(Arc::new(cfg)));
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let service = DocumentService::new(db.clone(), ledger.clone(), audit.clone());

    let page = service
      .cargo_flow_flat_query(voletu_core::services::document::specs::CargoFlowQuerySpec {
        page: Some(3),
        per_page: Some(10),
        filter: Some("diesel".to_string()),
      })
      .await
      .unwrap();

    assert_eq!(page.total, 1);
    assert_eq!(page.items.len(), 1);

    drop(service);
    drop(ledger);
    drop(audit);

    let log = Arc::try_unwrap(db).unwrap().into_transaction_log();
    let count_sql = &log[0].statements()[0].sql;
    let page_sql = &log[1].statements()[0].sql;

    match backend {
      DatabaseBackend::Postgres => {
        assert!(count_sql.contains("$1"));
        assert!(!count_sql.contains('?'));
        assert!(page_sql.contains("$8"));
        assert!(page_sql.contains("$9"));
      }
      DatabaseBackend::Sqlite | DatabaseBackend::MySql => {
        assert!(count_sql.contains('?'));
        assert!(page_sql.contains('?'));
      }
      _ => unreachable!(),
    }
  }
}
