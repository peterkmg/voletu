use std::collections::HashMap;

use sea_orm::{
  entity::prelude::Decimal, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter,
  QueryOrder,
};
use uuid::Uuid;

use super::FlowService;
use crate::{
  api::ApiError,
  dtos::response::flow::TruckDispatchFlowRow,
  entities::{dispatch_document, dispatch_item},
  enums::{DispatchMethod, PipelineStatus},
  services::common::normalize_pagination,
};

impl FlowService {
  /// Query the truck-dispatch flow: dispatch documents (method = Truck)
  /// with a computed `pipeline_status` based on their document status.
  pub async fn truck_dispatch_query(
    &self,
    pipeline_status: Option<PipelineStatus>,
    contractor_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<TruckDispatchFlowRow>, ApiError> {
    let (page, per_page) = normalize_pagination(page, per_page)?;

    // -- 1. Fetch active truck-method dispatch documents ---------------------
    let mut dispatch_condition = Condition::all()
      .add(dispatch_document::Column::DeletedAt.is_null())
      .add(dispatch_document::Column::DispatchMethod.eq(DispatchMethod::Truck));

    if let Some(cid) = contractor_id {
      dispatch_condition = dispatch_condition.add(dispatch_document::Column::ContractorId.eq(cid));
    }

    let dispatches = dispatch_document::Entity::find()
      .filter(dispatch_condition)
      .order_by_desc(dispatch_document::Column::Date)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await?;

    if dispatches.is_empty() {
      return Ok(vec![]);
    }

    let dispatch_ids: Vec<Uuid> = dispatches.iter().map(|d| d.id).collect();

    // -- 2. Resolve company names (contractors) ------------------------------
    let contractor_ids: Vec<Uuid> = dispatches.iter().map(|d| d.contractor_id).collect();
    let company_map = self.resolve_companies(&contractor_ids).await?;

    // -- 3. Fetch dispatch items ---------------------------------------------
    let items: Vec<dispatch_item::Model> = dispatch_item::Entity::find()
      .filter(
        Condition::all()
          .add(dispatch_item::Column::DispatchDocId.is_in(dispatch_ids))
          .add(dispatch_item::Column::DeletedAt.is_null()),
      )
      .all(self.db.as_ref())
      .await?;

    let mut first_item_by_dispatch: HashMap<Uuid, &dispatch_item::Model> = HashMap::new();
    let mut dispatched_sum: HashMap<Uuid, Decimal> = HashMap::new();
    for item in &items {
      first_item_by_dispatch
        .entry(item.dispatch_doc_id)
        .or_insert(item);
      *dispatched_sum
        .entry(item.dispatch_doc_id)
        .or_insert(Decimal::ZERO) += item.dispatched_amount;
    }

    // -- 4. Resolve product names --------------------------------------------
    let product_ids: Vec<Uuid> = first_item_by_dispatch
      .values()
      .map(|i| i.product_id)
      .collect();
    let product_map = self.resolve_products(&product_ids).await?;

    // -- 5. Build rows -------------------------------------------------------
    let mut rows: Vec<TruckDispatchFlowRow> = Vec::with_capacity(dispatches.len());

    for dd in &dispatches {
      let status = PipelineStatus::from_doc_status(Some(&dd.status));

      if let Some(ref filter) = pipeline_status {
        if status != *filter {
          continue;
        }
      }

      let first_item = first_item_by_dispatch.get(&dd.id);
      let product_name = first_item
        .and_then(|i| product_map.get(&i.product_id))
        .map(|n| n.to_string());
      let dispatched_quantity = dispatched_sum.get(&dd.id).copied();

      rows.push(TruckDispatchFlowRow {
        dispatch_id: dd.id,
        document_number: dd.document_number.clone(),
        date: dd.date.to_string(),
        contractor_id: dd.contractor_id,
        contractor_name: company_map
          .get(&dd.contractor_id)
          .cloned()
          .unwrap_or_else(|| "Unknown".to_owned()),
        product_name,
        dispatched_quantity,
        pipeline_status: status,
      });
    }

    Ok(rows)
  }
}
