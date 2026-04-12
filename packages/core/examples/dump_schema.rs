//! Creates an unencrypted SQLite file with the full Voletu schema.
//!
//! Used by the `er-diagram` pnpm script to produce a plain database that
//! `sea-orm-cli generate entity --er-diagram` can introspect without
//! SQLCipher support.

use sea_orm::{ConnectOptions, Database};
use voletu_core::entities::*;

/// Ensures all entity modules are linked into this binary.
///
/// The entity-registry feature registers entities via linker sections.
/// Because this example never references entity types through normal code
/// paths, the linker strips their object files (and the registry entries
/// within them). Touching one type per module prevents that.
fn ensure_entities_linked() {
  fn touch<T>() {
    std::hint::black_box(std::any::type_name::<T>());
  }
  touch::<acceptance_document::Entity>();
  touch::<acceptance_item::Entity>();
  touch::<dispatch_document::Entity>();
  touch::<dispatch_item::Entity>();
  touch::<dispatch_storage_measurement::Entity>();
  touch::<inventory_ledger_entry::Entity>();
  touch::<blending_component::Entity>();
  touch::<blending_document::Entity>();
  touch::<blending_result::Entity>();
  touch::<inventory_adjustment::Entity>();
  touch::<inventory_reconciliation::Entity>();
  touch::<ownership_transfer::Entity>();
  touch::<ownership_transfer_item::Entity>();
  touch::<physical_storage_transfer::Entity>();
  touch::<physical_transfer_item::Entity>();
  touch::<base::Entity>();
  touch::<company::Entity>();
  touch::<port::Entity>();
  touch::<product::Entity>();
  touch::<product_group::Entity>();
  touch::<product_type::Entity>();
  touch::<storage::Entity>();
  touch::<warehouse::Entity>();
  touch::<audit_log::Entity>();
  touch::<database_instance::Entity>();
  touch::<local::Entity>();
  touch::<node_base_assignment::Entity>();
  touch::<refresh_token::Entity>();
  touch::<role::Entity>();
  touch::<sync_watermark::Entity>();
  touch::<user::Entity>();
  touch::<rail_wagon_manifest::Entity>();
  touch::<rail_wagon_measurement::Entity>();
  touch::<rail_wagon_weight::Entity>();
  touch::<rail_waybill::Entity>();
  touch::<truck_waybill::Entity>();
  touch::<truck_waybill_item::Entity>();
  touch::<truck_weight_doc::Entity>();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  ensure_entities_linked();

  let path = std::path::Path::new("target/schema.db");

  // remove stale file so the schema is always fresh.
  if path.exists() {
    std::fs::remove_file(path)?;
  }

  let url = format!("sqlite://{}?mode=rwc", path.display());
  let db = Database::connect(ConnectOptions::new(&url)).await?;

  db.get_schema_registry("voletu-core::entities::*")
    .sync(&db)
    .await?;

  println!("Schema written to {}", path.display());
  Ok(())
}
