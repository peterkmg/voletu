use strum::EnumString;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumString)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum AuditTable {
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
}
