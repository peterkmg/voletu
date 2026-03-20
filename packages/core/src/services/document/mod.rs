use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::services::{audit::AuditService, ledger::LedgerService};

pub mod acceptance_doc;
pub mod acceptance_doc_extra;
pub mod acceptance_item;
pub mod blending_component;
pub mod blending_doc;
pub mod blending_doc_extra;
pub mod blending_result;
pub mod dispatch_doc;
pub mod dispatch_doc_extra;
pub mod dispatch_item;
pub mod dispatch_measurement;
pub mod ownership_doc;
pub mod ownership_doc_extra;
pub mod ownership_item;
pub mod physical_doc;
pub mod physical_doc_extra;
pub mod physical_item;
pub mod rail_manifest;
pub mod rail_measurement;
pub mod rail_waybill;
pub mod rail_waybill_extra;
pub mod rail_weight;
pub mod reconciliation_adjustment;
pub mod reconciliation_doc;
pub mod reconciliation_doc_extra;
pub mod resolve;
pub mod truck_waybill;
pub mod truck_waybill_extra;
pub mod truck_waybill_item;
pub mod truck_weight_doc;

pub struct DocumentService {
  pub(super) db: Arc<DatabaseConnection>,
  pub(super) ledger: Arc<LedgerService>,
  pub(super) audit: Arc<AuditService>,
}

impl DocumentService {
  pub fn new(
    db: Arc<DatabaseConnection>,
    ledger: Arc<LedgerService>,
    audit: Arc<AuditService>,
  ) -> Self {
    Self { db, ledger, audit }
  }
}
