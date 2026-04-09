use voletu_core_macros::response_dto;

/// Functional DTO returned by the debug seed endpoint.
#[response_dto]
pub struct SeedResult {
  pub product_types: usize,
  pub product_groups: usize,
  pub products: usize,
  pub companies: usize,
  pub ports: usize,
  pub bases: usize,
  pub warehouses: usize,
  pub storages: usize,
  pub users: usize,
  pub truck_waybills: usize,
  pub rail_waybills: usize,
  pub acceptance_docs: usize,
  pub dispatch_docs: usize,
  pub blending_docs: usize,
  pub ownership_transfers: usize,
  pub physical_transfers: usize,
  pub reconciliations: usize,
  pub ledger_entries: usize,
}
