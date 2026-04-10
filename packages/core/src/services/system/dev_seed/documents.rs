use chrono::Duration;
use fake::{faker::company::en::CompanyName, Fake};
use rand::RngExt;
use sea_orm::{prelude::Decimal, ActiveValue::Set};
use uuid::Uuid;

use super::{
  helpers::{maybe, pick, random_date, random_datetime, random_document_number, saved_uuid},
  DocumentContext,
  DocumentCounts,
  LocationSeed,
  ReferencePool,
  TransportPool,
  WaybillSeed,
};
use crate::{
  api::ApiError,
  entities::{
    acceptance_document,
    acceptance_item,
    blending_component,
    blending_document,
    blending_result,
    dispatch_document,
    dispatch_item,
    inventory_adjustment,
    inventory_reconciliation,
    ownership_transfer,
    ownership_transfer_item,
    physical_storage_transfer,
    physical_transfer_item,
    rail_wagon_manifest,
    rail_waybill,
    truck_waybill,
    truck_waybill_item,
  },
  enums::{AdjustmentType, ArrivalType, DispatchMethod, DispatchPurpose, DocumentStatus},
};

pub async fn seed_transport_documents(
  ctx: &DocumentContext<'_>,
  rng: &mut rand::rngs::StdRng,
  refs: &ReferencePool,
  locations: &[LocationSeed],
) -> Result<TransportPool, ApiError> {
  let mut pool = TransportPool::default();
  let mut truck_serial = 0usize;
  let mut rail_serial = 0usize;

  for location in locations {
    pool
      .truck
      .push(create_truck_waybill(ctx, rng, refs, location, truck_serial).await?);
    truck_serial += 1;
    pool
      .rail
      .push(create_rail_waybill(ctx, rng, refs, location, rail_serial).await?);
    rail_serial += 1;
  }

  for _ in 0..rng.random_range(160..=320) {
    let location = pick(rng, locations);
    pool
      .truck
      .push(create_truck_waybill(ctx, rng, refs, location, truck_serial).await?);
    truck_serial += 1;
  }

  for _ in 0..rng.random_range(100..=220) {
    let location = pick(rng, locations);
    pool
      .rail
      .push(create_rail_waybill(ctx, rng, refs, location, rail_serial).await?);
    rail_serial += 1;
  }

  Ok(pool)
}

pub async fn seed_inventory_documents(
  ctx: &DocumentContext<'_>,
  rng: &mut rand::rngs::StdRng,
  refs: &ReferencePool,
  locations: &[LocationSeed],
  transport: &TransportPool,
) -> Result<DocumentCounts, ApiError> {
  let mut counts = DocumentCounts::default();

  seed_dispatches(ctx, rng, refs, locations, &mut counts).await?;
  seed_acceptances(ctx, rng, refs, locations, transport, &mut counts).await?;
  seed_blends(ctx, rng, refs, locations, &mut counts).await?;
  seed_ownership_transfers(ctx, rng, refs, locations, &mut counts).await?;
  seed_physical_transfers(ctx, rng, refs, locations, &mut counts).await?;
  seed_reconciliations(ctx, rng, refs, locations, &mut counts).await?;

  Ok(counts)
}

async fn create_truck_waybill(
  ctx: &DocumentContext<'_>,
  rng: &mut rand::rngs::StdRng,
  refs: &ReferencePool,
  location: &LocationSeed,
  serial: usize,
) -> Result<WaybillSeed, ApiError> {
  let model = truck_waybill::ActiveModelEx {
    document_number: Set(random_document_number(&ctx.tag, "TW", serial)),
    date: Set(random_date(ctx.now, rng)),
    sender_id: Set(*pick(rng, &refs.companies.senders)),
    base_id: Set(location.base_id),
    items: (0..rng.random_range(1..=2))
      .map(|_| truck_waybill_item::ActiveModelEx {
        product_id: Set(*pick(rng, &refs.target_product_ids)),
        declared_amount: Set(Decimal::new(rng.random_range(5_000..50_000) as i64, 0)),
        ..Default::default()
      })
      .collect::<Vec<_>>()
      .into(),
    ..Default::default()
  }
  .save(ctx.conn)
  .await?;

  Ok(WaybillSeed {
    id: saved_uuid(model.id, "truck waybill")?,
    base_id: location.base_id,
  })
}

