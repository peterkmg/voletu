use sea_orm::{
  entity::prelude::{DateTimeUtc, Decimal, Uuid},
  sea_query::{
    raw_sql::{
      seaql::{query as raw_query, Query as RawSqlQuery},
      RawSqlQueryBuilder,
    },
    MysqlQueryBuilder,
    PostgresQueryBuilder,
    SqliteQueryBuilder,
  },
  ColumnTrait,
  DbBackend,
  EntityTrait,
  FromQueryResult,
  QueryFilter,
  Statement,
};

use crate::{
  api::ApiError,
  dtos::{CargoFlowFlatRow, CargoFlowPageResponse},
  entities::acceptance_document,
  enums,
  services::{
    common::normalize_pagination,
    document::{specs::CargoFlowQuerySpec, DocumentService},
  },
};

const CARGO_FLOW_UNION_SQL: &str = r#"
SELECT
  item.id AS id,
  doc.id AS document_id,
  doc.document_number AS document_number,
  doc.date_accepted AS date,
  'Incoming' AS flow_type,
  CASE
    WHEN doc.source_entity IS NULL THEN 'Acceptance'
    ELSE 'External Receipt'
  END AS operation,
  contractor.common_name AS contractor_name,
  NULL AS ownership_from_contractor_name,
  NULL AS ownership_to_contractor_name,
  doc.status AS status,
  '/incoming/external' AS flow_route,
  COALESCE(product.common_name, '') AS product_name,
  COALESCE(storage.common_name, '') AS storage_name,
  item.accepted_amount AS quantity,
  NULL AS item_type
FROM acceptance_documents doc
JOIN acceptance_items item ON item.acceptance_doc_id = doc.id
LEFT JOIN companies contractor ON contractor.id = doc.contractor_id
LEFT JOIN products product ON product.id = item.product_id
LEFT JOIN storages storage ON storage.id = item.storage_id

UNION ALL

SELECT
  item.id AS id,
  doc.id AS document_id,
  doc.document_number AS document_number,
  doc.date AS date,
  'Outgoing' AS flow_type,
  CASE
    WHEN doc.dispatch_method = 'Truck' AND doc.dispatch_purpose = 'External' THEN 'Truck Dispatch'
    WHEN doc.dispatch_method = 'Truck' AND doc.dispatch_purpose = 'Internal' THEN 'Internal Truck Dispatch'
    WHEN doc.dispatch_method = 'VesselTerminal' THEN 'Vessel/Terminal Dispatch'
    WHEN doc.dispatch_method = 'Bunkering' THEN 'Bunkering'
    ELSE 'Dispatch'
  END AS operation,
  contractor.common_name AS contractor_name,
  NULL AS ownership_from_contractor_name,
  NULL AS ownership_to_contractor_name,
  doc.status AS status,
  CASE
    WHEN doc.dispatch_method = 'Truck' THEN '/outgoing/truck'
    WHEN doc.dispatch_method = 'VesselTerminal' THEN '/outgoing/direct'
    WHEN doc.dispatch_method = 'Bunkering' THEN '/outgoing/bunkering'
    ELSE '/outgoing/truck'
  END AS flow_route,
  COALESCE(product.common_name, '') AS product_name,
  COALESCE(storage.common_name, '') AS storage_name,
  -item.dispatched_amount AS quantity,
  NULL AS item_type
FROM dispatch_documents doc
JOIN dispatch_items item ON item.dispatch_doc_id = doc.id
LEFT JOIN companies contractor ON contractor.id = doc.contractor_id
LEFT JOIN products product ON product.id = item.product_id
LEFT JOIN storages storage ON storage.id = item.storage_id

UNION ALL

SELECT
  item.id AS id,
  doc.id AS document_id,
  doc.document_number AS document_number,
  doc.date AS date,
  'Internal' AS flow_type,
  'Physical Transfer' AS operation,
  contractor.common_name AS contractor_name,
  NULL AS ownership_from_contractor_name,
  NULL AS ownership_to_contractor_name,
  doc.status AS status,
  '/internal/physical-transfer' AS flow_route,
  COALESCE(product.common_name, '') AS product_name,
  COALESCE(from_storage.common_name, '') AS storage_name,
  -item.amount AS quantity,
  'outflow' AS item_type
