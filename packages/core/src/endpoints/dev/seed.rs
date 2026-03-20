use std::{ops::Range, sync::Arc};

use axum::extract::State;
use chrono::{Duration, Utc};
use fake::{
  faker::{
    address::en::{CityName, CountryName},
    company::en::CompanyName,
    internet::en::Username,
    lorem::en::Sentence,
    name::en::Name,
  },
  Fake,
};
use rand::{RngExt, SeedableRng};
use sea_orm::{
  prelude::Decimal,
  ActiveModelTrait,
  ActiveValue::Set,
  DatabaseConnection,
  EntityTrait,
};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
  api::{response::ApiResponse, result::ApiResult, state::ApiState, ApiError},
  context::audit::with_audit_context,
  entities::{
    acceptance_document,
    acceptance_item,
    base,
    blending_component,
    blending_document,
    blending_result,
    company,
    dispatch_document,
    dispatch_item,
    inventory_adjustment,
    inventory_reconciliation,
    local,
    ownership_transfer,
    ownership_transfer_item,
    physical_storage_transfer,
    physical_transfer_item,
    port,
    product,
    product_group,
    product_type,
    rail_waybill,
    storage,
    truck_waybill,
    user,
    warehouse,
  },
  enums::{AdjustmentType, ArrivalType, DispatchMethod, DispatchPurpose, DocumentStatus, RoleType},
  utils::password::hash_password,
};

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

const PRODUCT_FAMILIES: &[&str] = &[
  "Crude Oil",
  "Gasoline",
  "Diesel",
  "Jet Fuel",
  "Lubricants",
  "LPG",
  "Bitumen",
  "Naphtha",
];

const BASE_SUFFIXES: &[&str] = &[
  "Terminal",
  "Depot",
  "Storage Base",
  "Tank Farm",
  "Logistics Hub",
];
const STORAGE_LABELS: &[&str] = &["Tank", "Cell", "Bay", "Reservoir", "Line"];
const COMPANY_ROLE_TAILS: &[&str] = &["Trading", "Logistics", "Terminal", "Energy", "Supply"];
const LEGAL_SUFFIXES: &[&str] = &["LLC", "Ltd.", "Inc.", "GmbH", "Zrt."];

