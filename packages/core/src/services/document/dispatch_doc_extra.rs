use sea_orm::{
  ColumnTrait,
  Condition,
  EntityLoaderTrait,
  EntityTrait,
  PaginatorTrait,
  QueryFilter,
  TransactionTrait,
};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{dispatch_document, dispatch_item, dispatch_storage_measurement},
  enums,
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

    response.document.status = crate::enums::DocumentStatus::Posted;
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
      document: document.into(),
      items,
      storage_measurements,
    })
  }

  pub async fn dispatch_document_query(
    &self,
    document_number: Option<&str>,
    status: Option<enums::DocumentStatus>,
    contractor_id: Option<Uuid>,
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
}
