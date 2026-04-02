use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, TransactionTrait};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::truck_waybill,
  services::DocumentService,
};

impl DocumentService {
  pub async fn truck_waybill_query(
    &self,
    document_number: Option<&str>,
    sender_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::TruckWaybillResponse>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(truck_waybill::Column::DeletedAt.is_null());

    if let Some(document_number) = document_number {
      condition =
        condition.add(truck_waybill::Column::DocumentNumber.contains(document_number));
    }

    if let Some(sender_id) = sender_id {
      condition = condition.add(truck_waybill::Column::SenderId.eq(sender_id));
    }

    let docs = truck_waybill::Entity::find()
      .filter(condition)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await?;

    Ok(
      docs
        .into_iter()
        .map(dtos::TruckWaybillResponse::from)
        .collect(),
    )
  }

  pub async fn truck_waybill_composite_create(
    &self,
    req: &dtos::TruckWaybillCompositeRequest,
  ) -> Result<dtos::TruckWaybillCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;
    let response = self.truck_waybill_composite_create_no_tx(&txn, req).await?;
    txn.commit().await?;
    Ok(response)
  }

  pub(crate) async fn truck_waybill_composite_create_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    req: &dtos::TruckWaybillCompositeRequest,
  ) -> Result<dtos::TruckWaybillCompositeResponse, ApiError> {
    let waybill = self
      .truck_waybill_create_no_tx(conn, &dtos::CreateTruckWaybillRequest::from_composite(req))
      .await?;

    let mut items: Option<Vec<dtos::TruckWaybillItemResponse>> = None;
    let mut weight_docs: Option<Vec<dtos::TruckWeightDocResponse>> = None;

    if let Some(item_reqs) = &req.items {
      for item_req in item_reqs {
        let item = self
          .truck_waybill_item_create_no_tx(
            conn,
            &dtos::CreateTruckWaybillItemRequest::from_composite(waybill.id, item_req),
          )
          .await?;

        items.get_or_insert_with(Vec::new).push(item);
      }
    }

    if let Some(weight_doc_reqs) = &req.weight_docs {
      for weight_doc_req in weight_doc_reqs {
        let weight_doc = self
          .truck_weight_doc_create_no_tx(
            conn,
            &dtos::CreateTruckWeightDocRequest::from_composite(waybill.id, weight_doc_req),
          )
          .await?;

        weight_docs.get_or_insert_with(Vec::new).push(weight_doc);
      }
    }

    Ok(dtos::TruckWaybillCompositeResponse {
      waybill,
      items,
      weight_docs,
    })
  }
}
