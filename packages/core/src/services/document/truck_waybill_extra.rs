use sea_orm::TransactionTrait;

use crate::{api::ApiError, dtos, services::DocumentService};

impl DocumentService {
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
      waybill: waybill.into(),
      items,
      weight_docs,
    })
  }
}
