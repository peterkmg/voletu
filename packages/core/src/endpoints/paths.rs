pub mod health {
  pub const ROOT: &str = "/health";
}

pub mod auth {
  pub const LOGIN: &str = "/auth/login";
  pub const REFRESH: &str = "/auth/refresh";
  pub const CHANGE_PASSWORD: &str = "/auth/change-password";
  pub const ME: &str = "/auth/me";
}

pub mod node {
  pub const INITIALIZE: &str = "/node/initialize";
  pub const RESTART: &str = "/node/restart";
  pub const STATUS: &str = "/node/status";
}

pub mod users {
  pub const ROOT: &str = "/users";
  pub const BY_ID: &str = "/users/{id}";
}

pub mod catalog {
  pub const COMPANIES: &str = "/catalog/companies";
  pub const COMPANIES_BY_ID: &str = "/catalog/companies/{id}";
  pub const COMPANIES_HARD_DELETE_BY_ID: &str = "/catalog/companies/{id}/hard";
  pub const COMPANIES_RESTORE_BY_ID: &str = "/catalog/companies/{id}/restore";
  pub const PRODUCT_TYPES: &str = "/catalog/product-types";
  pub const PRODUCT_TYPES_BY_ID: &str = "/catalog/product-types/{id}";
  pub const PRODUCT_TYPES_HARD_DELETE_BY_ID: &str = "/catalog/product-types/{id}/hard";
  pub const PRODUCT_TYPES_RESTORE_BY_ID: &str = "/catalog/product-types/{id}/restore";
  pub const PRODUCT_GROUPS: &str = "/catalog/product-groups";
  pub const PRODUCT_GROUPS_BY_ID: &str = "/catalog/product-groups/{id}";
  pub const PRODUCT_GROUPS_HARD_DELETE_BY_ID: &str = "/catalog/product-groups/{id}/hard";
  pub const PRODUCT_GROUPS_RESTORE_BY_ID: &str = "/catalog/product-groups/{id}/restore";
  pub const PRODUCTS: &str = "/catalog/products";
  pub const PRODUCTS_BY_ID: &str = "/catalog/products/{id}";
  pub const PRODUCTS_HARD_DELETE_BY_ID: &str = "/catalog/products/{id}/hard";
  pub const PRODUCTS_RESTORE_BY_ID: &str = "/catalog/products/{id}/restore";
  pub const BASES: &str = "/catalog/bases";
  pub const BASES_BY_ID: &str = "/catalog/bases/{id}";
  pub const BASES_HARD_DELETE_BY_ID: &str = "/catalog/bases/{id}/hard";
  pub const BASES_RESTORE_BY_ID: &str = "/catalog/bases/{id}/restore";
  pub const WAREHOUSES: &str = "/catalog/warehouses";
  pub const WAREHOUSES_BY_ID: &str = "/catalog/warehouses/{id}";
  pub const WAREHOUSES_HARD_DELETE_BY_ID: &str = "/catalog/warehouses/{id}/hard";
  pub const WAREHOUSES_RESTORE_BY_ID: &str = "/catalog/warehouses/{id}/restore";
  pub const STORAGES: &str = "/catalog/storages";
  pub const STORAGES_BY_ID: &str = "/catalog/storages/{id}";
  pub const STORAGES_HARD_DELETE_BY_ID: &str = "/catalog/storages/{id}/hard";
  pub const STORAGES_RESTORE_BY_ID: &str = "/catalog/storages/{id}/restore";
  pub const PORTS: &str = "/catalog/ports";
  pub const PORTS_BY_ID: &str = "/catalog/ports/{id}";
  pub const PORTS_HARD_DELETE_BY_ID: &str = "/catalog/ports/{id}/hard";
  pub const PORTS_RESTORE_BY_ID: &str = "/catalog/ports/{id}/restore";
}