FROM physical_storage_transfers doc
JOIN physical_transfer_items item ON item.physical_transfer_id = doc.id
LEFT JOIN companies contractor ON contractor.id = doc.contractor_id
LEFT JOIN products product ON product.id = item.product_id
LEFT JOIN storages from_storage ON from_storage.id = item.from_storage_id

UNION ALL

SELECT
  item.id AS id,
  doc.id AS document_id,
  doc.document_number AS document_number,
  doc.date AS date,
  'Internal' AS flow_type,
  'Physical Transfer' AS operation,
  contractor.common_name AS contractor_name,
  NULL AS ownership_from_contractor_name,
  NULL AS ownership_to_contractor_name,
  doc.status AS status,
  '/internal/physical-transfer' AS flow_route,
  COALESCE(product.common_name, '') AS product_name,
  COALESCE(to_storage.common_name, '') AS storage_name,
  item.amount AS quantity,
  'inflow' AS item_type
FROM physical_storage_transfers doc
JOIN physical_transfer_items item ON item.physical_transfer_id = doc.id
LEFT JOIN companies contractor ON contractor.id = doc.contractor_id
LEFT JOIN products product ON product.id = item.product_id
LEFT JOIN storages to_storage ON to_storage.id = item.to_storage_id

UNION ALL

SELECT
  item.id AS id,
  doc.id AS document_id,
  NULL AS document_number,
  doc.date AS date,
  'Internal' AS flow_type,
  'Ownership Transfer' AS operation,
  NULL AS contractor_name,
  from_contractor.common_name AS ownership_from_contractor_name,
  to_contractor.common_name AS ownership_to_contractor_name,
  doc.status AS status,
  '/internal/ownership-transfer' AS flow_route,
  COALESCE(product.common_name, '') AS product_name,
  COALESCE(storage.common_name, '') AS storage_name,
  item.amount AS quantity,
  NULL AS item_type
FROM ownership_transfers doc
JOIN ownership_transfer_items item ON item.ownership_transfer_id = doc.id
LEFT JOIN companies from_contractor ON from_contractor.id = item.from_contractor_id
LEFT JOIN companies to_contractor ON to_contractor.id = item.to_contractor_id
LEFT JOIN products product ON product.id = item.product_id
LEFT JOIN storages storage ON storage.id = item.storage_id

UNION ALL

SELECT
  component.id AS id,
  doc.id AS document_id,
  doc.document_number AS document_number,
  doc.date AS date,
  'Internal' AS flow_type,
  'Blending' AS operation,
  contractor.common_name AS contractor_name,
  NULL AS ownership_from_contractor_name,
  NULL AS ownership_to_contractor_name,
  doc.status AS status,
  '/internal/blending' AS flow_route,
  COALESCE(product.common_name, '') AS product_name,
  COALESCE(storage.common_name, '') AS storage_name,
  -component.amount_used AS quantity,
  'component' AS item_type
FROM blending_documents doc
JOIN blending_components component ON component.blending_doc_id = doc.id
LEFT JOIN companies contractor ON contractor.id = doc.contractor_id
LEFT JOIN products product ON product.id = component.source_product_id
LEFT JOIN storages storage ON storage.id = component.storage_id

UNION ALL

SELECT
  result.id AS id,
  doc.id AS document_id,
  doc.document_number AS document_number,
  doc.date AS date,
  'Internal' AS flow_type,
  'Blending' AS operation,
  contractor.common_name AS contractor_name,
  NULL AS ownership_from_contractor_name,
  NULL AS ownership_to_contractor_name,
  doc.status AS status,
  '/internal/blending' AS flow_route,
  COALESCE(target_product.common_name, '') AS product_name,
  COALESCE(storage.common_name, '') AS storage_name,
  result.produced_amount AS quantity,
  'result' AS item_type
FROM blending_documents doc
JOIN blending_results result ON result.blending_doc_id = doc.id
LEFT JOIN companies contractor ON contractor.id = doc.contractor_id
LEFT JOIN products target_product ON target_product.id = doc.target_product_id
LEFT JOIN storages storage ON storage.id = result.storage_id

UNION ALL

