use sea_orm::{
  ColumnTrait,
  Condition,
  EntityLoaderTrait,
  EntityTrait,
  PaginatorTrait,
  QueryFilter,
  QueryOrder,
  TransactionTrait,
};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::{self, response::pipeline::BlendingFlatRow},
  entities::{blending_component, blending_document, blending_result, company, product, storage},
  enums,
  services::document::DocumentService,
};

impl DocumentService {
  pub async fn blending_composite_create(
    &self,
    req: &dtos::CreateBlendingCompositeRequest,
  ) -> Result<dtos::BlendingCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;
    let response = self.blending_composite_create_no_tx(&txn, req).await?;
    txn.commit().await?;

    Ok(response)
  }

  pub async fn blending_composite_create_and_execute(
    &self,
    req: &dtos::CreateBlendingCompositeRequest,
    actor_id: Uuid,
  ) -> Result<dtos::BlendingCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;

    let mut res = self.blending_composite_create_no_tx(&txn, req).await?;

    self
      .blending_document_execute_no_tx(&txn, res.document.id, actor_id)
      .await?;

    res.document.status = crate::enums::DocumentStatus::Executed;
    txn.commit().await?;
    Ok(res)
  }

  pub(crate) async fn blending_composite_create_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    req: &dtos::CreateBlendingCompositeRequest,
  ) -> Result<dtos::BlendingCompositeResponse, ApiError> {
    let doc = self
      .blending_document_create_no_tx(conn, &dtos::CreateBlendingRequest::from_composite(req))
      .await?;

    let mut components = Vec::new();
    for comp_req in &req.components {
      components.push(
        self
          .blending_component_create_no_tx(
            conn,
            &dtos::CreateBlendingComponentRequest::from_composite(doc.id, comp_req),
          )
          .await?,
      );
    }

    let mut results = Vec::new();
    for res_req in &req.results {
      results.push(
        self
          .blending_result_create_no_tx(
            conn,
            &dtos::CreateBlendingResultRequest::from_composite(doc.id, res_req),
          )
          .await?,
      );
    }

    self
      .audit
      .backfill_document_routing(conn, "blending_documents", doc.id)
      .await?;

    Ok(dtos::BlendingCompositeResponse {
      document: doc,
      components,
      results,
    })
  }

  pub async fn blending_document_query(
    &self,
    doc_num: Option<&str>,
    status: Option<enums::DocumentStatus>,
    contractor_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::BlendingResponse>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(blending_document::Column::DeletedAt.is_null());

    if let Some(document_number) = doc_num {
      condition =
        condition.add(blending_document::Column::DocumentNumber.contains(document_number));
    }

    if let Some(status) = status {
      condition = condition.add(blending_document::Column::Status.eq(status));
    }

    if let Some(contractor_id) = contractor_id {
      condition = condition.add(blending_document::Column::ContractorId.eq(contractor_id));
    }

    let docs = blending_document::Entity::find()
      .filter(condition)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await?;

    Ok(docs.into_iter().map(dtos::BlendingResponse::from).collect())
  }

  pub async fn blending_composite_get(
    &self,
    document_id: Uuid,
  ) -> Result<dtos::BlendingCompositeResponse, ApiError> {
    let doc = blending_document::Entity::load()
      .filter_by_id(document_id)
      .filter(blending_document::Column::DeletedAt.is_null())
      .with(blending_component::Entity)
      .with(blending_result::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| {
        ApiError::NotFound(format!("Blending document '{}' not found", document_id))
      })?;

    dtos::BlendingCompositeResponse::try_from(doc)
  }

  /// Returns one row per blending component/result with document fields repeated.
  /// Used by the grouped-row list table on the frontend.
  pub async fn blending_flat_query(
    &self,
    status: Option<enums::DocumentStatus>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<BlendingFlatRow>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(blending_document::Column::DeletedAt.is_null());
    if let Some(s) = status {
      cond = cond.add(blending_document::Column::Status.eq(s));
    }

    let docs: Vec<blending_document::ModelEx> = blending_document::Entity::load()
      .filter(cond)
      .with(company::Entity) // doc-level contractor
      .with(product::Entity) // doc-level target_product
      .with((blending_component::Entity, product::Entity)) // component source_product
      .with((blending_component::Entity, storage::Entity))
      .with((blending_result::Entity, storage::Entity))
      .order_by_desc(blending_document::Column::Date)
      .paginate(db, per_page)
      .fetch_page(page - 1)
      .await?;

    let dash = "\u{2014}".to_string();

    let mut rows = Vec::new();
    for doc in &docs {
      let contractor_name = doc
        .contractor
        .as_ref()
        .map(|c| c.common_name.clone())
        .unwrap_or(dash.clone());
      let target_product_name = doc
        .target_product
        .as_ref()
        .map(|p| p.common_name.clone())
        .unwrap_or(dash.clone());

      let has_items = !doc.components.is_empty() || !doc.results.is_empty();

      if !has_items {
        rows.push(BlendingFlatRow {
          id: doc.id,
          document_id: doc.id,
          document_number: doc.document_number.clone(),
          date: doc.date.to_string(),
          status: doc.status,
          contractor_id_name: contractor_name.clone(),
          target_product_id_name: target_product_name.clone(),
          item_id: doc.id,
          item_type: dash.clone(),
          product_id_name: dash.clone(),
          storage_id_name: dash.clone(),
          amount: Default::default(),
        });
      }

      for comp in &doc.components {
        rows.push(BlendingFlatRow {
          id: doc.id,
          document_id: doc.id,
          document_number: doc.document_number.clone(),
          date: doc.date.to_string(),
          status: doc.status,
          contractor_id_name: contractor_name.clone(),
          target_product_id_name: target_product_name.clone(),
          item_id: comp.id,
          item_type: "component".to_string(),
          product_id_name: comp
            .source_product
            .as_ref()
            .map(|p| p.common_name.clone())
            .unwrap_or_default(),
          storage_id_name: comp
            .storage
            .as_ref()
            .map(|s| s.common_name.clone())
            .unwrap_or_default(),
          amount: comp.amount_used,
        });
      }

      for res in &doc.results {
        rows.push(BlendingFlatRow {
          id: doc.id,
          document_id: doc.id,
          document_number: doc.document_number.clone(),
          date: doc.date.to_string(),
          status: doc.status,
          contractor_id_name: contractor_name.clone(),
          target_product_id_name: target_product_name.clone(),
          item_id: res.id,
          item_type: "result".to_string(),
          product_id_name: target_product_name.clone(),
          storage_id_name: res
            .storage
            .as_ref()
            .map(|s| s.common_name.clone())
            .unwrap_or_default(),
          amount: res.produced_amount,
        });
      }
    }

    Ok(rows)
  }
}