pub mod acceptance {
  pub const ROOT: &str = "/acceptance";
  pub const BY_ID: &str = "/acceptance/{id}";
  pub const HARD_DELETE_BY_ID: &str = "/acceptance/{id}/hard";
  pub const SAVE: &str = "/acceptance/save";
  pub const SAVE_AND_EXECUTE: &str = "/acceptance/save-and-execute";
  pub const QUERY: &str = "/acceptance/query";
  pub const EXECUTE_BY_ID: &str = "/acceptance/execute/{id}";
  pub const REVERT_BY_ID: &str = "/acceptance/revert/{id}";
  pub const ITEMS: &str = "/acceptance/items";
  pub const ITEMS_BY_ID: &str = "/acceptance/items/{id}";
  pub const ITEMS_HARD_DELETE_BY_ID: &str = "/acceptance/items/{id}/hard";
  pub const COMPOSITE_BY_ID: &str = "/acceptance/composite/{id}";
  pub const COMPOSITE_SAVE: &str = "/acceptance/composite/save";
  pub const COMPOSITE_SAVE_AND_EXECUTE: &str = "/acceptance/composite/save-and-execute";
}

pub mod dispatch {
  pub const ROOT: &str = "/dispatch";
  pub const BY_ID: &str = "/dispatch/{id}";
  pub const HARD_DELETE_BY_ID: &str = "/dispatch/{id}/hard";
  pub const SAVE: &str = "/dispatch/save";
  pub const SAVE_AND_EXECUTE: &str = "/dispatch/save-and-execute";
  pub const QUERY: &str = "/dispatch/query";
  pub const EXECUTE_BY_ID: &str = "/dispatch/execute/{id}";
  pub const REVERT_BY_ID: &str = "/dispatch/revert/{id}";
  pub const ITEMS: &str = "/dispatch/items";
  pub const ITEMS_BY_ID: &str = "/dispatch/items/{id}";
  pub const ITEMS_HARD_DELETE_BY_ID: &str = "/dispatch/items/{id}/hard";
  pub const STORAGE_MEASUREMENTS: &str = "/dispatch/storage-measurements";
  pub const STORAGE_MEASUREMENTS_BY_ID: &str = "/dispatch/storage-measurements/{id}";
  pub const STORAGE_MEASUREMENTS_HARD_DELETE_BY_ID: &str =
    "/dispatch/storage-measurements/{id}/hard";
  pub const COMPOSITE_BY_ID: &str = "/dispatch/composite/{id}";
  pub const COMPOSITE_SAVE: &str = "/dispatch/composite/save";
  pub const COMPOSITE_SAVE_AND_EXECUTE: &str = "/dispatch/composite/save-and-execute";
}

pub mod blending {
  pub const ROOT: &str = "/blending";
  pub const BY_ID: &str = "/blending/{id}";
  pub const HARD_DELETE_BY_ID: &str = "/blending/{id}/hard";
  pub const SAVE: &str = "/blending/save";
  pub const SAVE_AND_EXECUTE: &str = "/blending/save-and-execute";
  pub const QUERY: &str = "/blending/query";
  pub const EXECUTE_BY_ID: &str = "/blending/execute/{id}";
  pub const REVERT_BY_ID: &str = "/blending/revert/{id}";
  pub const COMPONENTS: &str = "/blending/components";
  pub const COMPONENTS_BY_ID: &str = "/blending/components/{id}";
  pub const COMPONENTS_HARD_DELETE_BY_ID: &str = "/blending/components/{id}/hard";
  pub const RESULTS: &str = "/blending/results";
  pub const RESULTS_BY_ID: &str = "/blending/results/{id}";
  pub const RESULTS_HARD_DELETE_BY_ID: &str = "/blending/results/{id}/hard";
  pub const COMPOSITE_BY_ID: &str = "/blending/composite/{id}";
  pub const COMPOSITE_SAVE: &str = "/blending/composite/save";
  pub const COMPOSITE_SAVE_AND_EXECUTE: &str = "/blending/composite/save-and-execute";
}

