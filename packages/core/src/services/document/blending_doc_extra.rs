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
  entities::{blending_component, blending_document, blending_result},
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

    res.document.status = crate::enums::DocumentStatus::Posted;
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
}
