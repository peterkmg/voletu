use std::collections::{HashMap, HashSet};

use sea_orm::{
  ColumnTrait, Condition, EntityLoaderTrait, QueryFilter, QueryOrder, TransactionTrait,
};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::{
    response::pipeline::OwnershipTransferFlatRow, CreateOwnershipTransferItemRequest,
    CreateOwnershipTransferRequest, OwnershipTransferResponse,
  },
  entities::{company, ownership_transfer, ownership_transfer_item, product, storage},
  services::document::{
    query::{OwnershipTransferFlatQuerySpec, OwnershipTransferQuerySpec},
    DocumentService,
  },
};

impl DocumentService {
  pub async fn ownership_transfer_composite_create(
    &self,
    req: &CreateOwnershipTransferRequest,
  ) -> Result<OwnershipTransferResponse, ApiError> {
    tracing::info!("Creating ownership transfer");
    let txn = self.db.begin().await?;
    let response = self
      .ownership_transfer_composite_create_no_tx(&txn, req)
      .await?;

    txn.commit().await?;

    Ok(response)
  }

  pub async fn ownership_transfer_composite_create_and_execute(
    &self,
    req: &CreateOwnershipTransferRequest,
    actor_id: Uuid,
  ) -> Result<OwnershipTransferResponse, ApiError> {
    let txn = self.db.begin().await?;

    let mut response = self
      .ownership_transfer_composite_create_no_tx(&txn, req)
      .await?;

    self
      .ownership_transfer_execute_no_tx(&txn, response.id, actor_id)
      .await?;

    response.status = crate::enums::DocumentStatus::Executed;

    txn.commit().await?;

    Ok(response)
  }

  pub(crate) async fn ownership_transfer_composite_create_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    req: &CreateOwnershipTransferRequest,
  ) -> Result<OwnershipTransferResponse, ApiError> {
    let mut response = self.ownership_transfer_create_no_tx(conn, req).await?;

    for item_req in &req.items {
      response.items.push(
        self
          .ownership_item_create_no_tx(
            conn,
            &CreateOwnershipTransferItemRequest::from_composite(response.id, item_req),
          )
          .await?,
      );
    }

    self
      .audit
      .backfill_document_routing(conn, "ownership_transfers", response.id)
      .await?;

    Ok(response)
  }

  pub(super) async fn ownership_transfer_model(
    &self,
    id: Uuid,
  ) -> Result<ownership_transfer::ModelEx, ApiError> {
    ownership_transfer::Entity::load()
      .filter_by_id(id)
      .filter(ownership_transfer::Column::DeletedAt.is_null())
      .with((ownership_transfer_item::Entity, product::Entity))
      .with((ownership_transfer_item::Entity, storage::Entity))
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Ownership transfer '{}' not found", id)))
  }

  pub(super) async fn ownership_transfer_query_models(
    &self,
    query: &OwnershipTransferQuerySpec,
  ) -> Result<Vec<ownership_transfer::ModelEx>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(ownership_transfer::Column::DeletedAt.is_null());

    if let Some(status) = query.status {
      condition = condition.add(ownership_transfer::Column::Status.eq(status));
    }

    ownership_transfer::Entity::load()
      .filter(condition)
      .with((ownership_transfer_item::Entity, product::Entity))
      .with((ownership_transfer_item::Entity, storage::Entity))
      .order_by_desc(ownership_transfer::Column::Date)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await
      .map_err(Into::into)
  }

  pub(super) async fn ownership_transfer_contractor_names(
    &self,
    contractor_ids: impl IntoIterator<Item = Uuid>,
  ) -> Result<HashMap<Uuid, String>, ApiError> {
    let ids = contractor_ids.into_iter().collect::<HashSet<_>>();
    if ids.is_empty() {
      return Ok(HashMap::new());
    }

    let companies = company::Entity::load()
      .filter(company::Column::Id.is_in(ids.into_iter().collect::<Vec<_>>()))
      .all(self.db.as_ref())
      .await?;

    Ok(
      companies
        .into_iter()
        .map(|company| (company.id, company.common_name))
        .collect(),
    )
  }

  pub async fn ownership_transfer_composite_list(
    &self,
  ) -> Result<Vec<OwnershipTransferResponse>, ApiError> {
    let docs = self
      .ownership_transfer_query_models(&OwnershipTransferQuerySpec::default())
      .await?;

    docs
      .into_iter()
      .map(OwnershipTransferResponse::try_from)
      .collect()
  }

  pub async fn ownership_transfer_composite_get(
    &self,
    id: Uuid,
  ) -> Result<OwnershipTransferResponse, ApiError> {
    let doc = self.ownership_transfer_model(id).await?;

    OwnershipTransferResponse::try_from(doc)
  }

  pub async fn ownership_transfer_composite_query(
    &self,
    query: OwnershipTransferQuerySpec,
  ) -> Result<Vec<OwnershipTransferResponse>, ApiError> {
    self
      .ownership_transfer_query_models(&query)
      .await?
      .into_iter()
      .map(OwnershipTransferResponse::try_from)
      .collect()
  }

  /// Returns one row per ownership transfer item with document fields repeated.
  /// Used by the grouped-row list table on the frontend.
  pub async fn ownership_transfer_flat_query(
    &self,
    query: OwnershipTransferFlatQuerySpec,
  ) -> Result<Vec<OwnershipTransferFlatRow>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;
    let docs = self
      .ownership_transfer_query_models(&OwnershipTransferQuerySpec {
        status: query.status,
        page: Some(page),
        per_page: Some(per_page),
      })
      .await?;

    let contractor_map = self
      .ownership_transfer_contractor_names(
        docs
          .iter()
          .flat_map(|doc| {
            doc
              .items
              .iter()
              .flat_map(|item| [item.from_contractor_id, item.to_contractor_id])
          })
          .collect::<Vec<_>>(),
      )
      .await?;

    let dash = "\u{2014}".to_string();

    let mut rows = Vec::new();
    for doc in &docs {
      if doc.items.is_empty() {
        rows.push(OwnershipTransferFlatRow {
          id: doc.id,
          document_id: doc.id,
          date: doc.date.to_string(),
          status: doc.status,
          item_id: doc.id,
          product_id_name: dash.clone(),
          storage_id_name: dash.clone(),
          from_contractor_id_name: dash.clone(),
          to_contractor_id_name: dash.clone(),
          amount: Default::default(),
        });
      }
      for item in &doc.items {
        rows.push(OwnershipTransferFlatRow {
          id: doc.id,
          document_id: doc.id,
          date: doc.date.to_string(),
          status: doc.status,
          item_id: item.id,
          product_id_name: item
            .product
            .as_ref()
            .map(|p| p.common_name.clone())
            .unwrap_or_default(),
          storage_id_name: item
            .storage
            .as_ref()
            .map(|s| s.common_name.clone())
            .unwrap_or_default(),
          from_contractor_id_name: contractor_map
            .get(&item.from_contractor_id)
            .cloned()
            .unwrap_or(dash.clone()),
          to_contractor_id_name: contractor_map
            .get(&item.to_contractor_id)
            .cloned()
            .unwrap_or(dash.clone()),
          amount: item.amount,
        });
      }
    }

    Ok(rows)
  }
}
