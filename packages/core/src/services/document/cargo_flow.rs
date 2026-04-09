use crate::{
  api::ApiError,
  dtos::response::pipeline::CargoFlowFlatRow,
  enums,
  services::document::{
    query::{
      AcceptanceFlatQuerySpec, BlendingFlatQuerySpec, CargoFlowQuerySpec, DispatchFlatQuerySpec,
      OwnershipTransferFlatQuerySpec, PhysicalTransferFlatQuerySpec, ReconciliationFlatQuerySpec,
    },
    DocumentService,
  },
};

impl DocumentService {
  /// Returns a unified cargo-flow view: one row per item across ALL document types.
  ///
  /// Each document type's flat query is called without filters (full dataset),
  /// results are normalized into `CargoFlowFlatRow`, sorted by date descending,
  /// and paginated in-memory.
  pub async fn cargo_flow_flat_query(
    &self,
    query: CargoFlowQuerySpec,
  ) -> Result<Vec<CargoFlowFlatRow>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;

    // Fetch all document types in parallel (no filters, large page to get all).
    let big_page = Some(1u64);
    let big_per_page = Some(10_000u64);

    let (acceptance, dispatch, physical, ownership, blending, reconciliation) = tokio::try_join!(
      self.acceptance_flat_query(AcceptanceFlatQuerySpec::list(big_page, big_per_page)),
      self.dispatch_flat_query(DispatchFlatQuerySpec::list(big_page, big_per_page)),
      self
        .physical_transfer_flat_query(PhysicalTransferFlatQuerySpec::list(big_page, big_per_page)),
      self.ownership_transfer_flat_query(OwnershipTransferFlatQuerySpec::list(
        big_page,
        big_per_page
      )),
      self.blending_flat_query(BlendingFlatQuerySpec::list(big_page, big_per_page)),
      self.reconciliation_flat_query(ReconciliationFlatQuerySpec::list(big_page, big_per_page)),
    )?;

    let mut rows: Vec<CargoFlowFlatRow> = Vec::new();

    // --- Acceptance items → Incoming ---
    for r in &acceptance {
      let operation = match r.source_entity.as_deref() {
        Some(_) => "External Receipt".to_string(),
        None => "Acceptance".to_string(),
      };
      rows.push(CargoFlowFlatRow {
        id: r.item_id,
        document_id: r.document_id,
        document_number: r.document_number.clone(),
        date: r.date_accepted.clone(),
        flow_type: "Incoming".to_string(),
        operation,
        contractor_name: r.contractor_id_name.clone(),
        status: format!("{:?}", r.status),
        flow_route: "/incoming/external".to_string(),
        product_name: r.product_id_name.clone(),
        storage_name: r.storage_id_name.clone(),
        quantity: r.accepted_amount.to_string(),
        item_type: None,
      });
    }

    // --- Dispatch items → Outgoing ---
    for r in &dispatch {
      let (operation, flow_route) = match (r.dispatch_method, r.dispatch_purpose) {
        (enums::DispatchMethod::Truck, enums::DispatchPurpose::External) => {
          ("Truck Dispatch".to_string(), "/outgoing/truck")
        }
        (enums::DispatchMethod::Truck, enums::DispatchPurpose::Internal) => {
          ("Internal Truck Dispatch".to_string(), "/outgoing/truck")
        }
        (enums::DispatchMethod::VesselTerminal, _) => {
          ("Vessel/Terminal Dispatch".to_string(), "/outgoing/direct")
        }
        (enums::DispatchMethod::Bunkering, _) => ("Bunkering".to_string(), "/outgoing/bunkering"),
      };
      rows.push(CargoFlowFlatRow {
        id: r.item_id,
        document_id: r.document_id,
        document_number: r.document_number.clone(),
        date: r.date.clone(),
        flow_type: "Outgoing".to_string(),
        operation,
        contractor_name: r.contractor_id_name.clone(),
        status: format!("{:?}", r.status),
        flow_route: flow_route.to_string(),
        product_name: r.product_id_name.clone(),
        storage_name: r.storage_id_name.clone(),
        quantity: format!("-{}", r.dispatched_amount),
        item_type: None,
      });
    }

