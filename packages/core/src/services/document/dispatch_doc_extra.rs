use sea_orm::{
  entity::prelude::*,
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
  dtos::{
    self,
    response::pipeline::{DispatchFlatRow, TruckDispatchPipelineResponse},
  },
  entities::{
    company,
    dispatch_document,
    dispatch_item,
    dispatch_storage_measurement,
    product,
    storage,
  },
  enums::{self, DispatchMethod, DocumentStatus, PipelineStatus},
  services::document::DocumentService,
};

impl DocumentService {
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
    let document = self
      .dispatch_document_create_no_tx(conn, &req.dispatch)
      .await?;

    let mut items = Vec::new();
    for item_req in &req.items {
      items.push(
        self
          .dispatch_item_create_no_tx(
            conn,
            &dtos::CreateDispatchItemRequest::from_composite(document.id, item_req),
          )
          .await?,
      );
    }

    let mut storage_measurements = Vec::new();

    if let Some(measurements_reqs) = &req.storage_measurements {
      for req in measurements_reqs {
        storage_measurements.push(
          self
            .dispatch_storage_measurement_create_no_tx(
              conn,
              &dtos::CreateDispatchMeasurementRequest::from_composite(document.id, req),
            )
            .await?,
        );
      }
    }

    Ok(dtos::DispatchCompositeResponse {
      document,
      items,
      storage_measurements,
    })
  }

  #[allow(clippy::too_many_arguments)]
  pub async fn dispatch_document_query(
    &self,
    document_number: Option<&str>,
    status: Option<enums::DocumentStatus>,
    contractor_id: Option<Uuid>,
    dispatch_method: Option<enums::DispatchMethod>,
    dispatch_purpose: Option<enums::DispatchPurpose>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::DispatchResponse>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(dispatch_document::Column::DeletedAt.is_null());

    if let Some(document_number) = document_number {
      condition =
        condition.add(dispatch_document::Column::DocumentNumber.contains(document_number));
    }

    if let Some(status) = status {
      condition = condition.add(dispatch_document::Column::Status.eq(status));
    }

    if let Some(contractor_id) = contractor_id {
      condition = condition.add(dispatch_document::Column::ContractorId.eq(contractor_id));
    }

    if let Some(dispatch_method) = dispatch_method {
      condition = condition.add(dispatch_document::Column::DispatchMethod.eq(dispatch_method));
    }

    if let Some(dispatch_purpose) = dispatch_purpose {
      condition = condition.add(dispatch_document::Column::DispatchPurpose.eq(dispatch_purpose));
    }

    let docs = dispatch_document::Entity::find()
      .filter(condition)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await?;

    Ok(docs.into_iter().map(dtos::DispatchResponse::from).collect())
  }

  pub async fn dispatch_composite_get(
    &self,
    id: Uuid,
  ) -> Result<dtos::DispatchCompositeResponse, ApiError> {
    let doc = dispatch_document::Entity::load()
      .filter_by_id(id)
      .with(dispatch_item::Entity)
      .with(dispatch_storage_measurement::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Dispatch document '{}' not found", id)))?;

    dtos::DispatchCompositeResponse::try_from(doc)
  }

  pub async fn truck_dispatch_pipeline_query(
    &self,
    pipeline_status: Option<PipelineStatus>,
    contractor_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<TruckDispatchPipelineResponse>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all()
      .add(dispatch_document::Column::DeletedAt.is_null())
      .add(dispatch_document::Column::DispatchMethod.eq(DispatchMethod::Truck));
    if let Some(cid) = contractor_id {
      cond = cond.add(dispatch_document::Column::ContractorId.eq(cid));
    }
    if let Some(ps) = pipeline_status {
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
    status: Option<enums::DocumentStatus>,
    dispatch_method: Option<enums::DispatchMethod>,
    dispatch_purpose: Option<enums::DispatchPurpose>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<DispatchFlatRow>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(dispatch_document::Column::DeletedAt.is_null());
    if let Some(s) = status {
      cond = cond.add(dispatch_document::Column::Status.eq(s));
    }
    if let Some(dm) = dispatch_method {
      cond = cond.add(dispatch_document::Column::DispatchMethod.eq(dm));
    }
    if let Some(dp) = dispatch_purpose {
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
      let contractor_name = doc
        .contractor
        .as_ref()
        .map(|c| c.common_name.clone())
        .unwrap_or("\u{2014}".to_string());

      if doc.items.is_empty() {
        rows.push(DispatchFlatRow {
          id: doc.id,
          document_id: doc.id,
          document_number: doc.document_number.clone(),
          date: doc.date.to_string(),
          status: doc.status,
          dispatch_method: doc.dispatch_method,
          dispatch_purpose: doc.dispatch_purpose,
          contractor_id_name: contractor_name.clone(),
          item_id: doc.id,
          product_id_name: "\u{2014}".to_string(),
          storage_id_name: "\u{2014}".to_string(),
          dispatched_amount: Default::default(),
        });
      }
      for item in &doc.items {
        rows.push(DispatchFlatRow {
          id: doc.id,
          document_id: doc.id,
          document_number: doc.document_number.clone(),
          date: doc.date.to_string(),
          status: doc.status,
          dispatch_method: doc.dispatch_method,
          dispatch_purpose: doc.dispatch_purpose,
          contractor_id_name: contractor_name.clone(),
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
          dispatched_amount: item.dispatched_amount,
        });
      }
    }

    Ok(rows)
  }
}