async fn do_seed(db: &DatabaseConnection) -> Result<SeedResult, ApiError> {
  let mut rng = {
    let mut os_rng = rand::rng();
    rand::rngs::StdRng::from_rng(&mut os_rng)
  };
  let now = Utc::now();

  let dev_password_hash = hash_password("password123")
    .await
    .map_err(ApiError::Internal)?;

  let mut ptype_ids: Vec<Uuid> = Vec::new();
  let mut pgroup_count = 0usize;
  let mut product_count = 0usize;
  let mut component_product_ids: Vec<Uuid> = Vec::new();
  let mut target_product_ids: Vec<Uuid> = Vec::new();

  for family in PRODUCT_FAMILIES {
    let type_name = format!("{family} {}", title_fragment(fake_fragment(1..3)));
    let ptype = product_type::ActiveModel {
      common_name: Set(unique_name(type_name, &mut rng)),
      long_name: Set(maybe(&mut rng, 0.35, |_rng| {
        format!("{family} {}", title_fragment(fake_fragment(4..7)))
      })),
      ..Default::default()
    }
    .insert(db)
    .await?;
    ptype_ids.push(ptype.id);

    let group_count = rng.random_range(2..=4);
    for group_idx in 0..group_count {
      let group_name = unique_name(
        format!("{} {}", family, title_fragment(fake_fragment(1..3))),
        &mut rng,
      );
      let pgroup = product_group::ActiveModel {
        product_type_id: Set(ptype.id),
        common_name: Set(group_name),
        long_name: Set(maybe(&mut rng, 0.3, |_rng| {
          format!(
            "{family} {} blend line {}",
            title_fragment(fake_fragment(2..5)),
            group_idx + 1
          )
        })),
        ..Default::default()
      }
      .insert(db)
      .await?;
      pgroup_count += 1;

      let products_in_group = rng.random_range(2..=5);
      for product_idx in 0..products_in_group {
        let is_component = product_idx < 2 || rng.random_bool(0.55);
        let product_model = product::ActiveModel {
          product_group_id: Set(pgroup.id),
          manufacturer_id: Set(None),
          common_name: Set(format!(
            "{} {}",
            title_fragment(fake_fragment(1..3)),
            unique_suffix(&mut rng)
          )),
          long_name: Set(maybe(&mut rng, 0.25, |rng| {
            format!(
              "{family} {} batch {}",
              title_fragment(fake_fragment(2..5)),
              unique_suffix(rng)
            )
          })),
          add_identification: Set(maybe(&mut rng, 0.4, |rng| {
            format!("LOT-{}-{}", group_idx + 1, unique_suffix(rng))
          })),
          is_component: Set(is_component),
          ..Default::default()
        }
        .insert(db)
        .await?;
        product_count += 1;

        if is_component {
          component_product_ids.push(product_model.id);
        } else {
          target_product_ids.push(product_model.id);
        }
      }
    }
  }

  if target_product_ids.is_empty() {
    target_product_ids.extend(component_product_ids.iter().copied());
  }

  let mut contractor_ids: Vec<Uuid> = Vec::new();
  let mut sender_ids: Vec<Uuid> = Vec::new();
  let mut exporter_ids: Vec<Uuid> = Vec::new();
  let mut all_company_ids: Vec<Uuid> = Vec::new();

  let company_count = rng.random_range(24..=42);
  for _ in 0..company_count {
    let company_name = unique_name(
      format!(
        "{} {}",
        CompanyName().fake::<String>(),
        pick(&mut rng, COMPANY_ROLE_TAILS)
      ),
      &mut rng,
    );

    let is_contractor = rng.random_bool(0.7);
    let is_exporter = rng.random_bool(0.35);
    let is_manufacturer = rng.random_bool(0.25);
    let is_sender = rng.random_bool(0.45);

    let model = company::ActiveModel {
      common_name: Set(company_name.clone()),
      legal_name: Set(maybe(&mut rng, 0.55, |rng| {
        format!(
          "{} {}",
          CompanyName().fake::<String>(),
          pick(rng, LEGAL_SUFFIXES)
        )
      })),
      is_contractor: Set(is_contractor),
      is_exporter: Set(is_exporter),
      is_manufacturer: Set(is_manufacturer),
      is_sender: Set(is_sender),
      ..Default::default()
    }
    .insert(db)
    .await?;

    all_company_ids.push(model.id);
    if is_contractor {
      contractor_ids.push(model.id);
    }
    if is_sender {
      sender_ids.push(model.id);
    }
    if is_exporter {
      exporter_ids.push(model.id);
    }
  }

  if contractor_ids.is_empty() {
    contractor_ids.extend(all_company_ids.iter().copied());
  }
  if sender_ids.is_empty() {
    sender_ids.extend(all_company_ids.iter().copied());
  }
  if exporter_ids.is_empty() {
    exporter_ids.extend(all_company_ids.iter().copied());
  }

  let mut port_ids: Vec<Uuid> = Vec::new();
  let port_count = rng.random_range(12..=22);
  for _ in 0..port_count {
    let model = port::ActiveModel {
      common_name: Set(unique_name(
        format!("{} Harbor", CityName().fake::<String>()),
        &mut rng,
      )),
      country: Set(maybe(&mut rng, 0.8, |_rng| CountryName().fake::<String>())),
      ..Default::default()
    }
    .insert(db)
    .await?;
    port_ids.push(model.id);
  }

  let mut base_ids: Vec<Uuid> = Vec::new();
  let mut warehouse_ids: Vec<Uuid> = Vec::new();
  let mut storage_ids: Vec<Uuid> = Vec::new();

  let base_count = rng.random_range(5..=9);
  for _ in 0..base_count {
    let base_name = unique_name(
      format!(
        "{} {}",
        CityName().fake::<String>(),
        pick(&mut rng, BASE_SUFFIXES)
      ),
      &mut rng,
    );

    let base_model = base::ActiveModel {
      common_name: Set(base_name.clone()),
      long_name: Set(maybe(&mut rng, 0.25, |_rng| {
        format!("{} {}", base_name, title_fragment(fake_fragment(3..6)))
      })),
      ..Default::default()
    }
    .insert(db)
    .await?;
    base_ids.push(base_model.id);

    let warehouse_count = rng.random_range(2..=4);
    for warehouse_idx in 0..warehouse_count {
      let warehouse_name = format!(
        "{} {} {}",
        base_name,
        title_fragment(fake_fragment(1..3)),
        warehouse_idx + 1
      );
      let warehouse_model = warehouse::ActiveModel {
        base_id: Set(base_model.id),
        common_name: Set(warehouse_name),
        long_name: Set(maybe(&mut rng, 0.2, |_rng| {
          title_fragment(fake_fragment(4..7))
        })),
        ..Default::default()
      }
      .insert(db)
      .await?;
      warehouse_ids.push(warehouse_model.id);

      let storage_count = rng.random_range(3..=7);
      for _ in 0..storage_count {
        let is_type_specific = rng.random_bool(0.35);
        let capacity_val = rng.random_range(2_500u64..=85_000u64);
        let storage_model = storage::ActiveModel {
          warehouse_id: Set(warehouse_model.id),
          common_name: Set(format!(
            "{} {}",
            pick(&mut rng, STORAGE_LABELS),
            rng.random_range(1..=96)
          )),
          long_name: Set(maybe(&mut rng, 0.2, |_rng| {
            title_fragment(fake_fragment(2..5))
          })),
          capacity: Set(Some(Decimal::from(capacity_val))),
          is_type_specific: Set(is_type_specific),
          product_type_id: Set(if is_type_specific {
            Some(*pick(&mut rng, &ptype_ids))
          } else {
            None
          }),
          ..Default::default()
        }
        .insert(db)
        .await?;
        storage_ids.push(storage_model.id);
      }
    }
  }

  let extra_roles = [
    RoleType::SeniorSupervisor,
    RoleType::Supervisor,
    RoleType::Operator,
  ];
  let user_count = rng.random_range(6..=14);
  for user_idx in 0..user_count {
    let fullname = Name().fake::<String>();
    user::ActiveModel {
      username: Set(fake_username(&mut rng, user_idx)),
      fullname: Set(Some(fullname)),
      password_hash: Set(dev_password_hash.clone()),
      role_id: Set(extra_roles[user_idx % extra_roles.len()].uuid()),
      home_base_id: Set(Some(*pick(&mut rng, &base_ids))),
      ..Default::default()
    }
    .insert(db)
    .await?;
  }

  let mut truck_ids: Vec<Uuid> = Vec::new();
  let truck_count = rng.random_range(180..=360);
  for idx in 0..truck_count {
    let days_ago = rng.random_range(0..=730) as i64;
    let date = (now - Duration::days(days_ago)).date_naive();
    let sender_id = *pick(&mut rng, &sender_ids);
    let model = truck_waybill::ActiveModel {
      document_number: Set(fake_document_number("TW", idx, &mut rng, now)),
      date: Set(date),
      sender_id: Set(sender_id),
      ..Default::default()
    }
    .insert(db)
    .await?;
    truck_ids.push(model.id);
  }

  let mut rail_ids: Vec<Uuid> = Vec::new();
  let rail_count = rng.random_range(120..=260);
  for idx in 0..rail_count {
    let days_ago = rng.random_range(0..=730) as i64;
    let date = (now - Duration::days(days_ago)).date_naive();
    let sender_id = *pick(&mut rng, &sender_ids);
    let model = rail_waybill::ActiveModel {
      document_number: Set(fake_document_number("RW", idx, &mut rng, now)),
      date: Set(date),
      sender_id: Set(sender_id),
      ..Default::default()
    }
    .insert(db)
    .await?;
    rail_ids.push(model.id);
  }

  let dispatch_purposes = [DispatchPurpose::External, DispatchPurpose::Internal];
  let dispatch_methods = [
    DispatchMethod::Truck,
    DispatchMethod::VesselTerminal,
    DispatchMethod::Bunkering,
  ];
  let mut dispatch_ids: Vec<Uuid> = Vec::new();
  let mut dispatch_count = 0usize;

  let dispatch_doc_count = rng.random_range(280..=560);
  for idx in 0..dispatch_doc_count {
    let days_ago = rng.random_range(0..=730) as i64;
    let doc_date = now - Duration::days(days_ago);
    let contractor_id = *pick(&mut rng, &contractor_ids);
    let purpose = *pick(&mut rng, &dispatch_purposes);
    let method = *pick(&mut rng, &dispatch_methods);
    let start_ops = doc_date - Duration::hours(rng.random_range(1..=8));
    let end_ops = start_ops + Duration::hours(rng.random_range(1..=6));

    let destination_base_id = if matches!(purpose, DispatchPurpose::Internal) {
      Some(*pick(&mut rng, &base_ids))
    } else {
      None
    };
    let receiver_entity = if matches!(purpose, DispatchPurpose::External) || rng.random_bool(0.25) {
      Some(CompanyName().fake::<String>())
    } else {
      None
    };
    let exporter_id = if matches!(purpose, DispatchPurpose::External) && rng.random_bool(0.6) {
      Some(*pick(&mut rng, &exporter_ids))
    } else {
      None
    };
    let port_id = if matches!(purpose, DispatchPurpose::External)
      || matches!(method, DispatchMethod::VesselTerminal)
    {
      Some(*pick(&mut rng, &port_ids))
    } else {
      None
    };

    let doc = dispatch_document::ActiveModel {
      document_number: Set(fake_document_number("DIS", idx, &mut rng, now)),
      date: Set(doc_date),
      status: Set(DocumentStatus::Posted),
      version: Set(1),
      executed_at: Set(Some(
        end_ops + Duration::minutes(rng.random_range(15..=180)),
      )),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      dispatch_purpose: Set(purpose),
      dispatch_method: Set(method),
      contractor_id: Set(contractor_id),
      destination_base_id: Set(destination_base_id),
      receiver_entity: Set(receiver_entity),
      start_cargo_ops: Set(Some(start_ops)),
      end_cargo_ops: Set(Some(end_ops)),
      bunker_type: Set(None),
      exporter_id: Set(exporter_id),
      port_id: Set(port_id),
      ..Default::default()
    }
    .insert(db)
    .await?;
    dispatch_ids.push(doc.id);
    dispatch_count += 1;

    let item_count = rng.random_range(1..=4);
    for _ in 0..item_count {
      let product_id = *pick(&mut rng, &target_product_ids);
      let storage_id = *pick(&mut rng, &storage_ids);
      let amount = rng.random_range(80u64..=12_500u64);

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

  let arrival_types = [ArrivalType::Truck, ArrivalType::Rail, ArrivalType::External];
  let mut acceptance_count = 0usize;
  let acceptance_doc_count = rng.random_range(320..=620);
  for idx in 0..acceptance_doc_count {
    let days_ago = rng.random_range(0..=730) as i64;
    let doc_date = now - Duration::days(days_ago);
    let arrival_type = *pick(&mut rng, &arrival_types);

    let truck_waybill_id = if matches!(arrival_type, ArrivalType::Truck) {
      Some(*pick(&mut rng, &truck_ids))
    } else {
      None
    };
    let rail_waybill_id = if matches!(arrival_type, ArrivalType::Rail) {
      Some(*pick(&mut rng, &rail_ids))
    } else {
      None
    };
    let transit_dispatch_id = if matches!(arrival_type, ArrivalType::External)
      && rng.random_bool(0.2)
      && !dispatch_ids.is_empty()
    {
      Some(*pick(&mut rng, &dispatch_ids))
    } else {
      None
    };
    let source_entity = if matches!(arrival_type, ArrivalType::External) || rng.random_bool(0.15) {
      Some(CompanyName().fake::<String>())
    } else {
      None
    };

    let doc = acceptance_document::ActiveModel {
      document_number: Set(fake_document_number("ACC", idx, &mut rng, now)),
      date_accepted: Set(doc_date),
      status: Set(DocumentStatus::Posted),
      version: Set(1),
      executed_at: Set(Some(
        doc_date + Duration::minutes(rng.random_range(20..=240)),
      )),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      arrival_type: Set(arrival_type),
      source_entity: Set(source_entity),
      truck_waybill_id: Set(truck_waybill_id),
      rail_waybill_id: Set(rail_waybill_id),
      transit_dispatch_id: Set(transit_dispatch_id),
      ..Default::default()
    }
    .insert(db)
    .await?;
    acceptance_count += 1;

    let item_count = rng.random_range(1..=4);
    for _ in 0..item_count {
      let amount = rng.random_range(100u64..=11_500u64);
      acceptance_item::ActiveModel {
        acceptance_doc_id: Set(doc.id),
        product_id: Set(*pick(&mut rng, &target_product_ids)),
        contractor_id: Set(*pick(&mut rng, &contractor_ids)),
        storage_id: Set(*pick(&mut rng, &storage_ids)),
        accepted_amount: Set(Decimal::from(amount)),
        ..Default::default()
      }
      .insert(db)
      .await?;
    }
  }

  let mut blending_count = 0usize;
  let blending_doc_count = rng.random_range(100..=220);
  for idx in 0..blending_doc_count {
    let days_ago = rng.random_range(0..=730) as i64;
    let doc_date = now - Duration::days(days_ago);
    let doc = blending_document::ActiveModel {
      document_number: Set(fake_document_number("BLD", idx, &mut rng, now)),
      date: Set(doc_date),
      status: Set(DocumentStatus::Posted),
      version: Set(1),
      executed_at: Set(Some(
        doc_date + Duration::minutes(rng.random_range(30..=180)),
      )),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      contractor_id: Set(*pick(&mut rng, &contractor_ids)),
      target_product_id: Set(*pick(&mut rng, &target_product_ids)),
      ..Default::default()
    }
    .insert(db)
    .await?;
    blending_count += 1;

    let component_count = rng.random_range(2..=4);
    for _ in 0..component_count {
      let amount = rng.random_range(50u64..=6_000u64);
      blending_component::ActiveModel {
        blending_doc_id: Set(doc.id),
        storage_id: Set(*pick(&mut rng, &storage_ids)),
        source_product_id: Set(*pick(&mut rng, &component_product_ids)),
        amount_used: Set(Decimal::from(amount)),
        ..Default::default()
      }
      .insert(db)
      .await?;
    }

    let result_count = rng.random_range(1..=2);
    for _ in 0..result_count {
      let produced = rng.random_range(80u64..=9_500u64);
      blending_result::ActiveModel {
        blending_doc_id: Set(doc.id),
        storage_id: Set(*pick(&mut rng, &storage_ids)),
        produced_amount: Set(Decimal::from(produced)),
        ..Default::default()
      }
      .insert(db)
      .await?;
    }
  }

  let mut ownership_count = 0usize;
  let ownership_doc_count = rng.random_range(140..=260);
  for _ in 0..ownership_doc_count {
    let days_ago = rng.random_range(0..=730) as i64;
    let doc_date = now - Duration::days(days_ago);
    let doc = ownership_transfer::ActiveModel {
      date: Set(doc_date),
      status: Set(DocumentStatus::Posted),
      version: Set(1),
      executed_at: Set(Some(
        doc_date + Duration::minutes(rng.random_range(15..=120)),
      )),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      ..Default::default()
    }
    .insert(db)
    .await?;
    ownership_count += 1;

    let item_count = rng.random_range(1..=4);
    for _ in 0..item_count {
      let from_id = *pick(&mut rng, &contractor_ids);
      let mut to_id = *pick(&mut rng, &contractor_ids);
      while to_id == from_id {
        to_id = *pick(&mut rng, &contractor_ids);
      }

      ownership_transfer_item::ActiveModel {
        ownership_transfer_id: Set(doc.id),
        storage_id: Set(*pick(&mut rng, &storage_ids)),
        product_id: Set(*pick(&mut rng, &target_product_ids)),
        from_contractor_id: Set(from_id),
        to_contractor_id: Set(to_id),
        amount: Set(Decimal::from(rng.random_range(40u64..=5_500u64))),
        ..Default::default()
      }
      .insert(db)
      .await?;
    }
  }

  let mut physical_count = 0usize;
  let physical_doc_count = rng.random_range(90..=180);
  for idx in 0..physical_doc_count {
    let days_ago = rng.random_range(0..=730) as i64;
    let doc_date = now - Duration::days(days_ago);
    let start_ops = doc_date - Duration::hours(rng.random_range(1..=6));
    let end_ops = start_ops + Duration::hours(rng.random_range(1..=6));
    let doc = physical_storage_transfer::ActiveModel {
      document_number: Set(fake_document_number("PST", idx, &mut rng, now)),
      date: Set(doc_date),
      status: Set(DocumentStatus::Posted),
      version: Set(1),
      executed_at: Set(Some(
        end_ops + Duration::minutes(rng.random_range(10..=120)),
      )),
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

    let item_count = rng.random_range(1..=4);
    for _ in 0..item_count {
      let from_storage_id = *pick(&mut rng, &storage_ids);
      let mut to_storage_id = *pick(&mut rng, &storage_ids);
      while to_storage_id == from_storage_id {
        to_storage_id = *pick(&mut rng, &storage_ids);
      }

      physical_transfer_item::ActiveModel {
        physical_transfer_id: Set(doc.id),
        contractor_id: Set(*pick(&mut rng, &contractor_ids)),
        product_id: Set(*pick(&mut rng, &target_product_ids)),
        from_storage_id: Set(from_storage_id),
        to_storage_id: Set(to_storage_id),
        amount: Set(Decimal::from(rng.random_range(35u64..=6_000u64))),
        ..Default::default()
      }
      .insert(db)
      .await?;
    }
  }

  let adjustment_types = [AdjustmentType::Surplus, AdjustmentType::Loss];
  let mut recon_count = 0usize;
  let reconciliation_doc_count = rng.random_range(60..=130);
  for idx in 0..reconciliation_doc_count {
    let days_ago = rng.random_range(0..=730) as i64;
    let doc_date = now - Duration::days(days_ago);
    let doc = inventory_reconciliation::ActiveModel {
      document_number: Set(fake_document_number("REC", idx, &mut rng, now)),
      date: Set(doc_date),
      status: Set(DocumentStatus::Posted),
      version: Set(1),
      executed_at: Set(Some(
        doc_date + Duration::minutes(rng.random_range(30..=240)),
      )),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      warehouse_id: Set(*pick(&mut rng, &warehouse_ids)),
      ..Default::default()
    }
    .insert(db)
    .await?;
    recon_count += 1;

    let adjustment_count = rng.random_range(1..=5);
    for _ in 0..adjustment_count {
      inventory_adjustment::ActiveModel {
        reconciliation_id: Set(doc.id),
        storage_id: Set(*pick(&mut rng, &storage_ids)),
        product_id: Set(*pick(&mut rng, &target_product_ids)),
        contractor_id: Set(*pick(&mut rng, &contractor_ids)),
        adjustment_type: Set(*pick(&mut rng, &adjustment_types)),
        amount: Set(Decimal::from(rng.random_range(1u64..=900u64))),
        reason: Set(maybe(&mut rng, 0.45, |_rng| fake_fragment(4..8))),
        ..Default::default()
      }
      .insert(db)
      .await?;
    }
  }

  Ok(SeedResult {
    product_types: ptype_ids.len(),
    product_groups: pgroup_count,
    products: product_count,
    companies: all_company_ids.len(),
    ports: port_ids.len(),
    bases: base_ids.len(),
    warehouses: warehouse_ids.len(),
    storages: storage_ids.len(),
    users: user_count,
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

fn pick<'a, T>(rng: &mut rand::rngs::StdRng, items: &'a [T]) -> &'a T {
  &items[rng.random_range(0..items.len())]
}

fn maybe<T>(
  rng: &mut rand::rngs::StdRng,
  chance: f64,
  value: impl FnOnce(&mut rand::rngs::StdRng) -> T,
) -> Option<T> {
  if rng.random_bool(chance) {
    Some(value(rng))
  } else {
    None
  }
}

fn fake_fragment(words: Range<usize>) -> String {
  Sentence(words)
    .fake::<String>()
    .trim_end_matches('.')
    .to_string()
}

fn title_fragment(input: String) -> String {
  input
    .split_whitespace()
    .map(|word| {
      let mut chars = word.chars();
      match chars.next() {
        Some(first) => {
          let mut out = first.to_uppercase().collect::<String>();
          out.push_str(chars.as_str());
          out
        }
        None => String::new(),
      }
    })
    .collect::<Vec<_>>()
    .join(" ")
}

fn unique_name(base: String, rng: &mut rand::rngs::StdRng) -> String {
  format!("{base} {}", unique_suffix(rng))
}

fn unique_suffix(rng: &mut rand::rngs::StdRng) -> u16 {
  rng.random_range(100..=999)
}

fn fake_document_number(
  prefix: &str,
  serial: usize,
  rng: &mut rand::rngs::StdRng,
  now: chrono::DateTime<Utc>,
) -> String {
  format!(
    "{prefix}-{}-{:05}-{:03}",
    now.format("%y%m%d"),
    serial + 1,
    unique_suffix(rng)
  )
}

fn fake_username(rng: &mut rand::rngs::StdRng, index: usize) -> String {
  let raw = Username().fake::<String>();
  let cleaned: String = raw
    .chars()
    .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '.')
    .collect();
  let base = if cleaned.is_empty() {
    "devuser".to_string()
  } else {
    cleaned.to_lowercase()
  };
  format!("{base}{}{}", index + 1, unique_suffix(rng))
}
