use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::{
  config::ApiConfig,
  services::{
    audit::AuditService,
    catalog::CatalogService as DomainCatalogService,
    document::DocumentService,
    flow::FlowService,
    ledger::LedgerService,
    sync::SyncService,
    system::SystemService,
  },
};

pub struct ApiServices {
  pub system: Arc<SystemService>,
  pub catalog_service: Arc<DomainCatalogService>,
  pub document: Arc<DocumentService>,
  pub flow: Arc<FlowService>,
  pub ledger: Arc<LedgerService>,
  pub audit: Arc<AuditService>,
  pub sync: Arc<SyncService>,
}

impl ApiServices {
  pub fn new(db: Arc<DatabaseConnection>, cfg: Arc<ApiConfig>) -> Self {
    let audit = Arc::new(AuditService::new(cfg.clone()));
    let system = Arc::new(SystemService::new(db.clone(), cfg.clone()));
    let ledger = Arc::new(LedgerService::new(db.clone()));
    let catalog_service = Arc::new(DomainCatalogService::new(db.clone(), audit.clone()));
    let document = Arc::new(DocumentService::new(
      db.clone(),
      ledger.clone(),
      audit.clone(),
    ));
    let flow = Arc::new(FlowService::new(db.clone()));
    let sync = Arc::new(SyncService::new(db.clone(), cfg));

    Self {
      system,
      catalog_service,
      document,
      flow,
      ledger,
      audit,
      sync,
    }
  }
}
