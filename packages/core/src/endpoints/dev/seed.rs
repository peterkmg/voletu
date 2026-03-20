use std::sync::Arc;

use axum::extract::State;
use chrono::{Duration, Utc};
use rand::{Rng, SeedableRng};
use sea_orm::{prelude::Decimal, ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
  api::{response::ApiResponse, result::ApiResult, state::ApiState, ApiError},
  context::audit::with_audit_context,
  entities::{
    acceptance_document, acceptance_item, base, blending_component, blending_document,
    blending_result, company, dispatch_document, dispatch_item, inventory_adjustment,
    inventory_reconciliation, local, ownership_transfer, ownership_transfer_item,
    physical_storage_transfer, physical_transfer_item, port, product, product_group, product_type,
    rail_waybill, storage, truck_waybill, user, warehouse,
  },
  enums::{AdjustmentType, ArrivalType, DispatchMethod, DispatchPurpose, DocumentStatus, RoleType},
  utils::password::hash_password,
};
use utoipa_axum::{router::OpenApiRouter, routes};

// ---------------------------------------------------------------------------
// Response type
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, ToSchema)]
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
}

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------

#[utoipa::path(
  post,
  tag = "Dev",
  operation_id = "dev_seed",
  summary = "Seed database with fake data",
  description = "Populates the database with realistic fake data for development. Additive — can be called multiple times. Only available in debug builds.",
  path = "/dev/seed",
  responses((status = 200, body = ApiResponse<SeedResult>))
)]
#[axum::debug_handler]
async fn dev_seed(State(state): State<Arc<ApiState>>) -> ApiResult<SeedResult> {
  let local_cfg = local::Entity::find()
    .one(&*state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("local config not found".to_string()))?;

  let actor_id = Uuid::now_v7();
  let origin_db_id = local_cfg.local_db_id;

  let result = with_audit_context(actor_id, origin_db_id, || async {
    do_seed(&state.db).await
  })
  .await?;

  Ok(ApiResponse::success(result))
}

pub fn dev_routes(state: Arc<ApiState>) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(dev_seed))
    .with_state(state)
}

// ---------------------------------------------------------------------------
// Static reference data
// ---------------------------------------------------------------------------

const PRODUCT_TYPES: &[&str] = &[
  "Crude Oil",
  "Gasoline",
  "Diesel",
  "Jet Fuel",
  "Lubricants",
  "LPG",
  "Bitumen",
  "Naphtha",
];

const PRODUCT_GROUP_SUFFIXES: &[&str] = &["Premium", "Standard", "Commercial"];
const PRODUCT_VARIANTS: &[&str] = &["Grade A", "Grade B", "Grade C"];

const COMPANY_PREFIXES: &[&str] = &[
  "Global", "Atlantic", "Pacific", "Trans", "Euro", "Eastern", "Western", "Northern", "Petro",
  "Alpha",
];
const COMPANY_SUFFIXES: &[&str] = &[
  "Energy", "Fuels", "Trading", "Logistics", "Resources", "Oil", "Gas", "Petroleum", "Commodities",
  "Chemicals",
];

const PORTS: &[(&str, &str)] = &[
  ("Rotterdam", "Netherlands"),
  ("Hamburg", "Germany"),
  ("Singapore", "Singapore"),
  ("Houston", "USA"),
  ("Shanghai", "China"),
  ("Antwerp", "Belgium"),
  ("Dubai", "UAE"),
  ("New Orleans", "USA"),
  ("Marseille", "France"),
  ("Odessa", "Ukraine"),
  ("Novorossiysk", "Russia"),
  ("Ust-Luga", "Russia"),
  ("Primorsk", "Russia"),
  ("Ventspils", "Latvia"),
  ("Tallinn", "Estonia"),
  ("Klaipeda", "Lithuania"),
  ("Gdansk", "Poland"),
  ("Amsterdam", "Netherlands"),
  ("Fujairah", "UAE"),
  ("Algeciras", "Spain"),
];