SELECT
  adjustment.id AS id,
  doc.id AS document_id,
  doc.document_number AS document_number,
  doc.date AS date,
  'Internal' AS flow_type,
  'Reconciliation' AS operation,
  contractor.common_name AS contractor_name,
  NULL AS ownership_from_contractor_name,
  NULL AS ownership_to_contractor_name,
  doc.status AS status,
  '/internal/reconciliation' AS flow_route,
  COALESCE(product.common_name, '') AS product_name,
  COALESCE(storage.common_name, '') AS storage_name,
  CASE
    WHEN adjustment.adjustment_type = 'Loss' THEN -adjustment.amount
    ELSE adjustment.amount
  END AS quantity,
  CASE
    WHEN adjustment.adjustment_type = 'Loss' THEN 'Loss'
    ELSE 'Surplus'
  END AS item_type
FROM inventory_reconciliations doc
JOIN inventory_adjustments adjustment ON adjustment.reconciliation_id = doc.id
LEFT JOIN companies contractor ON contractor.id = doc.contractor_id
LEFT JOIN products product ON product.id = adjustment.product_id
LEFT JOIN storages storage ON storage.id = adjustment.storage_id
"#;

#[derive(Debug, FromQueryResult)]
struct CargoFlowQueryRow {
  id: Uuid,
  document_id: Uuid,
  document_number: Option<String>,
  date: DateTimeUtc,
  flow_type: String,
  operation: String,
  contractor_name: Option<String>,
  ownership_from_contractor_name: Option<String>,
  ownership_to_contractor_name: Option<String>,
  status: enums::DocumentStatus,
  flow_route: String,
  product_name: String,
  storage_name: String,
  quantity: Decimal,
  item_type: Option<String>,
}

#[derive(Debug, FromQueryResult)]
struct CountRow {
  total: i64,
}

impl From<CargoFlowQueryRow> for CargoFlowFlatRow {
  fn from(value: CargoFlowQueryRow) -> Self {
    let contractor_name = value.contractor_name.unwrap_or_else(|| {
      format!(
        "{} → {}",
        value.ownership_from_contractor_name.unwrap_or_default(),
        value.ownership_to_contractor_name.unwrap_or_default()
      )
    });

    Self {
      id: value.id,
      document_id: value.document_id,
      document_number: value
        .document_number
        .unwrap_or_else(|| format!("OT-{}", short_uuid(value.document_id))),
      date: value.date.to_rfc3339(),
      flow_type: value.flow_type,
      operation: value.operation,
      contractor_name,
      status: format!("{:?}", value.status),
      flow_route: value.flow_route,
      product_name: value.product_name,
      storage_name: value.storage_name,
      quantity: value.quantity.to_string(),
      item_type: value.item_type,
    }
  }
}

impl DocumentService {
  pub async fn cargo_flow_flat_query(
    &self,
    query: CargoFlowQuerySpec,
  ) -> Result<CargoFlowPageResponse, ApiError> {
    let (page, per_page) = normalize_pagination(query.page, query.per_page)?;

    let filter = query
      .filter
      .as_deref()
      .map(str::trim)
      .filter(|value| !value.is_empty())
      .map(str::to_owned);

    let total = self.count_cargo_flow_rows(filter.as_deref()).await?;

    let rows = self
      .load_cargo_flow_rows(page, per_page, filter.as_deref())
      .await?;

    Ok(CargoFlowPageResponse {
      items: rows.into_iter().map(CargoFlowFlatRow::from).collect(),
      total,
    })
  }

  async fn count_cargo_flow_rows(&self, filter: Option<&str>) -> Result<u64, ApiError> {
    let statement = cargo_flow_count_statement(self.db.get_database_backend(), filter);

    let row = acceptance_document::Entity::find()
      .filter(acceptance_document::Column::Id.is_not_null())
      .from_raw_sql(statement)
      .into_model::<CountRow>()
      .one(self.db.as_ref())
      .await?
      .unwrap_or(CountRow { total: 0 });

    Ok(row.total.max(0) as u64)
  }