async fn create_rail_waybill(
  ctx: &DocumentContext<'_>,
  rng: &mut rand::rngs::StdRng,
  refs: &ReferencePool,
  location: &LocationSeed,
  serial: usize,
) -> Result<WaybillSeed, ApiError> {
  let model = rail_waybill::ActiveModelEx {
    document_number: Set(random_document_number(&ctx.tag, "RW", serial)),
    date: Set(random_date(ctx.now, rng)),
    sender_id: Set(*pick(rng, &refs.companies.senders)),
    base_id: Set(location.base_id),
    wagon_manifests: (0..rng.random_range(1..=3))
      .map(|_| {
        let volume = Decimal::new(rng.random_range(20_000..80_000) as i64, 0);
        let density = Decimal::new(rng.random_range(700..950) as i64, 1);
        rail_wagon_manifest::ActiveModelEx {
          wagon_number: Set(format!(
            "{:08}",
            rng.random_range(10_000_000u32..99_999_999u32)
          )),
          product_id: Set(*pick(rng, &refs.target_product_ids)),
          declared_volume: Set(volume),
          declared_density: Set(density),
          declared_mass: Set(volume * density / Decimal::new(10, 0)),
          ..Default::default()
        }
      })
      .collect::<Vec<_>>()
      .into(),
    ..Default::default()
  }
  .save(ctx.conn)
  .await?;

  Ok(WaybillSeed {
    id: saved_uuid(model.id, "rail waybill")?,
    base_id: location.base_id,
  })
}