const BASE_NAMES: &[&str] = &[
  "Northern Terminal",
  "Eastern Depot",
  "Western Storage Base",
  "Southern Terminal",
  "Central Tank Farm",
  "Port-Side Terminal",
  "Inland Distribution Base",
  "Rail-Junction Terminal",
];

const WAREHOUSE_SUFFIXES: &[&str] = &["Section A", "Section B", "Section C"];

// ---------------------------------------------------------------------------
// Core seeding logic
// ---------------------------------------------------------------------------

async fn do_seed(db: &DatabaseConnection) -> Result<SeedResult, ApiError> {
  let mut rng = rand::rngs::StdRng::seed_from_u64(Utc::now().timestamp_millis() as u64);
  let now = Utc::now();

  // Pre-compute password hash once (argon2 is intentionally slow)
  let dev_password_hash = hash_password("password123")
    .await
    .map_err(|e| ApiError::Internal(e))?;

  // ── Reference: product hierarchy ─────────────────────────────────────────

  let mut ptype_ids: Vec<Uuid> = Vec::new();
  for &name in PRODUCT_TYPES {
    let m = product_type::ActiveModel {
      common_name: Set(name.to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(db)
    .await?;
    ptype_ids.push(m.id);
  }

  let mut pgroup_ids: Vec<Uuid> = Vec::new();
  for &ptype_id in &ptype_ids {
    for &suffix in PRODUCT_GROUP_SUFFIXES {
      let ptype_name = PRODUCT_TYPES[ptype_ids.iter().position(|&x| x == ptype_id).unwrap()];
      let m = product_group::ActiveModel {
        product_type_id: Set(ptype_id),
        common_name: Set(format!("{} {}", ptype_name, suffix)),
        long_name: Set(None),
        ..Default::default()
      }
      .insert(db)
      .await?;
      pgroup_ids.push(m.id);
    }
  }

  let mut product_ids: Vec<Uuid> = Vec::new();
  for (i, &pgroup_id) in pgroup_ids.iter().enumerate() {
    for (j, &variant) in PRODUCT_VARIANTS.iter().enumerate() {
      let m = product::ActiveModel {
        product_group_id: Set(pgroup_id),
        manufacturer_id: Set(None),
        common_name: Set(format!("PG-{} {}", i + 1, variant)),
        long_name: Set(None),
        add_identification: Set(None),
        is_component: Set(j < 2), // Grade A and B are components; Grade C is blended product
        ..Default::default()
      }
      .insert(db)
      .await?;
      product_ids.push(m.id);
    }
  }

  // ── Reference: companies ──────────────────────────────────────────────────

  let mut contractor_ids: Vec<Uuid> = Vec::new();
  let mut sender_ids: Vec<Uuid> = Vec::new();
  let mut all_company_ids: Vec<Uuid> = Vec::new();

  for i in 0..30usize {
    let prefix = COMPANY_PREFIXES[i % COMPANY_PREFIXES.len()];
    let suffix = COMPANY_SUFFIXES[(i / COMPANY_PREFIXES.len()) % COMPANY_SUFFIXES.len()];
    let name = format!("{} {} {}", prefix, suffix, i + 1);

    let role_bucket = i % 5;
    let is_contractor = role_bucket == 0 || role_bucket == 1 || role_bucket == 4;
    let is_exporter = role_bucket == 2 || role_bucket == 4;
    let is_manufacturer = role_bucket == 3;
    let is_sender = role_bucket == 1 || role_bucket == 4;

    let m = company::ActiveModel {
      common_name: Set(name),
      legal_name: Set(None),
      is_contractor: Set(is_contractor),
      is_exporter: Set(is_exporter),
      is_manufacturer: Set(is_manufacturer),
      is_sender: Set(is_sender),
      ..Default::default()
    }
    .insert(db)
    .await?;

    all_company_ids.push(m.id);
    if is_contractor {
      contractor_ids.push(m.id);
    }
    if is_sender {
      sender_ids.push(m.id);
    }
  }

  // ── Reference: ports ──────────────────────────────────────────────────────

  let mut port_ids: Vec<Uuid> = Vec::new();
  for &(city, country) in PORTS {
    let m = port::ActiveModel {
      common_name: Set(city.to_string()),
      country: Set(Some(country.to_string())),
      ..Default::default()
    }
    .insert(db)
    .await?;
    port_ids.push(m.id);
  }

  // ── Reference: topology (bases → warehouses → storages) ──────────────────

  let mut base_ids: Vec<Uuid> = Vec::new();
  let mut warehouse_ids: Vec<Uuid> = Vec::new();
  let mut storage_ids: Vec<Uuid> = Vec::new();

  for &base_name in BASE_NAMES {
    let b = base::ActiveModel {
      common_name: Set(base_name.to_string()),
      long_name: Set(None),
      ..Default::default()
    }
    .insert(db)
    .await?;
    base_ids.push(b.id);

    for &wh_suffix in WAREHOUSE_SUFFIXES {
      let wh = warehouse::ActiveModel {
        base_id: Set(b.id),
        common_name: Set(format!("{} — {}", base_name, wh_suffix)),
        long_name: Set(None),
        ..Default::default()
      }
      .insert(db)
      .await?;
      warehouse_ids.push(wh.id);

      for tank_n in 1..=4usize {
        let capacity_val = 5_000u64 + (rng.next_u64() % 45_001);
        let s = storage::ActiveModel {
          warehouse_id: Set(wh.id),
          common_name: Set(format!("Tank {}", tank_n)),
          long_name: Set(None),
          capacity: Set(Some(Decimal::from(capacity_val))),
          is_type_specific: Set(tank_n == 1), // first tank per warehouse is type-specific
          product_type_id: Set(if tank_n == 1 {
            Some(ptype_ids[(rng.next_u64() as usize) % ptype_ids.len()])
          } else {
            None
          }),
          ..Default::default()
        }
        .insert(db)
        .await?;
        storage_ids.push(s.id);
      }
    }
  }

  // ── Users ─────────────────────────────────────────────────────────────────

  let extra_roles = [
    RoleType::SeniorSupervisor,
    RoleType::Supervisor,
    RoleType::Operator,
  ];
  for i in 0..8usize {
    let role = extra_roles[i % extra_roles.len()];
    user::ActiveModel {
      username: Set(format!("devuser{}", i + 1)),
      fullname: Set(Some(format!("Dev User {}", i + 1))),
      password_hash: Set(dev_password_hash.clone()),
      role_id: Set(role.uuid()),
      home_base_id: Set(Some(base_ids[i % base_ids.len()])),
      ..Default::default()
    }
    .insert(db)
    .await?;
  }

  // ── Transport: truck waybills ─────────────────────────────────────────────

  let mut truck_ids: Vec<Uuid> = Vec::new();
  for i in 0..300usize {
    let days_ago = (rng.next_u64() % 730) as i64;
    let date = (now - Duration::days(days_ago)).date_naive();
    let sender_id = sender_ids[(rng.next_u64() as usize) % sender_ids.len()];
    let m = truck_waybill::ActiveModel {
      document_number: Set(format!("TW-{:06}", 100_000 + i)),
      date: Set(date),
      sender_id: Set(sender_id),
      ..Default::default()
    }
    .insert(db)
    .await?;
    truck_ids.push(m.id);
  }

  // ── Transport: rail waybills ──────────────────────────────────────────────

  let mut rail_ids: Vec<Uuid> = Vec::new();
  for i in 0..200usize {
    let days_ago = (rng.next_u64() % 730) as i64;
    let date = (now - Duration::days(days_ago)).date_naive();
    let sender_id = sender_ids[(rng.next_u64() as usize) % sender_ids.len()];
    let m = rail_waybill::ActiveModel {
      document_number: Set(format!("RW-{:06}", 100_000 + i)),
      date: Set(date),
      sender_id: Set(sender_id),
      ..Default::default()
    }
    .insert(db)
    .await?;
    rail_ids.push(m.id);
  }

  // ── Documents: acceptance ─────────────────────────────────────────────────

  let arrival_types = [ArrivalType::Truck, ArrivalType::Rail, ArrivalType::External];
  let mut acceptance_count = 0usize;

  for i in 0..500usize {
    let days_ago = (rng.next_u64() % 730) as i64;
    let doc_date = now - Duration::days(days_ago);
    let arrival_type = arrival_types[(rng.next_u64() as usize) % arrival_types.len()];

    let doc = acceptance_document::ActiveModel {
      document_number: Set(format!("ACC-{:06}", 100_000 + i)),
      date_accepted: Set(doc_date),
      status: Set(DocumentStatus::Posted),
      version: Set(1),
      executed_at: Set(Some(doc_date + Duration::hours(1))),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      arrival_type: Set(arrival_type),
      source_entity: Set(None),
      truck_waybill_id: Set(None),
      rail_waybill_id: Set(None),
      transit_dispatch_id: Set(None),
      ..Default::default()
    }
    .insert(db)
    .await?;
    acceptance_count += 1;

    // 2–3 items per document
    let item_count = 2 + (rng.next_u64() % 2) as usize;
    for _ in 0..item_count {
      let product_id = product_ids[(rng.next_u64() as usize) % product_ids.len()];
      let contractor_id = contractor_ids[(rng.next_u64() as usize) % contractor_ids.len()];
      let storage_id = storage_ids[(rng.next_u64() as usize) % storage_ids.len()];
      let amount = 100u64 + rng.next_u64() % 9_900;

      acceptance_item::ActiveModel {
        acceptance_doc_id: Set(doc.id),
        product_id: Set(product_id),
        contractor_id: Set(contractor_id),
        storage_id: Set(storage_id),
        accepted_amount: Set(Decimal::from(amount)),
        ..Default::default()
      }
      .insert(db)
      .await?;
    }
  }

  // ── Documents: dispatch ───────────────────────────────────────────────────

  let dispatch_purposes = [DispatchPurpose::External, DispatchPurpose::Internal];
  let dispatch_methods = [
    DispatchMethod::Truck,
    DispatchMethod::VesselTerminal,
    DispatchMethod::Bunkering,
  ];
  let mut dispatch_count = 0usize;

  for i in 0..500usize {
    let days_ago = (rng.next_u64() % 730) as i64;
    let doc_date = now - Duration::days(days_ago);
    let contractor_id = contractor_ids[(rng.next_u64() as usize) % contractor_ids.len()];
    let purpose = dispatch_purposes[(rng.next_u64() as usize) % dispatch_purposes.len()];
    let method = dispatch_methods[(rng.next_u64() as usize) % dispatch_methods.len()];

    let doc = dispatch_document::ActiveModel {
      document_number: Set(format!("DIS-{:06}", 100_000 + i)),
      date: Set(doc_date),
      status: Set(DocumentStatus::Posted),
      version: Set(1),
      executed_at: Set(Some(doc_date + Duration::hours(2))),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      dispatch_purpose: Set(purpose),
      dispatch_method: Set(method),
      contractor_id: Set(contractor_id),
      destination_base_id: Set(None),
      receiver_entity: Set(None),
      start_cargo_ops: Set(None),
      end_cargo_ops: Set(None),
      bunker_type: Set(None),
      exporter_id: Set(None),
      port_id: Set(Some(port_ids[(rng.next_u64() as usize) % port_ids.len()])),
      ..Default::default()
    }
    .insert(db)
    .await?;
    dispatch_count += 1;

    let item_count = 2 + (rng.next_u64() % 2) as usize;
    for _ in 0..item_count {
      let product_id = product_ids[(rng.next_u64() as usize) % product_ids.len()];
      let storage_id = storage_ids[(rng.next_u64() as usize) % storage_ids.len()];
      let amount = 100u64 + rng.next_u64() % 9_900;

      dispatch_item::ActiveModel {
        dispatch_doc_id: Set(doc.id),
        product_id: Set(product_id),
        storage_id: Set(storage_id),
        dispatched_amount: Set(Decimal::from(amount)),
        ..Default::default()
      }
      .insert(db)
      .await?;
    }
  }

  // ── Documents: blending ───────────────────────────────────────────────────

  let mut blending_count = 0usize;

  for i in 0..150usize {
    let days_ago = (rng.next_u64() % 730) as i64;
    let doc_date = now - Duration::days(days_ago);
    let contractor_id = contractor_ids[(rng.next_u64() as usize) % contractor_ids.len()];
    let target_product_id = product_ids[(rng.next_u64() as usize) % product_ids.len()];

    let doc = blending_document::ActiveModel {
      document_number: Set(format!("BLD-{:06}", 100_000 + i)),
      date: Set(doc_date),
      status: Set(DocumentStatus::Posted),
      version: Set(1),
      executed_at: Set(Some(doc_date + Duration::hours(3))),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      contractor_id: Set(contractor_id),
      target_product_id: Set(target_product_id),
      ..Default::default()
    }
    .insert(db)
    .await?;
    blending_count += 1;

    // 2 components
    for _ in 0..2usize {
      let source_product_id = product_ids[(rng.next_u64() as usize) % product_ids.len()];
      let storage_id = storage_ids[(rng.next_u64() as usize) % storage_ids.len()];
      let amount = 50u64 + rng.next_u64() % 5_000;

      blending_component::ActiveModel {
        blending_doc_id: Set(doc.id),
        storage_id: Set(storage_id),
        source_product_id: Set(source_product_id),
        amount_used: Set(Decimal::from(amount)),
        ..Default::default()
      }
      .insert(db)
      .await?;
    }

    // 1 result
    let result_storage_id = storage_ids[(rng.next_u64() as usize) % storage_ids.len()];
    let produced = 80u64 + rng.next_u64() % 9_000;
    blending_result::ActiveModel {
      blending_doc_id: Set(doc.id),
      storage_id: Set(result_storage_id),
      produced_amount: Set(Decimal::from(produced)),
      ..Default::default()
    }
    .insert(db)
    .await?;
  }

  // ── Documents: ownership transfer ─────────────────────────────────────────

  let mut ownership_count = 0usize;

  for _i in 0..200usize {
    let days_ago = (rng.next_u64() % 730) as i64;
    let doc_date = now - Duration::days(days_ago);

    let doc = ownership_transfer::ActiveModel {
      date: Set(doc_date),
      status: Set(DocumentStatus::Posted),
      version: Set(1),
      executed_at: Set(Some(doc_date + Duration::hours(1))),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      ..Default::default()
    }
    .insert(db)
    .await?;
    ownership_count += 1;

    let item_count = 2 + (rng.next_u64() % 3) as usize;
    for _ in 0..item_count {
      let from_idx = (rng.next_u64() as usize) % contractor_ids.len();
      let mut to_idx = (rng.next_u64() as usize) % contractor_ids.len();
      if to_idx == from_idx {
        to_idx = (to_idx + 1) % contractor_ids.len();
      }
      let storage_id = storage_ids[(rng.next_u64() as usize) % storage_ids.len()];
      let product_id = product_ids[(rng.next_u64() as usize) % product_ids.len()];
      let amount = 50u64 + rng.next_u64() % 5_000;

      ownership_transfer_item::ActiveModel {
        ownership_transfer_id: Set(doc.id),
        storage_id: Set(storage_id),
        product_id: Set(product_id),
        from_contractor_id: Set(contractor_ids[from_idx]),
        to_contractor_id: Set(contractor_ids[to_idx]),
        amount: Set(Decimal::from(amount)),
        ..Default::default()
      }
      .insert(db)
      .await?;
    }
  }

  // ── Documents: physical storage transfer ─────────────────────────────────

  let mut physical_count = 0usize;

  for i in 0..120usize {
    let days_ago = (rng.next_u64() % 730) as i64;
    let doc_date = now - Duration::days(days_ago);
    let start_ops = doc_date - Duration::hours(2);
    let end_ops = doc_date;

    let doc = physical_storage_transfer::ActiveModel {
      document_number: Set(format!("PST-{:06}", 100_000 + i)),
      date: Set(doc_date),
      status: Set(DocumentStatus::Posted),
      version: Set(1),
      executed_at: Set(Some(doc_date + Duration::hours(1))),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      start_cargo_ops: Set(start_ops),
      end_cargo_ops: Set(end_ops),
      ..Default::default()
    }
    .insert(db)
    .await?;
    physical_count += 1;

    let item_count = 2 + (rng.next_u64() % 2) as usize;
    for _ in 0..item_count {
      let contractor_id = contractor_ids[(rng.next_u64() as usize) % contractor_ids.len()];
      let product_id = product_ids[(rng.next_u64() as usize) % product_ids.len()];
      let from_idx = (rng.next_u64() as usize) % storage_ids.len();
      let mut to_idx = (rng.next_u64() as usize) % storage_ids.len();
      if to_idx == from_idx {
        to_idx = (to_idx + 1) % storage_ids.len();
      }
      let amount = 50u64 + rng.next_u64() % 5_000;

      physical_transfer_item::ActiveModel {
        physical_transfer_id: Set(doc.id),
        contractor_id: Set(contractor_id),
        product_id: Set(product_id),
        from_storage_id: Set(storage_ids[from_idx]),
        to_storage_id: Set(storage_ids[to_idx]),
        amount: Set(Decimal::from(amount)),
        ..Default::default()
      }
      .insert(db)
      .await?;
    }
  }

  // ── Documents: inventory reconciliation ───────────────────────────────────

  let adjustment_types = [AdjustmentType::Surplus, AdjustmentType::Loss];
  let mut recon_count = 0usize;

  for i in 0..80usize {
    let days_ago = (rng.next_u64() % 730) as i64;
    let doc_date = now - Duration::days(days_ago);
    let warehouse_id = warehouse_ids[(rng.next_u64() as usize) % warehouse_ids.len()];

    let doc = inventory_reconciliation::ActiveModel {
      document_number: Set(format!("REC-{:06}", 100_000 + i)),
      date: Set(doc_date),
      status: Set(DocumentStatus::Posted),
      version: Set(1),
      executed_at: Set(Some(doc_date + Duration::hours(4))),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      warehouse_id: Set(warehouse_id),
      ..Default::default()
    }
    .insert(db)
    .await?;
    recon_count += 1;

    // 2–4 adjustments per reconciliation
    let adj_count = 2 + (rng.next_u64() % 3) as usize;
    for _ in 0..adj_count {
      let storage_id = storage_ids[(rng.next_u64() as usize) % storage_ids.len()];
      let product_id = product_ids[(rng.next_u64() as usize) % product_ids.len()];
      let contractor_id = contractor_ids[(rng.next_u64() as usize) % contractor_ids.len()];
      let adj_type = adjustment_types[(rng.next_u64() as usize) % adjustment_types.len()];
      let amount = 1u64 + rng.next_u64() % 500;

      inventory_adjustment::ActiveModel {
        reconciliation_id: Set(doc.id),
        storage_id: Set(storage_id),
        product_id: Set(product_id),
        contractor_id: Set(contractor_id),
        adjustment_type: Set(adj_type),
        amount: Set(Decimal::from(amount)),
        reason: Set(None),
        ..Default::default()
      }
      .insert(db)
      .await?;
    }
  }

  Ok(SeedResult {
    product_types: ptype_ids.len(),
    product_groups: pgroup_ids.len(),
    products: product_ids.len(),
    companies: all_company_ids.len(),
    ports: port_ids.len(),
    bases: base_ids.len(),
    warehouses: warehouse_ids.len(),
    storages: storage_ids.len(),
    users: 8,
    truck_waybills: truck_ids.len(),
    rail_waybills: rail_ids.len(),
    acceptance_docs: acceptance_count,
    dispatch_docs: dispatch_count,
    blending_docs: blending_count,
    ownership_transfers: ownership_count,
    physical_transfers: physical_count,
    reconciliations: recon_count,
  })
}
