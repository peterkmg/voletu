use std::collections::HashMap;

use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::response::flow::TruckDispatchFlowRow,
  entities::{company, dispatch_document, dispatch_item, product},
  enums::{DispatchMethod, DocumentStatus},
  services::common::normalize_pagination,
};

use super::FlowService;

impl FlowService {
  /// Query the truck-dispatch flow: dispatch documents (method = Truck)
  /// with a computed `pipeline_status` based on their document status.
  #[allow(clippy::too_many_arguments)]
  pub async fn truck_dispatch_query(
    &self,
    pipeline_status: Option<&str>,
    contractor_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<TruckDispatchFlowRow>, ApiError> {
    let (page, per_page) = normalize_pagination(page, per_page)?;

    // ── 1. Fetch active truck-method dispatch documents ────────────────
    let mut dispatch_condition = Condition::all()
      .add(dispatch_document::Column::DeletedAt.is_null())
      .add(dispatch_document::Column::DispatchMethod.eq(DispatchMethod::Truck));

    if let Some(cid) = contractor_id {
      dispatch_condition =
        dispatch_condition.add(dispatch_document::Column::ContractorId.eq(cid));
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
    let contractor_ids: Vec<Uuid> = dispatches.iter().map(|d| d.contractor_id).collect();

    // ── 2. Fetch company names (contractors) ────────────────────────────
    let unique_contractor_ids: Vec<Uuid> = {
      let mut ids = contractor_ids.clone();
      ids.sort();
      ids.dedup();
      ids
    };
    let companies: Vec<company::Model> = company::Entity::find()
      .filter(company::Column::Id.is_in(unique_contractor_ids))
      .all(self.db.as_ref())
      .await?;
    let company_map: HashMap<Uuid, &str> = companies
      .iter()
      .map(|c| (c.id, c.common_name.as_str()))
      .collect();

    // ── 3. Fetch dispatch items ────────────────────────────────────────
    let items: Vec<dispatch_item::Model> = dispatch_item::Entity::find()
      .filter(
        Condition::all()
          .add(dispatch_item::Column::DispatchDocId.is_in(dispatch_ids.clone()))
          .add(dispatch_item::Column::DeletedAt.is_null()),
      )
      .all(self.db.as_ref())
      .await?;

    // Group items by dispatch, take the first one per dispatch
    let mut first_item_by_dispatch: HashMap<Uuid, &dispatch_item::Model> = HashMap::new();
    for item in &items {
      first_item_by_dispatch
        .entry(item.dispatch_doc_id)
        .or_insert(item);
    }

    // Sum dispatched_amount per dispatch doc
    let mut dispatched_sum: HashMap<Uuid, sea_orm::entity::prelude::Decimal> = HashMap::new();
    for item in &items {
      *dispatched_sum
        .entry(item.dispatch_doc_id)
        .or_insert_with(|| sea_orm::entity::prelude::Decimal::ZERO) += item.dispatched_amount;
    }

    // ── 4. Fetch product names for the items ────────────────────────────
    let product_ids: Vec<Uuid> = first_item_by_dispatch
      .values()
      .map(|i| i.product_id)
      .collect();
    let products: Vec<product::Model> = if product_ids.is_empty() {
      vec![]
    } else {
      product::Entity::find()
        .filter(product::Column::Id.is_in(product_ids))
        .all(self.db.as_ref())
        .await?
    };
    let product_map: HashMap<Uuid, &str> = products
      .iter()
      .map(|p| (p.id, p.common_name.as_str()))
      .collect();

    // ── 5. Build rows ──────────────────────────────────────────────────
    let mut rows: Vec<TruckDispatchFlowRow> = Vec::with_capacity(dispatches.len());

    for dd in &dispatches {
      let status = match dd.status {
        DocumentStatus::Draft => "draft",
        DocumentStatus::Posted => "executed",
      };

      // Apply pipeline_status filter (post-query filter)
      if let Some(ps) = pipeline_status {
        if status != ps {
          continue;
        }
      }

      let first_item = first_item_by_dispatch.get(&dd.id);
      let product_name = first_item
        .and_then(|i| product_map.get(&i.product_id))
        .map(|n| (*n).to_owned());
      let dispatched_quantity = dispatched_sum.get(&dd.id).copied();

      rows.push(TruckDispatchFlowRow {
        dispatch_id: dd.id,
        document_number: dd.document_number.clone(),
        date: dd.date.to_string(),
        contractor_id: dd.contractor_id,
        contractor_name: company_map
          .get(&dd.contractor_id)
          .unwrap_or(&"Unknown")
          .to_string(),
        product_name,
        dispatched_quantity,
        pipeline_status: status.to_owned(),
      });
    }

    Ok(rows)
  }
}
