use sea_orm::{
  entity::prelude::*,
  ColumnTrait,
  Condition,
  EntityLoaderTrait,
  QueryFilter,
  QueryOrder,
  TransactionTrait,
};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::{
    self,
    response::pipeline::{DispatchFlatRow, DispatchFlatRowRef, TruckDispatchPipelineResponse},
  },
  entities::{
    base,
    company,
    dispatch_document,
    dispatch_item,
    dispatch_storage_measurement,
    port,
    product,
    storage,
  },
  enums::{DispatchMethod, DocumentStatus, PipelineStatus},
  services::document::{
    query::{DispatchDocumentQuerySpec, DispatchFlatQuerySpec, TruckDispatchPipelineQuerySpec},
    DocumentService,
  },
};

impl DocumentService {
  pub(super) async fn dispatch_document_model(
    &self,
    id: Uuid,
  ) -> Result<dispatch_document::ModelEx, ApiError> {
    dispatch_document::Entity::load()
      .filter_by_id(id)
      .filter(dispatch_document::Column::DeletedAt.is_null())
      .with(company::Entity)
      .with(base::Entity)
      .with(port::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Dispatch document '{}' not found", id)))
  }

  pub(super) async fn dispatch_document_query_models(
    &self,
    query: &DispatchDocumentQuerySpec,
  ) -> Result<Vec<dispatch_document::ModelEx>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(dispatch_document::Column::DeletedAt.is_null());

    if let Some(document_number) = query.document_number.as_deref() {
      condition =
        condition.add(dispatch_document::Column::DocumentNumber.contains(document_number));
    }

    if let Some(status) = query.status {
      condition = condition.add(dispatch_document::Column::Status.eq(status));
    }

    if let Some(contractor_id) = query.contractor_id {
      condition = condition.add(dispatch_document::Column::ContractorId.eq(contractor_id));
    }

    if let Some(dispatch_method) = query.dispatch_method {
      condition = condition.add(dispatch_document::Column::DispatchMethod.eq(dispatch_method));
    }

    if let Some(dispatch_purpose) = query.dispatch_purpose {
      condition = condition.add(dispatch_document::Column::DispatchPurpose.eq(dispatch_purpose));
    }

    Ok(
      dispatch_document::Entity::load()
        .filter(condition)
        .with(company::Entity)
        .with(base::Entity)
        .with(port::Entity)
        .order_by_desc(dispatch_document::Column::Date)
        .paginate(self.db.as_ref(), per_page)
        .fetch_page(page - 1)
        .await?,
    )
  }

  pub(super) async fn dispatch_composite_model(
    &self,
    id: Uuid,
  ) -> Result<dispatch_document::ModelEx, ApiError> {
    dispatch_document::Entity::load()
      .filter_by_id(id)
      .filter(dispatch_document::Column::DeletedAt.is_null())
      .with(company::Entity)
      .with(base::Entity)
      .with(port::Entity)
      .with((dispatch_item::Entity, product::Entity))
      .with((dispatch_item::Entity, storage::Entity))
      .with((dispatch_storage_measurement::Entity, storage::Entity))
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Dispatch document '{}' not found", id)))
  }

  pub async fn dispatch_composite_create(
    &self,
    req: &dtos::CreateDispatchCompositeRequest,
  ) -> Result<dtos::DispatchCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;
    let response = self.dispatch_composite_create_no_tx(&txn, req).await?;
    txn.commit().await?;