async fn seed_dispatches(
  ctx: &DocumentContext<'_>,
  rng: &mut rand::rngs::StdRng,
  refs: &ReferencePool,
  locations: &[LocationSeed],
  counts: &mut DocumentCounts,
) -> Result<(), ApiError> {
  let purposes = [DispatchPurpose::External, DispatchPurpose::Internal];
  let methods = [
    DispatchMethod::Truck,
    DispatchMethod::VesselTerminal,
    DispatchMethod::Bunkering,
  ];

  for serial in 0..rng.random_range(260..=520) {
    let location = pick(rng, locations);
    let dispatch_date = random_datetime(ctx.now, rng);
    let purpose = *pick(rng, &purposes);
    let method = *pick(rng, &methods);
    let start_ops = dispatch_date - Duration::hours(rng.random_range(1..=8));
    let end_ops = start_ops + Duration::hours(rng.random_range(1..=6));
    let contractor_id = *pick(rng, &refs.companies.contractors);

    let saved = dispatch_document::ActiveModelEx {
      document_number: Set(random_document_number(&ctx.tag, "DIS", serial)),
      date: Set(dispatch_date),
      status: Set(DocumentStatus::Executed),
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
      destination_base_id: Set(if matches!(purpose, DispatchPurpose::Internal) {
        Some(pick(rng, locations).base_id)
      } else {
        None
      }),
      receiver_entity: Set(
        if matches!(purpose, DispatchPurpose::External) || rng.random_bool(0.25) {
          Some(CompanyName().fake::<String>())
        } else {
          None
        },
      ),
      start_cargo_ops: Set(Some(start_ops)),
      end_cargo_ops: Set(Some(end_ops)),
      bunker_type: Set(None),
      exporter_id: Set(
        if matches!(purpose, DispatchPurpose::External) && rng.random_bool(0.6) {
          Some(*pick(rng, &refs.companies.exporters))
        } else {
          None
        },
      ),
      port_id: Set(
        if matches!(purpose, DispatchPurpose::External)
          || matches!(method, DispatchMethod::VesselTerminal)
        {
          Some(*pick(rng, &refs.port_ids))
        } else {
          None
        },
      ),
      items: (0..rng.random_range(1..=4))
        .map(|_| dispatch_item::ActiveModelEx {
          product_id: Set(*pick(rng, &refs.target_product_ids)),
          storage_id: Set(location.pick_storage(rng)),
          dispatched_amount: Set(Decimal::from(rng.random_range(80u64..=12_500u64))),
          ..Default::default()
        })
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
    .save(ctx.conn)
    .await?;

    let dispatch_id = saved_uuid(saved.id, "dispatch")?;
    counts.dispatch_docs += 1;
    ctx
      .audit
      .backfill_document_routing::<dispatch_document::Entity>(ctx.conn, dispatch_id)
      .await?;
  }

  Ok(())
}

async fn seed_acceptances(
  ctx: &DocumentContext<'_>,
  rng: &mut rand::rngs::StdRng,
  refs: &ReferencePool,
  locations: &[LocationSeed],
  transport: &TransportPool,
  counts: &mut DocumentCounts,
) -> Result<(), ApiError> {
  let arrival_types = [ArrivalType::Truck, ArrivalType::Rail, ArrivalType::External];

  for serial in 0..rng.random_range(300..=580) {
    let location = pick(rng, locations);
    let date_accepted = random_datetime(ctx.now, rng);
    let arrival_type = *pick(rng, &arrival_types);
    let truck_waybill_id = match arrival_type {
      ArrivalType::Truck => pick_waybill_for_base(rng, &transport.truck, location.base_id),
      _ => None,
    };
    let rail_waybill_id = match arrival_type {
      ArrivalType::Rail => pick_waybill_for_base(rng, &transport.rail, location.base_id),
      _ => None,
    };

    let saved = acceptance_document::ActiveModelEx {
      document_number: Set(random_document_number(&ctx.tag, "ACC", serial)),
      date_accepted: Set(date_accepted),
      status: Set(DocumentStatus::Executed),
      version: Set(1),
      executed_at: Set(Some(
        date_accepted + Duration::minutes(rng.random_range(20..=240)),
      )),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      arrival_type: Set(arrival_type),
      source_entity: Set(
        if matches!(arrival_type, ArrivalType::External) || rng.random_bool(0.15) {
          Some(CompanyName().fake::<String>())
        } else {
          None
        },
      ),
      contractor_id: Set(*pick(rng, &refs.companies.contractors)),
      truck_waybill_id: Set(truck_waybill_id),
      rail_waybill_id: Set(rail_waybill_id),
      transit_dispatch_id: Set(None),
      items: (0..rng.random_range(1..=4))
        .map(|_| acceptance_item::ActiveModelEx {
          product_id: Set(*pick(rng, &refs.target_product_ids)),
          storage_id: Set(location.pick_storage(rng)),
          accepted_amount: Set(Decimal::from(rng.random_range(100u64..=11_500u64))),
          ..Default::default()
        })
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
    .save(ctx.conn)
    .await?;

    let acceptance_id = saved_uuid(saved.id, "acceptance")?;
    counts.acceptance_docs += 1;
    ctx
      .audit
      .backfill_document_routing::<acceptance_document::Entity>(ctx.conn, acceptance_id)
      .await?;
  }

  Ok(())
}

async fn seed_blends(
  ctx: &DocumentContext<'_>,
  rng: &mut rand::rngs::StdRng,
  refs: &ReferencePool,
  locations: &[LocationSeed],
  counts: &mut DocumentCounts,
) -> Result<(), ApiError> {
  for serial in 0..rng.random_range(90..=200) {
    let location = pick(rng, locations);
    let date = random_datetime(ctx.now, rng);
    let saved = blending_document::ActiveModelEx {
      document_number: Set(random_document_number(&ctx.tag, "BLD", serial)),
      date: Set(date),
      status: Set(DocumentStatus::Executed),
      version: Set(1),
      executed_at: Set(Some(date + Duration::minutes(rng.random_range(30..=180)))),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      contractor_id: Set(*pick(rng, &refs.companies.contractors)),
      target_product_id: Set(*pick(rng, &refs.target_product_ids)),
      components: (0..rng.random_range(2..=4))
        .map(|_| blending_component::ActiveModelEx {
          storage_id: Set(location.pick_storage(rng)),
          source_product_id: Set(*pick(rng, &refs.component_product_ids)),
          amount_used: Set(Decimal::from(rng.random_range(50u64..=6_000u64))),
          ..Default::default()
        })
        .collect::<Vec<_>>()
        .into(),
      results: (0..rng.random_range(1..=2))
        .map(|_| blending_result::ActiveModelEx {
          storage_id: Set(location.pick_storage(rng)),
          produced_amount: Set(Decimal::from(rng.random_range(80u64..=9_500u64))),
          ..Default::default()
        })
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
    .save(ctx.conn)
    .await?;

    let blending_id = saved_uuid(saved.id, "blending")?;
    counts.blending_docs += 1;
    ctx
      .audit
      .backfill_document_routing::<blending_document::Entity>(ctx.conn, blending_id)
      .await?;
  }

  Ok(())
}

async fn seed_ownership_transfers(
  ctx: &DocumentContext<'_>,
  rng: &mut rand::rngs::StdRng,
  refs: &ReferencePool,
  locations: &[LocationSeed],
  counts: &mut DocumentCounts,
) -> Result<(), ApiError> {
  for _ in 0..rng.random_range(120..=240) {
    let location = pick(rng, locations);
    let date = random_datetime(ctx.now, rng);
    let saved = ownership_transfer::ActiveModelEx {
      date: Set(date),
      status: Set(DocumentStatus::Executed),
      version: Set(1),
      executed_at: Set(Some(date + Duration::minutes(rng.random_range(15..=120)))),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      items: (0..rng.random_range(1..=4))
        .map(|_| {
          let from_contractor_id = *pick(rng, &refs.companies.contractors);
          let mut to_contractor_id = *pick(rng, &refs.companies.contractors);
          while to_contractor_id == from_contractor_id {
            to_contractor_id = *pick(rng, &refs.companies.contractors);
          }

          ownership_transfer_item::ActiveModelEx {
            storage_id: Set(location.pick_storage(rng)),
            product_id: Set(*pick(rng, &refs.target_product_ids)),
            from_contractor_id: Set(from_contractor_id),
            to_contractor_id: Set(to_contractor_id),
            amount: Set(Decimal::from(rng.random_range(40u64..=5_500u64))),
            ..Default::default()
          }
        })
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
    .save(ctx.conn)
    .await?;

    let ownership_id = saved_uuid(saved.id, "ownership transfer")?;
    counts.ownership_transfers += 1;
    ctx
      .audit
      .backfill_document_routing::<ownership_transfer::Entity>(ctx.conn, ownership_id)
      .await?;
  }

  Ok(())
}

async fn seed_physical_transfers(
  ctx: &DocumentContext<'_>,
  rng: &mut rand::rngs::StdRng,
  refs: &ReferencePool,
  locations: &[LocationSeed],
  counts: &mut DocumentCounts,
) -> Result<(), ApiError> {
  for serial in 0..rng.random_range(80..=170) {
    let location = pick(rng, locations);
    let date = random_datetime(ctx.now, rng);
    let start_ops = date - Duration::hours(rng.random_range(1..=6));
    let end_ops = start_ops + Duration::hours(rng.random_range(1..=6));

    let saved = physical_storage_transfer::ActiveModelEx {
      document_number: Set(random_document_number(&ctx.tag, "PST", serial)),
      date: Set(date),
      status: Set(DocumentStatus::Executed),
      version: Set(1),
      executed_at: Set(Some(
        end_ops + Duration::minutes(rng.random_range(10..=120)),
      )),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      contractor_id: Set(*pick(rng, &refs.companies.contractors)),
      start_cargo_ops: Set(start_ops),
      end_cargo_ops: Set(end_ops),
      items: (0..rng.random_range(1..=4))
        .map(|_| {
          let from_storage_id = location.pick_storage(rng);
          let mut to_storage_id = location.pick_storage(rng);
          while to_storage_id == from_storage_id {
            to_storage_id = location.pick_storage(rng);
          }

          physical_transfer_item::ActiveModelEx {
            product_id: Set(*pick(rng, &refs.target_product_ids)),
            from_storage_id: Set(from_storage_id),
            to_storage_id: Set(to_storage_id),
            amount: Set(Decimal::from(rng.random_range(35u64..=6_000u64))),
            ..Default::default()
          }
        })
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
    .save(ctx.conn)
    .await?;

    let physical_id = saved_uuid(saved.id, "physical transfer")?;
    counts.physical_transfers += 1;
    ctx
      .audit
      .backfill_document_routing::<physical_storage_transfer::Entity>(ctx.conn, physical_id)
      .await?;
  }

  Ok(())
}

async fn seed_reconciliations(
  ctx: &DocumentContext<'_>,
  rng: &mut rand::rngs::StdRng,
  refs: &ReferencePool,
  locations: &[LocationSeed],
  counts: &mut DocumentCounts,
) -> Result<(), ApiError> {
  let adjustment_types = [AdjustmentType::Surplus, AdjustmentType::Loss];

  for serial in 0..rng.random_range(50..=120) {
    let location = pick(rng, locations);
    let warehouse = pick(rng, &location.warehouses);
    let date = random_datetime(ctx.now, rng);
    let saved = inventory_reconciliation::ActiveModelEx {
      document_number: Set(random_document_number(&ctx.tag, "REC", serial)),
      date: Set(date),
      status: Set(DocumentStatus::Executed),
      version: Set(1),
      executed_at: Set(Some(date + Duration::minutes(rng.random_range(30..=240)))),
      executed_by: Set(None),
      reverted_at: Set(None),
      reverted_by: Set(None),
      contractor_id: Set(*pick(rng, &refs.companies.contractors)),
      warehouse_id: Set(warehouse.warehouse_id),
      adjustments: (0..rng.random_range(1..=5))
        .map(|_| inventory_adjustment::ActiveModelEx {
          storage_id: Set(*pick(rng, &warehouse.storage_ids)),
          product_id: Set(*pick(rng, &refs.target_product_ids)),
          adjustment_type: Set(*pick(rng, &adjustment_types)),
          amount: Set(Decimal::from(rng.random_range(1u64..=900u64))),
          reason: Set(maybe(rng, 0.45, |_rng| CompanyName().fake::<String>())),
          ..Default::default()
        })
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
    .save(ctx.conn)
    .await?;

    let reconciliation_id = saved_uuid(saved.id, "reconciliation")?;
    counts.reconciliations += 1;
    ctx
      .audit
      .backfill_document_routing::<inventory_reconciliation::Entity>(ctx.conn, reconciliation_id)
      .await?;
  }

  Ok(())
}

fn pick_waybill_for_base(
  rng: &mut rand::rngs::StdRng,
  waybills: &[WaybillSeed],
  base_id: Uuid,
) -> Option<Uuid> {
  let mut selected = None;

  for (seen, waybill) in waybills
    .iter()
    .filter(|waybill| waybill.base_id == base_id)
    .enumerate()
  {
    let seen = seen + 1;
    if rng.random_range(0..seen) == 0 {
      selected = Some(waybill.id);
    }
  }

  selected
}
