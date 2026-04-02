use sea_orm::{
  entity::prelude::Decimal, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter,
  QueryOrder,
};
use uuid::Uuid;

use super::FlowService;
use crate::{
  api::ApiError,
  dtos::response::flow::RailReceiptFlowRow,
  entities::{acceptance_document, acceptance_item, rail_wagon_manifest, rail_waybill},
  enums::PipelineStatus,
  services::common::normalize_pagination,
};

impl FlowService {
  pub async fn rail_receipt_query(
    &self,
    pipeline_status: Option<PipelineStatus>,
    contractor_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<RailReceiptFlowRow>, ApiError> {
    let (page, per_page) = normalize_pagination(page, per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(rail_waybill::Column::DeletedAt.is_null());
    if let Some(cid) = contractor_id {
      cond = cond.add(rail_waybill::Column::SenderId.eq(cid));
    }

    let waybills = rail_waybill::Entity::find()
      .filter(cond)
      .order_by_desc(rail_waybill::Column::Date)
      .paginate(db, per_page)
      .fetch_page(page - 1)
      .await?;

    if waybills.is_empty() {
      return Ok(vec![]);
    }

    let wb_ids: Vec<Uuid> = waybills.iter().map(|w| w.id).collect();

    let acceptances = acceptance_document::Entity::find()
      .filter(
        Condition::all()
          .add(acceptance_document::Column::RailWaybillId.is_in(wb_ids.clone()))
          .add(acceptance_document::Column::DeletedAt.is_null()),
      )
      .all(db)
      .await?;
    let acc_by_wb = Self::first_per_parent(&acceptances, |a| {
      a.rail_waybill_id.expect("filtered by is_in")
    });

    let manifests = rail_wagon_manifest::Entity::find()
      .filter(
        Condition::all()
          .add(rail_wagon_manifest::Column::RailWaybillId.is_in(wb_ids))
          .add(rail_wagon_manifest::Column::DeletedAt.is_null()),
      )
      .all(db)
      .await?;
    let first_manifest = Self::first_per_parent(&manifests, |m| m.rail_waybill_id);

    let acc_ids: Vec<Uuid> = acceptances.iter().map(|a| a.id).collect();
    let acc_items: Vec<acceptance_item::Model> = if acc_ids.is_empty() {
      vec![]
    } else {
      acceptance_item::Entity::find()
        .filter(
          Condition::all()
            .add(acceptance_item::Column::AcceptanceDocId.is_in(acc_ids))
            .add(acceptance_item::Column::DeletedAt.is_null()),
        )
        .all(db)
        .await?
    };
    let mut acc_sums: std::collections::HashMap<Uuid, Decimal> = std::collections::HashMap::new();
    for ai in &acc_items {
      *acc_sums.entry(ai.acceptance_doc_id).or_insert(Decimal::ZERO) += ai.accepted_amount;
    }

    let company_map =
      self.resolve_companies(&waybills.iter().map(|w| w.sender_id).collect::<Vec<_>>()).await?;
    let product_map = self
      .resolve_products(&first_manifest.values().map(|m| m.product_id).collect::<Vec<_>>())
      .await?;

    let mut rows = Vec::with_capacity(waybills.len());
    for wb in &waybills {
      let acc = acc_by_wb.get(&wb.id).copied();
      let status = PipelineStatus::from_doc_status(acc.map(|a| &a.status));
      if pipeline_status.is_some() && pipeline_status != Some(status) {
        continue;
      }

      let manifest = first_manifest.get(&wb.id);
      rows.push(RailReceiptFlowRow {
        basis_id: wb.id,
        basis_document_number: wb.document_number.clone(),
        basis_date: wb.date.to_string(),
        contractor_id: wb.sender_id,
        contractor_name: Self::company_name(&company_map, wb.sender_id),
        product_name: manifest.and_then(|m| product_map.get(&m.product_id).cloned()),
        expected_quantity: manifest.map(|m| m.declared_mass),
        action_id: acc.map(|a| a.id),
        action_document_number: acc.map(|a| a.document_number.clone()),
        actual_quantity: acc.and_then(|a| acc_sums.get(&a.id).copied()),
        pipeline_status: status,
      });
    }

    Ok(rows)
  }
}