    Ok(response)
  }

  pub async fn dispatch_composite_create_and_execute(
    &self,
    req: &dtos::CreateDispatchCompositeRequest,
    actor_id: Uuid,
  ) -> Result<dtos::DispatchCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;

    let mut response = self.dispatch_composite_create_no_tx(&txn, req).await?;

    self
      .dispatch_document_execute_no_tx(&txn, response.document.id, actor_id)
      .await?;

    response.document.status = crate::enums::DocumentStatus::Executed;
    txn.commit().await?;

    Ok(response)
  }

  pub(crate) async fn dispatch_composite_create_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    req: &dtos::CreateDispatchCompositeRequest,
  ) -> Result<dtos::DispatchCompositeResponse, ApiError> {
    let saved = dispatch_document::ActiveModelEx::from(req)
      .save(conn)
      .await?;
    let document_id = match saved.id {
      sea_orm::ActiveValue::Set(id) | sea_orm::ActiveValue::Unchanged(id) => id,
      sea_orm::ActiveValue::NotSet => {
        return Err(ApiError::Internal(anyhow::anyhow!(
          "dispatch graph save returned no id"
        )));
      }
    };

    self
      .audit
      .backfill_document_routing::<dispatch_document::Entity>(conn, document_id)
      .await?;

    dtos::DispatchCompositeResponse::try_from(
      dispatch_document::Entity::load()
        .filter_by_id(document_id)
        .filter(dispatch_document::Column::DeletedAt.is_null())
        .with(company::Entity)
        .with(base::Entity)
        .with(port::Entity)
        .with((dispatch_item::Entity, product::Entity))
        .with((dispatch_item::Entity, storage::Entity))
        .with((dispatch_storage_measurement::Entity, storage::Entity))
        .one(conn)
        .await?
        .ok_or_else(|| {
          ApiError::NotFound(format!("Dispatch document '{}' not found", document_id))
        })?,
    )
  }

  pub async fn dispatch_document_query(
    &self,
    query: DispatchDocumentQuerySpec,
  ) -> Result<Vec<dtos::DispatchResponse>, ApiError> {
    Ok(
      self
        .dispatch_document_query_models(&query)
        .await?
        .into_iter()
        .map(|doc| dtos::DispatchResponse::from(dispatch_document::Model::from(doc)))
        .collect(),
    )
  }

  pub async fn dispatch_composite_get(
    &self,
    id: Uuid,
  ) -> Result<dtos::DispatchCompositeResponse, ApiError> {
    let doc = self.dispatch_composite_model(id).await?;

    dtos::DispatchCompositeResponse::try_from(doc)
  }

  pub async fn truck_dispatch_pipeline_query(
    &self,
    query: TruckDispatchPipelineQuerySpec,
  ) -> Result<Vec<TruckDispatchPipelineResponse>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all()
      .add(dispatch_document::Column::DeletedAt.is_null())
      .add(dispatch_document::Column::DispatchMethod.eq(DispatchMethod::Truck));
    if let Some(cid) = query.contractor_id {
      cond = cond.add(dispatch_document::Column::ContractorId.eq(cid));
    }
    if let Some(ps) = query.pipeline_status {
      match ps {
        PipelineStatus::Pending => return Ok(vec![]),
        PipelineStatus::Draft => {
          cond = cond.add(dispatch_document::Column::Status.eq(DocumentStatus::Draft))
        }
        PipelineStatus::Executed => {
          cond = cond.add(dispatch_document::Column::Status.eq(DocumentStatus::Executed))
        }
      }
    }

    let dispatches: Vec<dispatch_document::ModelEx> = dispatch_document::Entity::load()
      .filter(cond)
      .with(company::Entity)
      .with((dispatch_item::Entity, product::Entity))
      .order_by_desc(dispatch_document::Column::Date)
      .paginate(db, per_page)
      .fetch_page(page - 1)
      .await?;

    let mut rows = Vec::with_capacity(dispatches.len());
    for dd in &dispatches {
      let first_item = dd.items.get(0);
      let total: Decimal = dd.items.iter().map(|i| i.dispatched_amount).sum();

      rows.push(TruckDispatchPipelineResponse {
        id: dd.id,
        document_number: dd.document_number.clone(),
        date: dd.date.to_string(),
        contractor_id: dd.contractor_id,
        contractor_name: dd
          .contractor
          .as_ref()
          .map(|c| c.common_name.clone())
          .unwrap_or_default(),
        product_name: first_item.and_then(|i| i.product.as_ref().map(|p| p.common_name.clone())),
        dispatched_quantity: if total > Decimal::ZERO {
          Some(total)
        } else {
          None
        },
        pipeline_status: PipelineStatus::from_doc_status(Some(&dd.status)),
      });
    }

    Ok(rows)
  }

  /// Returns one row per dispatch item with document fields repeated.
  /// Used by the grouped-row list table on the frontend.
  pub async fn dispatch_flat_query(
    &self,
    query: DispatchFlatQuerySpec,
  ) -> Result<Vec<DispatchFlatRow>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(dispatch_document::Column::DeletedAt.is_null());
    if let Some(s) = query.status {
      cond = cond.add(dispatch_document::Column::Status.eq(s));
    }
    if let Some(dm) = query.dispatch_method {
      cond = cond.add(dispatch_document::Column::DispatchMethod.eq(dm));
    }
    if let Some(dp) = query.dispatch_purpose {
      cond = cond.add(dispatch_document::Column::DispatchPurpose.eq(dp));
    }

    let docs: Vec<dispatch_document::ModelEx> = dispatch_document::Entity::load()
      .filter(cond)
      .with(company::Entity)
      .with((dispatch_item::Entity, product::Entity))
      .with((dispatch_item::Entity, storage::Entity))
      .order_by_desc(dispatch_document::Column::Date)
      .paginate(db, per_page)
      .fetch_page(page - 1)
      .await?;

    let mut rows = Vec::new();
    for doc in &docs {
      if doc.items.is_empty() {
        rows.push(DispatchFlatRow::from(DispatchFlatRowRef {
          document: doc,
          item: None,
        }));
      }
      for item in &doc.items {
        rows.push(DispatchFlatRow::from(DispatchFlatRowRef {
          document: doc,
          item: Some(item),
        }));
      }
    }

    Ok(rows)
  }
}