  async fn load_cargo_flow_rows(
    &self,
    page: u64,
    per_page: u64,
    filter: Option<&str>,
  ) -> Result<Vec<CargoFlowQueryRow>, ApiError> {
    let offset = (page - 1) * per_page;
    let statement =
      cargo_flow_load_statement(self.db.get_database_backend(), per_page, offset, filter);

    acceptance_document::Entity::find()
      .filter(acceptance_document::Column::Id.is_not_null())
      .from_raw_sql(statement)
      .into_model::<CargoFlowQueryRow>()
      .all(self.db.as_ref())
      .await
      .map_err(ApiError::from)
  }
}

fn cargo_flow_count_statement(backend: DbBackend, filter: Option<&str>) -> Statement {
  let pattern = filter.map(|value| format!("%{}%", value.to_lowercase()));

  let mut sql = raw_sql_builder(backend);
  sql
    .push_fragment("SELECT COUNT(*) AS total FROM (")
    .push_fragment(CARGO_FLOW_UNION_SQL)
    .push_fragment(") cargo_flow");

  if pattern.is_some() {
    append_cargo_flow_filter_sql(&mut sql);
  }

  let query = if let Some(pattern) = pattern.as_ref() {
    append_cargo_flow_filter(raw_query(&sql.finish()), pattern)
  } else {
    raw_query(&sql.finish())
  };

  raw_statement_from_query(backend, query)
}

fn cargo_flow_load_statement(
  backend: DbBackend,
  per_page: u64,
  offset: u64,
  filter: Option<&str>,
) -> Statement {
  let pattern = filter.map(|value| format!("%{}%", value.to_lowercase()));

  let mut sql = raw_sql_builder(backend);
  sql
    .push_fragment("SELECT * FROM (")
    .push_fragment(CARGO_FLOW_UNION_SQL)
    .push_fragment(") cargo_flow");

  if pattern.is_some() {
    append_cargo_flow_filter_sql(&mut sql);
  }
  sql
    .push_fragment(" ORDER BY date DESC, document_id DESC, id DESC LIMIT ")
    .push_parameters(1)
    .push_fragment(" OFFSET ")
    .push_parameters(1);

  let query = if let Some(pattern) = pattern.as_ref() {
    append_cargo_flow_filter(raw_query(&sql.finish()), pattern)
      .bind(&per_page)
      .bind(&offset)
  } else {
    raw_query(&sql.finish()).bind(&per_page).bind(&offset)
  };

  raw_statement_from_query(backend, query)
}

fn raw_sql_builder(backend: DbBackend) -> RawSqlQueryBuilder {
  match backend {
    DbBackend::Postgres => RawSqlQueryBuilder::new(PostgresQueryBuilder),
    DbBackend::MySql => RawSqlQueryBuilder::new(MysqlQueryBuilder),
    DbBackend::Sqlite => RawSqlQueryBuilder::new(SqliteQueryBuilder),
    _ => RawSqlQueryBuilder::new(SqliteQueryBuilder),
  }
}

fn append_cargo_flow_filter_sql(sql: &mut RawSqlQueryBuilder) {
  sql
    .push_fragment(" WHERE LOWER(COALESCE(document_number, '')) LIKE ")
    .push_parameters(1)
    .push_fragment(" OR LOWER(COALESCE(contractor_name, '')) LIKE ")
    .push_parameters(1)
    .push_fragment(" OR LOWER(COALESCE(ownership_from_contractor_name, '')) LIKE ")
    .push_parameters(1)
    .push_fragment(" OR LOWER(COALESCE(ownership_to_contractor_name, '')) LIKE ")
    .push_parameters(1)
    .push_fragment(" OR LOWER(operation) LIKE ")
    .push_parameters(1)
    .push_fragment(" OR LOWER(product_name) LIKE ")
    .push_parameters(1)
    .push_fragment(" OR LOWER(storage_name) LIKE ")
    .push_parameters(1);
}

fn append_cargo_flow_filter(query: RawSqlQuery, pattern: &String) -> RawSqlQuery {
  let mut query = query;

  for _ in 0..7 {
    query = query.bind(pattern);
  }

  query
}

fn raw_statement_from_query(backend: DbBackend, query: RawSqlQuery) -> Statement {
  let (sql, values) = query.into_parts();
  Statement::from_sql_and_values(backend, sql, values.0)
}

fn short_uuid(id: Uuid) -> String {
  id.to_string().chars().take(8).collect()
}