pub mod operations {
  pub const PHYSICAL_TRANSFERS: &str = "/physical-transfers";
  pub const PHYSICAL_TRANSFER_DOCUMENTS: &str = "/physical-transfers/documents";
  pub const PHYSICAL_TRANSFER_DOCUMENTS_BY_ID: &str = "/physical-transfers/documents/{id}";
  pub const PHYSICAL_TRANSFER_DOCUMENTS_HARD_DELETE_BY_ID: &str =
    "/physical-transfers/documents/{id}/hard";
  pub const PHYSICAL_TRANSFER_DOCUMENTS_SAVE_AND_EXECUTE: &str =
    "/physical-transfers/documents/save-and-execute";
  pub const PHYSICAL_TRANSFER_DOCUMENTS_EXECUTE_BY_ID: &str =
    "/physical-transfers/documents/execute/{id}";
  pub const PHYSICAL_TRANSFER_DOCUMENTS_REVERT_BY_ID: &str =
    "/physical-transfers/documents/revert/{id}";
  pub const PHYSICAL_TRANSFERS_BY_ID: &str = "/physical-transfers/{id}";
  pub const PHYSICAL_TRANSFERS_HARD_DELETE_BY_ID: &str = "/physical-transfers/{id}/hard";
  pub const PHYSICAL_TRANSFERS_QUERY: &str = "/physical-transfers/query";
  pub const PHYSICAL_TRANSFERS_COMPOSITE_BY_ID: &str = "/physical-transfers/composite/{id}";
  pub const PHYSICAL_TRANSFERS_SAVE: &str = "/physical-transfers/save";
  pub const PHYSICAL_TRANSFERS_SAVE_AND_EXECUTE: &str = "/physical-transfers/save-and-execute";
  pub const PHYSICAL_TRANSFERS_EXECUTE_BY_ID: &str = "/physical-transfers/execute/{id}";
  pub const PHYSICAL_TRANSFERS_REVERT_BY_ID: &str = "/physical-transfers/revert/{id}";
  pub const PHYSICAL_TRANSFER_ITEMS: &str = "/physical-transfers/items";
  pub const PHYSICAL_TRANSFER_ITEMS_BY_ID: &str = "/physical-transfers/items/{id}";
  pub const PHYSICAL_TRANSFER_ITEMS_HARD_DELETE_BY_ID: &str = "/physical-transfers/items/{id}/hard";

  pub const OWNERSHIP_TRANSFERS: &str = "/ownership-transfers";
  pub const OWNERSHIP_TRANSFER_DOCUMENTS: &str = "/ownership-transfers/documents";
  pub const OWNERSHIP_TRANSFER_DOCUMENTS_BY_ID: &str = "/ownership-transfers/documents/{id}";
  pub const OWNERSHIP_TRANSFER_DOCUMENTS_HARD_DELETE_BY_ID: &str =
    "/ownership-transfers/documents/{id}/hard";
  pub const OWNERSHIP_TRANSFER_DOCUMENTS_SAVE_AND_EXECUTE: &str =
    "/ownership-transfers/documents/save-and-execute";
  pub const OWNERSHIP_TRANSFER_DOCUMENTS_EXECUTE_BY_ID: &str =
    "/ownership-transfers/documents/execute/{id}";
  pub const OWNERSHIP_TRANSFER_DOCUMENTS_REVERT_BY_ID: &str =
    "/ownership-transfers/documents/revert/{id}";
  pub const OWNERSHIP_TRANSFERS_BY_ID: &str = "/ownership-transfers/{id}";
  pub const OWNERSHIP_TRANSFERS_HARD_DELETE_BY_ID: &str = "/ownership-transfers/{id}/hard";
  pub const OWNERSHIP_TRANSFERS_QUERY: &str = "/ownership-transfers/query";
  pub const OWNERSHIP_TRANSFERS_COMPOSITE_BY_ID: &str = "/ownership-transfers/composite/{id}";
  pub const OWNERSHIP_TRANSFERS_SAVE: &str = "/ownership-transfers/save";
  pub const OWNERSHIP_TRANSFERS_SAVE_AND_EXECUTE: &str = "/ownership-transfers/save-and-execute";
  pub const OWNERSHIP_TRANSFERS_EXECUTE_BY_ID: &str = "/ownership-transfers/execute/{id}";
  pub const OWNERSHIP_TRANSFERS_REVERT_BY_ID: &str = "/ownership-transfers/revert/{id}";
  pub const OWNERSHIP_TRANSFER_ITEMS: &str = "/ownership-transfers/items";
  pub const OWNERSHIP_TRANSFER_ITEMS_BY_ID: &str = "/ownership-transfers/items/{id}";
  pub const OWNERSHIP_TRANSFER_ITEMS_HARD_DELETE_BY_ID: &str =
    "/ownership-transfers/items/{id}/hard";

