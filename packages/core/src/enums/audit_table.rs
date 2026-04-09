use sea_orm::{prelude::StringLen, DeriveActiveEnum, EntityName, EnumIter, ModelTrait};
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(
  Clone,
  Copy,
  Debug,
  PartialEq,
  Eq,
  Hash,
  EnumString,
  EnumIter,
  DeriveActiveEnum,
  Serialize,
  Deserialize,
  utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[sea_orm(
  rs_type = "String",
  db_type = "String(StringLen::None)",
  rename_all = "snake_case"
)]
pub enum AuditTable {
  AcceptanceDocuments,
  AcceptanceItems,
  AuditLogs,
  Bases,
  BlendingComponents,
  BlendingDocuments,
  BlendingResults,
  Companies,
  DatabaseInstances,
  DispatchDocuments,
  DispatchItems,
  DispatchStorageMeasurements,
  InventoryAdjustments,
  InventoryLedgerEntries,
  InventoryReconciliations,
  Local,
  OwnershipTransfers,
  OwnershipTransferItems,
  PhysicalStorageTransfers,
  PhysicalTransferItems,
  Ports,
  Products,
  ProductGroups,
  ProductTypes,
  RailWagonManifests,
  RailWagonMeasurements,
  RailWagonWeights,
  RailWaybills,
  RefreshTokens,
  Roles,
  Storages,
  SyncWatermarks,
  TruckWaybills,
  TruckWaybillItems,
  TruckWeightDocs,
  Users,
  Warehouses,
}

impl AuditTable {
  pub(crate) fn resolve(table_name: &str) -> Option<Self> {
    table_name.parse::<Self>().ok()
  }

  pub(crate) fn for_entity_name(table_name: &str) -> Option<Self> {
    Self::resolve(table_name)
  }

  pub(crate) fn for_model<M: ModelTrait>() -> Option<Self> {
    Self::for_entity_name(<<M as ModelTrait>::Entity as EntityName>::table_name(
      &Default::default(),
    ))
  }

  pub fn table_name(self) -> &'static str {
    match self {
      Self::AcceptanceDocuments => "acceptance_documents",
      Self::AcceptanceItems => "acceptance_items",
      Self::AuditLogs => "audit_logs",
      Self::Bases => "bases",
      Self::BlendingComponents => "blending_components",
      Self::BlendingDocuments => "blending_documents",
      Self::BlendingResults => "blending_results",
      Self::Companies => "companies",
      Self::DatabaseInstances => "database_instances",
      Self::DispatchDocuments => "dispatch_documents",
      Self::DispatchItems => "dispatch_items",
      Self::DispatchStorageMeasurements => "dispatch_storage_measurements",
      Self::InventoryAdjustments => "inventory_adjustments",
      Self::InventoryLedgerEntries => "inventory_ledger_entries",
      Self::InventoryReconciliations => "inventory_reconciliations",
      Self::Local => "local",
      Self::OwnershipTransfers => "ownership_transfers",
      Self::OwnershipTransferItems => "ownership_transfer_items",
      Self::PhysicalStorageTransfers => "physical_storage_transfers",
      Self::PhysicalTransferItems => "physical_transfer_items",
      Self::Ports => "ports",
      Self::Products => "products",
      Self::ProductGroups => "product_groups",
      Self::ProductTypes => "product_types",
      Self::RailWagonManifests => "rail_wagon_manifests",
      Self::RailWagonMeasurements => "rail_wagon_measurements",
      Self::RailWagonWeights => "rail_wagon_weights",
      Self::RailWaybills => "rail_waybills",
      Self::RefreshTokens => "refresh_tokens",
      Self::Roles => "roles",
      Self::Storages => "storages",
      Self::SyncWatermarks => "sync_watermarks",
      Self::TruckWaybills => "truck_waybills",
      Self::TruckWaybillItems => "truck_waybill_items",
      Self::TruckWeightDocs => "truck_weight_docs",
      Self::Users => "users",
      Self::Warehouses => "warehouses",
    }
  }

  pub fn sync_excluded_tables() -> &'static [Self] {
    &[Self::Local, Self::Roles]
  }

  pub fn sync_global_tables() -> &'static [Self] {
    &[
      Self::Companies,
      Self::Products,
      Self::ProductGroups,
      Self::ProductTypes,
      Self::Bases,
      Self::Warehouses,
      Self::Storages,
      Self::Ports,
      Self::Users,
      Self::DatabaseInstances,
    ]
  }
}
