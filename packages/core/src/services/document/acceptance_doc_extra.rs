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
  endpoints::query::NullableFilter,
  entities::{acceptance_document, acceptance_item},
  enums,
  services::document::DocumentService,
};

impl DocumentService {
  pub async fn acceptance_composite_create(
    &self,
    req: &dtos::CreateAcceptanceCompositeRequest,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;

    let res = self.acceptance_composite_create_no_tx(&txn, req).await?;

    txn.commit().await?;

    Ok(res)
  }

  pub async fn acceptance_composite_create_and_execute(
    &self,
    req: &dtos::CreateAcceptanceCompositeRequest,
    actor_id: Uuid,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;

    let mut response = self.acceptance_composite_create_no_tx(&txn, req).await?;

    self
      .acceptance_document_execute_no_tx(&txn, response.document.id, actor_id)
      .await?;

    response.document.status = crate::enums::DocumentStatus::Posted;
    txn.commit().await?;

    Ok(response)
  }

  pub(crate) async fn acceptance_composite_create_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    req: &dtos::CreateAcceptanceCompositeRequest,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    let document = self
      .acceptance_document_create_no_tx(conn, &req.acceptance)
      .await?;

    let mut items = Vec::new();

    for item_req in &req.items {
      items.push(
        self
          .acceptance_item_create_no_tx(
            conn,
            &dtos::CreateAcceptanceItemRequest::from_composite(document.id, item_req),
          )
          .await?,
      );
    }

    Ok(dtos::AcceptanceCompositeResponse { document, items })
  }

  #[allow(clippy::too_many_arguments)]
  pub async fn acceptance_document_query(
    &self,
    document_number: Option<&str>,
    status: Option<enums::DocumentStatus>,
    truck_waybill_id: Option<NullableFilter>,
    rail_waybill_id: Option<NullableFilter>,
    transit_dispatch_id: Option<NullableFilter>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::AcceptanceResponse>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(acceptance_document::Column::DeletedAt.is_null());

    if let Some(document_number) = document_number {
      condition =
        condition.add(acceptance_document::Column::DocumentNumber.contains(document_number));
    }

    if let Some(status) = status {
      condition = condition.add(acceptance_document::Column::Status.eq(status));
    }

    if let Some(filter) = truck_waybill_id {
      match filter {
        NullableFilter::IsNull => {
          condition = condition.add(acceptance_document::Column::TruckWaybillId.is_null());
        }
        NullableFilter::IsNotNull => {
          condition = condition.add(acceptance_document::Column::TruckWaybillId.is_not_null());
        }
      }
    }

    if let Some(filter) = rail_waybill_id {
      match filter {
        NullableFilter::IsNull => {
          condition = condition.add(acceptance_document::Column::RailWaybillId.is_null());
        }
        NullableFilter::IsNotNull => {
          condition = condition.add(acceptance_document::Column::RailWaybillId.is_not_null());
        }
      }
    }

    if let Some(filter) = transit_dispatch_id {
      match filter {
        NullableFilter::IsNull => {
          condition = condition.add(acceptance_document::Column::TransitDispatchId.is_null());
        }
        NullableFilter::IsNotNull => {
          condition = condition.add(acceptance_document::Column::TransitDispatchId.is_not_null());
        }
      }
    }

    let docs = acceptance_document::Entity::find()
      .filter(condition)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await?;

    Ok(
      docs
        .into_iter()
        .map(dtos::AcceptanceResponse::from)
        .collect(),
    )
  }

  pub async fn acceptance_composite_get(
    &self,
    id: Uuid,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    let doc = acceptance_document::Entity::load()
      .filter_by_id(id)
      .filter(acceptance_document::Column::DeletedAt.is_null())
      .with(acceptance_item::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Acceptance document '{}' not found", id)))?;

    dtos::AcceptanceCompositeResponse::try_from(doc)
  }
}