  pub const RECONCILIATIONS: &str = "/reconciliations";
  pub const RECONCILIATIONS_BY_ID: &str = "/reconciliations/{id}";
  pub const RECONCILIATIONS_HARD_DELETE_BY_ID: &str = "/reconciliations/{id}/hard";
  pub const RECONCILIATIONS_QUERY: &str = "/reconciliations/query";
  pub const RECONCILIATIONS_SAVE: &str = "/reconciliations/save";
  pub const RECONCILIATIONS_SAVE_AND_EXECUTE: &str = "/reconciliations/save-and-execute";
  pub const RECONCILIATIONS_EXECUTE_BY_ID: &str = "/reconciliations/execute/{id}";
  pub const RECONCILIATIONS_REVERT_BY_ID: &str = "/reconciliations/revert/{id}";
  pub const RECONCILIATION_ADJUSTMENTS: &str = "/reconciliations/adjustments";
  pub const RECONCILIATION_ADJUSTMENTS_BY_ID: &str = "/reconciliations/adjustments/{id}";
  pub const RECONCILIATION_ADJUSTMENTS_HARD_DELETE_BY_ID: &str =
    "/reconciliations/adjustments/{id}/hard";
  pub const RECONCILIATION_ADJUSTMENTS_SAVE: &str = "/reconciliations/adjustments/save";
}

pub mod transport {
  pub mod truck {
    pub const WAYBILLS: &str = "/transport/truck/waybills";
    pub const WAYBILLS_BY_ID: &str = "/transport/truck/waybills/{id}";
    pub const WAYBILLS_HARD_DELETE_BY_ID: &str = "/transport/truck/waybills/{id}/hard";
    pub const ITEMS: &str = "/transport/truck/items";
    pub const ITEMS_BY_ID: &str = "/transport/truck/items/{id}";
    pub const ITEMS_HARD_DELETE_BY_ID: &str = "/transport/truck/items/{id}/hard";
    pub const WEIGHT_DOCS: &str = "/transport/truck/weight-docs";
    pub const WEIGHT_DOCS_BY_ID: &str = "/transport/truck/weight-docs/{id}";
    pub const WEIGHT_DOCS_HARD_DELETE_BY_ID: &str = "/transport/truck/weight-docs/{id}/hard";
    pub const COMPOSITE_CREATE: &str = "/transport/truck/save";
    pub const SAVE: &str = COMPOSITE_CREATE;
  }

  pub mod rail {
    pub const WAYBILLS: &str = "/transport/rail/waybills";
    pub const WAYBILLS_BY_ID: &str = "/transport/rail/waybills/{id}";
    pub const WAYBILLS_HARD_DELETE_BY_ID: &str = "/transport/rail/waybills/{id}/hard";
    pub const MANIFESTS: &str = "/transport/rail/manifests";
    pub const MANIFESTS_BY_ID: &str = "/transport/rail/manifests/{id}";
    pub const MANIFESTS_HARD_DELETE_BY_ID: &str = "/transport/rail/manifests/{id}/hard";
    pub const MEASUREMENTS: &str = "/transport/rail/measurements";
    pub const MEASUREMENTS_BY_ID: &str = "/transport/rail/measurements/{id}";
    pub const MEASUREMENTS_HARD_DELETE_BY_ID: &str = "/transport/rail/measurements/{id}/hard";
    pub const WEIGHTS: &str = "/transport/rail/weights";
    pub const WEIGHTS_BY_ID: &str = "/transport/rail/weights/{id}";
    pub const WEIGHTS_HARD_DELETE_BY_ID: &str = "/transport/rail/weights/{id}/hard";
    pub const COMPOSITE_CREATE: &str = "/transport/rail/save";
    pub const SAVE: &str = COMPOSITE_CREATE;
  }
}

pub mod ledger {
  pub const ROOT: &str = "/ledger";
  pub const QUERY: &str = "/ledger/query";
}

pub mod sync {
  pub const AUDIT_LOGS: &str = "/audit-logs";
  pub const OUTBOUND: &str = "/sync/outbound";
  pub const PULL: &str = "/sync/pull";
  pub const PUSH: &str = "/sync/push";
  pub const STATUS: &str = "/sync/status";
  pub const WATERMARKS: &str = "/sync/watermarks";
}

pub mod docs {
  pub const SWAGGER_UI: &str = "/swagger-ui";
  pub const OPENAPI_JSON: &str = "/api-docs/openapi.json";
}
