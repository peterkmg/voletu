use std::collections::HashMap;

use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::response::flow::RailReceiptFlowRow,
  entities::{acceptance_document, acceptance_item, company, product, rail_wagon_manifest, rail_waybill},
  enums::DocumentStatus,
  services::common::normalize_pagination,
};

use super::FlowService;

impl FlowService {
  /// Query the rail-receipt flow: rail waybills LEFT JOINed with acceptance
  /// documents, with a computed `pipeline_status`.
  #[allow(clippy::too_many_arguments)]
  pub async fn rail_receipt_query(
    &self,
    pipeline_status: Option<&str>,
    contractor_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<RailReceiptFlowRow>, ApiError> {
    let (page, per_page) = normalize_pagination(page, per_page)?;

    // ── 1. Fetch active rail waybills ──────────────────────────────────
    let mut waybill_condition = Condition::all()
      .add(rail_waybill::Column::DeletedAt.is_null());

    if let Some(cid) = contractor_id {
      waybill_condition = waybill_condition.add(rail_waybill::Column::SenderId.eq(cid));
    }

    let waybills = rail_waybill::Entity::find()
      .filter(waybill_condition)
      .order_by_desc(rail_waybill::Column::Date)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await?;

    if waybills.is_empty() {
      return Ok(vec![]);
    }

    let waybill_ids: Vec<Uuid> = waybills.iter().map(|w| w.id).collect();
    let sender_ids: Vec<Uuid> = waybills.iter().map(|w| w.sender_id).collect();

    // ── 2. Fetch acceptance documents linked to these waybills ──────────
    let acceptances: Vec<acceptance_document::Model> = acceptance_document::Entity::find()
      .filter(
        Condition::all()
          .add(acceptance_document::Column::RailWaybillId.is_in(waybill_ids.clone()))
          .add(acceptance_document::Column::DeletedAt.is_null()),
      )
      .all(self.db.as_ref())
      .await?;

    let acceptance_by_waybill: HashMap<Uuid, &acceptance_document::Model> = acceptances
      .iter()
      .filter_map(|a| a.rail_waybill_id.map(|wid| (wid, a)))
      .collect();

    // ── 3. Fetch company names (senders/contractors) ────────────────────
    let unique_sender_ids: Vec<Uuid> = {
      let mut ids = sender_ids.clone();
      ids.sort();
      ids.dedup();
      ids
    };
    let companies: Vec<company::Model> = company::Entity::find()
      .filter(company::Column::Id.is_in(unique_sender_ids))
      .all(self.db.as_ref())
      .await?;
    let company_map: HashMap<Uuid, &str> = companies
      .iter()
      .map(|c| (c.id, c.common_name.as_str()))
      .collect();

    // ── 4. Fetch first wagon manifest per waybill (for product + quantity)
    let manifests: Vec<rail_wagon_manifest::Model> = rail_wagon_manifest::Entity::find()
      .filter(
        Condition::all()
          .add(rail_wagon_manifest::Column::RailWaybillId.is_in(waybill_ids.clone()))
          .add(rail_wagon_manifest::Column::DeletedAt.is_null()),
      )
      .all(self.db.as_ref())
      .await?;

    // Group manifests by waybill, take the first one per waybill
    let mut first_manifest_by_waybill: HashMap<Uuid, &rail_wagon_manifest::Model> = HashMap::new();
    for manifest in &manifests {
      first_manifest_by_waybill
        .entry(manifest.rail_waybill_id)
        .or_insert(manifest);
    }

    // ── 5. Fetch product names for the manifests ────────────────────────
    let product_ids: Vec<Uuid> = first_manifest_by_waybill
      .values()
      .map(|m| m.product_id)
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

    // ── 6. Fetch acceptance items for linked acceptances ────────────────
    let acceptance_ids: Vec<Uuid> = acceptances.iter().map(|a| a.id).collect();
    let acceptance_items: Vec<acceptance_item::Model> = if acceptance_ids.is_empty() {
      vec![]
    } else {
      acceptance_item::Entity::find()
        .filter(
          Condition::all()
            .add(acceptance_item::Column::AcceptanceDocId.is_in(acceptance_ids))
            .add(acceptance_item::Column::DeletedAt.is_null()),
        )
        .all(self.db.as_ref())
        .await?
    };

    // Sum accepted_amount per acceptance doc
    let mut accepted_sum: HashMap<Uuid, sea_orm::entity::prelude::Decimal> = HashMap::new();
    for ai in &acceptance_items {
      *accepted_sum
        .entry(ai.acceptance_doc_id)
        .or_insert_with(|| sea_orm::entity::prelude::Decimal::ZERO) += ai.accepted_amount;
    }

    // ── 7. Build rows ──────────────────────────────────────────────────
    let mut rows: Vec<RailReceiptFlowRow> = Vec::with_capacity(waybills.len());

    for wb in &waybills {
      let acceptance = acceptance_by_waybill.get(&wb.id).copied();

      let status = match acceptance {
        None => "pending",
        Some(a) => match a.status {
          DocumentStatus::Draft => "draft",
          DocumentStatus::Posted => "executed",
        },
      };

      // Apply pipeline_status filter (post-join filter)
      if let Some(ps) = pipeline_status {
        if status != ps {
          continue;
        }
      }

      let first_manifest = first_manifest_by_waybill.get(&wb.id);
      let product_name = first_manifest
        .and_then(|m| product_map.get(&m.product_id))
        .map(|n| (*n).to_owned());
      let expected_quantity = first_manifest.map(|m| m.declared_mass);

      let actual_quantity = acceptance.and_then(|a| accepted_sum.get(&a.id).copied());

      rows.push(RailReceiptFlowRow {
        basis_id: wb.id,
        basis_document_number: wb.document_number.clone(),
        basis_date: wb.date.to_string(),
        contractor_id: wb.sender_id,
        contractor_name: company_map
          .get(&wb.sender_id)
          .unwrap_or(&"Unknown")
          .to_string(),
        product_name,
        expected_quantity,
        action_id: acceptance.map(|a| a.id),
        action_document_number: acceptance.map(|a| a.document_number.clone()),
        actual_quantity,
        pipeline_status: status.to_owned(),
      });
    }

    Ok(rows)
  }
}