    // --- Physical Transfer items → Internal (two rows: outflow + inflow) ---
    for r in &physical {
      // Row 1: outflow from source storage
      rows.push(CargoFlowFlatRow {
        id: r.item_id,
        document_id: r.document_id,
        document_number: r.document_number.clone(),
        date: r.date.clone(),
        flow_type: "Internal".to_string(),
        operation: "Physical Transfer".to_string(),
        contractor_name: r.contractor_id_name.clone(),
        status: format!("{:?}", r.status),
        flow_route: "/internal/physical-transfer".to_string(),
        product_name: r.product_id_name.clone(),
        storage_name: r.from_storage_id_name.clone(),
        quantity: format!("-{}", r.amount),
        item_type: Some("outflow".into()),
      });
      // Row 2: inflow to destination storage
      rows.push(CargoFlowFlatRow {
        id: r.item_id,
        document_id: r.document_id,
        document_number: r.document_number.clone(),
        date: r.date.clone(),
        flow_type: "Internal".to_string(),
        operation: "Physical Transfer".to_string(),
        contractor_name: r.contractor_id_name.clone(),
        status: format!("{:?}", r.status),
        flow_route: "/internal/physical-transfer".to_string(),
        product_name: r.product_id_name.clone(),
        storage_name: r.to_storage_id_name.clone(),
        quantity: r.amount.to_string(),
        item_type: Some("inflow".into()),
      });
    }

    // --- Ownership Transfer items → Internal ---
    for r in &ownership {
      let contractor_name = format!(
        "{} \u{2192} {}",
        r.from_contractor_id_name, r.to_contractor_id_name
      );
      rows.push(CargoFlowFlatRow {
        id: r.item_id,
        document_id: r.document_id,
        document_number: format!("OT-{}", r.document_id.as_fields().0),
        date: r.date.clone(),
        flow_type: "Internal".to_string(),
        operation: "Ownership Transfer".to_string(),
        contractor_name,
        status: format!("{:?}", r.status),
        flow_route: "/internal/ownership-transfer".to_string(),
        product_name: r.product_id_name.clone(),
        storage_name: r.storage_id_name.clone(),
        quantity: r.amount.to_string(),
        item_type: None,
      });
    }

    // --- Blending items → Internal ---
    for r in &blending {
      // Components (consumed) are negative, results (produced) are positive
      let quantity = if r.item_type == "component" {
        format!("-{}", r.amount)
      } else {
        r.amount.to_string()
      };
      rows.push(CargoFlowFlatRow {
        id: r.item_id,
        document_id: r.document_id,
        document_number: r.document_number.clone(),
        date: r.date.clone(),
        flow_type: "Internal".to_string(),
        operation: "Blending".to_string(),
        contractor_name: r.contractor_id_name.clone(),
        status: format!("{:?}", r.status),
        flow_route: "/internal/blending".to_string(),
        product_name: r.product_id_name.clone(),
        storage_name: r.storage_id_name.clone(),
        quantity,
        item_type: Some(r.item_type.clone()),
      });
    }

    // --- Reconciliation adjustments → Internal ---
    for r in &reconciliation {
      // Loss is negative (inventory decreased), Surplus is positive (inventory increased)
      let quantity = if r.adjustment_type == enums::AdjustmentType::Loss {
        format!("-{}", r.amount)
      } else {
        r.amount.to_string()
      };
      rows.push(CargoFlowFlatRow {
        id: r.item_id,
        document_id: r.document_id,
        document_number: r.document_number.clone(),
        date: r.date.clone(),
        flow_type: "Internal".to_string(),
        operation: "Reconciliation".to_string(),
        contractor_name: r.contractor_id_name.clone(),
        status: format!("{:?}", r.status),
        flow_route: "/internal/reconciliation".to_string(),
        product_name: r.product_id_name.clone(),
        storage_name: r.storage_id_name.clone(),
        quantity,
        item_type: Some(format!("{:?}", r.adjustment_type)),
      });
    }

    // Sort by date descending (string comparison works for ISO dates)
    rows.sort_by(|a, b| b.date.cmp(&a.date));

    // In-memory pagination
    let start = ((page - 1) * per_page) as usize;
    let paginated = rows
      .into_iter()
      .skip(start)
      .take(per_page as usize)
      .collect();

    Ok(paginated)
  }
}
