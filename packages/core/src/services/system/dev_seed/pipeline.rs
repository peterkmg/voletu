use anyhow::anyhow;
use chrono::{DateTime, Utc};
use rand::{RngExt, SeedableRng};
use sea_orm::{
  prelude::Decimal,
  ActiveModelTrait,
  ActiveValue::Set,
  DatabaseTransaction,
  EntityLoaderTrait,
  EntityTrait,
  PaginatorTrait,
  TransactionError,
  TransactionTrait,
};
use strum::VariantArray;
use uuid::Uuid;

use super::{
  documents::{seed_inventory_documents, seed_transport_documents},
  helpers::{
    fake_fragment,
    numbered_name,
    pick,
    random_username,
    saved_uuid,
    title_fragment,
    versioned_name,
    SeedTag,
    BASE_SUFFIXES,
    COMPANY_ROLE_TAILS,
    LEGAL_SUFFIXES,
    PRODUCT_FAMILIES,
    STORAGE_LABELS,
  },
};
use crate::{
  api::ApiError,
  context::audit::with_audit_context,
  dtos::SeedResult,
  entities::{
    base,
    company,
    inventory_ledger_entry,
    port,
    product,
    product_group,
    product_type,
    storage,
    user,
    warehouse,
  },
  enums::{LedgerEntrySourceEvent, LedgerEntrySourceKind, RoleType},
  services::{audit::AuditService, system::local::load_local_bootstrap, SystemService},
  utils::password::hash_password,
};

#[derive(Clone)]
pub(super) struct SeedContext<'a> {
  pub(super) conn: &'a DatabaseTransaction,
  pub(super) audit: &'a AuditService,
  pub(super) now: DateTime<Utc>,
  pub(super) tag: SeedTag,
  pub(super) name_offsets: SeedNameOffsets,
  pub(super) dev_password_hash: &'a str,
}

impl<'a> SeedContext<'a> {
  fn new(
    conn: &'a DatabaseTransaction,
    audit: &'a AuditService,
    now: DateTime<Utc>,
    tag: SeedTag,
    name_offsets: SeedNameOffsets,
    dev_password_hash: &'a str,
  ) -> Self {
    Self {
      conn,
      audit,
      now,
      tag,
      name_offsets,
      dev_password_hash,
    }
  }
}

pub(super) type DocumentContext<'a> = SeedContext<'a>;

#[derive(Clone, Copy, Debug)]
pub(super) struct SeedNameOffsets {
  product_types: usize,
  product_groups: usize,
  products: usize,
  companies: usize,
  ports: usize,
  bases: usize,
  users: usize,
}

#[derive(Clone, Debug)]
pub(super) struct CompanyPool {
  pub(super) all: Vec<Uuid>,
  pub(super) contractors: Vec<Uuid>,
  pub(super) exporters: Vec<Uuid>,
  pub(super) senders: Vec<Uuid>,
}

#[derive(Clone, Debug)]
pub(super) struct ReferencePool {
  pub(super) product_type_ids: Vec<Uuid>,
  pub(super) product_group_ids: Vec<Uuid>,
  pub(super) product_ids: Vec<Uuid>,
  pub(super) component_product_ids: Vec<Uuid>,
  pub(super) target_product_ids: Vec<Uuid>,
  pub(super) companies: CompanyPool,
  pub(super) port_ids: Vec<Uuid>,
}

#[derive(Clone, Debug)]
pub(super) struct WarehouseSeed {
  pub(super) warehouse_id: Uuid,
  pub(super) storage_ids: Vec<Uuid>,
}

#[derive(Clone, Debug)]
pub(super) struct LocationSeed {
  pub(super) base_id: Uuid,
  pub(super) warehouses: Vec<WarehouseSeed>,
}

impl LocationSeed {
  pub(super) fn pick_storage(&self, rng: &mut rand::rngs::StdRng) -> Uuid {
    let warehouse = pick(rng, &self.warehouses);
    *pick(rng, &warehouse.storage_ids)
  }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct WaybillSeed {
  pub(super) id: Uuid,
  pub(super) base_id: Uuid,
}

#[derive(Clone, Debug, Default)]
pub(super) struct TransportPool {
  pub(super) truck: Vec<WaybillSeed>,
  pub(super) rail: Vec<WaybillSeed>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct DocumentCounts {
  pub(super) acceptance_docs: usize,
  pub(super) dispatch_docs: usize,
  pub(super) blending_docs: usize,
  pub(super) ownership_transfers: usize,
  pub(super) physical_transfers: usize,
  pub(super) reconciliations: usize,
}

impl SystemService {
  pub async fn dev_seed(&self) -> Result<SeedResult, ApiError> {
    let local_cfg = load_local_bootstrap(self.db.as_ref()).await?;
    let actor_id = Uuid::now_v7();
    let origin_db_id = local_cfg.local_db_id;

    with_audit_context(actor_id, origin_db_id, || async {
      self.seed_dev_batch().await
    })
    .await
  }

  async fn seed_dev_batch(&self) -> Result<SeedResult, ApiError> {
    let now = Utc::now();
    let run_id = Uuid::now_v7();
    let dev_password_hash = hash_password("password123")
      .await
      .map_err(ApiError::Internal)?;
    let audit = self.audit.clone();

    self
      .db
      .transaction::<_, SeedResult, ApiError>(|txn| {
        let audit = audit.clone();
        let dev_password_hash = dev_password_hash.clone();
        Box::pin(async move {
          let name_offsets = seed_name_offsets(txn).await?;
          let display_sequence = display_sequence_from_existing(name_offsets.product_types);
          let ctx = SeedContext::new(
            txn,
            audit.as_ref(),
            now,
            SeedTag::new(now, run_id, display_sequence),
            name_offsets,
            &dev_password_hash,
          );
          let mut rng = rng_from_run(run_id);

          let refs = seed_reference_data(&ctx, &mut rng).await?;
          let locations = seed_locations(&ctx, &mut rng, &refs.product_type_ids).await?;
          let user_count = seed_users(&ctx, &mut rng, &locations).await?;
          let transport = seed_transport_documents(&ctx, &mut rng, &refs, &locations).await?;
          let documents =
            seed_inventory_documents(&ctx, &mut rng, &refs, &locations, &transport).await?;
          let ledger_entries = seed_ledger(&ctx, &mut rng, &refs, &locations).await?;

          Ok(SeedResult {
            product_types: refs.product_type_ids.len(),
            product_groups: refs.product_group_ids.len(),
            products: refs.product_ids.len(),
            companies: refs.companies.all.len(),
            ports: refs.port_ids.len(),
            bases: locations.len(),
            warehouses: locations
              .iter()
              .map(|location| location.warehouses.len())
              .sum(),
            storages: locations
              .iter()
              .flat_map(|location| location.warehouses.iter())
              .map(|warehouse| warehouse.storage_ids.len())
              .sum(),
            users: user_count,
            truck_waybills: transport.truck.len(),
            rail_waybills: transport.rail.len(),
            acceptance_docs: documents.acceptance_docs,
            dispatch_docs: documents.dispatch_docs,
            blending_docs: documents.blending_docs,
            ownership_transfers: documents.ownership_transfers,
            physical_transfers: documents.physical_transfers,
            reconciliations: documents.reconciliations,
            ledger_entries,
          })
        })
      })
      .await
      .map_err(|error| match error {
        TransactionError::Connection(db_error) => ApiError::Database(db_error),
        TransactionError::Transaction(api_error) => api_error,
      })
  }
}

fn rng_from_run(run_id: Uuid) -> rand::rngs::StdRng {
  let mut seed = [0u8; 32];
  seed[..16].copy_from_slice(run_id.as_bytes());
  seed[16..].copy_from_slice(run_id.as_bytes());
  rand::rngs::StdRng::from_seed(seed)
}

async fn seed_name_offsets(txn: &DatabaseTransaction) -> Result<SeedNameOffsets, ApiError> {
  Ok(SeedNameOffsets {
    product_types: product_type::Entity::find().count(txn).await? as usize,
    product_groups: product_group::Entity::find().count(txn).await? as usize,
    products: product::Entity::find().count(txn).await? as usize,
    companies: company::Entity::find().count(txn).await? as usize,
    ports: port::Entity::find().count(txn).await? as usize,
    bases: base::Entity::find().count(txn).await? as usize,
    users: user::Entity::find().count(txn).await? as usize,
  })
}

fn display_sequence_from_existing(existing_product_types: usize) -> usize {
  ((existing_product_types + PRODUCT_FAMILIES.len() - 1) / PRODUCT_FAMILIES.len()) + 1
}

async fn seed_reference_data(
  ctx: &SeedContext<'_>,
  rng: &mut rand::rngs::StdRng,
) -> Result<ReferencePool, ApiError> {
  let mut product_type_ids = Vec::with_capacity(PRODUCT_FAMILIES.len());
  let mut product_group_ids = Vec::new();
  let mut product_ids = Vec::new();
  let mut component_product_ids = Vec::new();
  let mut target_product_ids = Vec::new();

  for family in PRODUCT_FAMILIES {
    let product_type = product_type::ActiveModel {
      common_name: Set(versioned_name(&ctx.tag, *family)),
      long_name: Set(Some(format!("{family} {}", fake_fragment(2..5)))),
      ..Default::default()
    }
    .insert(ctx.conn)
    .await?;
    product_type_ids.push(product_type.id);

    for group_index in 0..rng.random_range(2..=3) {
      let group_serial = ctx.name_offsets.product_groups + product_group_ids.len();
      let group = product_group::ActiveModel {
        product_type_id: Set(product_type.id),
        common_name: Set(numbered_name(format!("{family} Group"), group_serial)),
        long_name: Set(Some(format!("{family} {}", fake_fragment(3..6)))),
        ..Default::default()
      }
      .insert(ctx.conn)
      .await?;
      product_group_ids.push(group.id);

      for product_index in 0..rng.random_range(2..=4) {
        let is_component = group_index == 0 || (group_index == 1 && product_index == 0);
        let product_serial = ctx.name_offsets.products + product_ids.len();
        let product_name = if is_component {
          numbered_name(format!("{family} Component"), product_serial)
        } else {
          numbered_name(format!("{family} Blend"), product_serial)
        };

        let saved = product::ActiveModel {
          product_group_id: Set(group.id),
          manufacturer_id: Set(None),
          common_name: Set(product_name),
          long_name: Set(Some(format!("{family} {}", fake_fragment(4..8)))),
          add_identification: Set(None),
          is_component: Set(is_component),
          ..Default::default()
        }
        .insert(ctx.conn)
        .await?;

        product_ids.push(saved.id);
        if is_component {
          component_product_ids.push(saved.id);
        } else {
          target_product_ids.push(saved.id);
        }
      }
    }
  }

  let companies = seed_companies(ctx, rng).await?;
  let port_ids = seed_ports(ctx, rng).await?;

  if target_product_ids.is_empty() || component_product_ids.is_empty() {
    return Err(ApiError::Internal(anyhow!(
      "dev seed requires both target and component product pools"
    )));
  }

  Ok(ReferencePool {
    product_type_ids,
    product_group_ids,
    product_ids,
    component_product_ids,
    target_product_ids,
    companies,
    port_ids,
  })
}

async fn seed_companies(
  ctx: &SeedContext<'_>,
  _rng: &mut rand::rngs::StdRng,
) -> Result<CompanyPool, ApiError> {
  let mut all = Vec::new();
  let mut contractors = Vec::new();
  let mut exporters = Vec::new();
  let mut senders = Vec::new();

  for index in 0..18 {
    let company_serial = ctx.name_offsets.companies + index;
    let role_tail = COMPANY_ROLE_TAILS[index % COMPANY_ROLE_TAILS.len()];
    let company_common_name = numbered_name(format!("{role_tail} Company"), company_serial);

    let saved = company::ActiveModel {
      common_name: Set(company_common_name.clone()),
      legal_name: Set(Some(format!(
        "{} {}",
        company_common_name,
        LEGAL_SUFFIXES[index % LEGAL_SUFFIXES.len()]
      ))),
      is_contractor: Set(index < 10 || index % 3 == 0),
      is_exporter: Set((4..12).contains(&index)),
      is_manufacturer: Set(index % 2 == 0),
      is_sender: Set(index < 8 || index % 5 == 0),
      ..Default::default()
    }
    .insert(ctx.conn)
    .await?;

    all.push(saved.id);
    if saved.is_contractor {
      contractors.push(saved.id);
    }
    if saved.is_exporter {
      exporters.push(saved.id);
    }
    if saved.is_sender {
      senders.push(saved.id);
    }
  }

  Ok(CompanyPool {
    all,
    contractors,
    exporters,
    senders,
  })
}

async fn seed_ports(
  ctx: &SeedContext<'_>,
  _rng: &mut rand::rngs::StdRng,
) -> Result<Vec<Uuid>, ApiError> {
  let mut port_ids = Vec::with_capacity(8);
  for index in 0..8 {
    let saved = port::ActiveModel {
      common_name: Set(numbered_name("Port", ctx.name_offsets.ports + index)),
      country: Set(Some(title_fragment(fake_fragment(1..3)))),
      ..Default::default()
    }
    .insert(ctx.conn)
    .await?;
    port_ids.push(saved.id);
  }

  Ok(port_ids)
}

async fn seed_locations(
  ctx: &SeedContext<'_>,
  rng: &mut rand::rngs::StdRng,
  product_type_ids: &[Uuid],
) -> Result<Vec<LocationSeed>, ApiError> {
  let mut locations = Vec::new();

  for _ in 0..rng.random_range(4..=6) {
    let warehouse_count = rng.random_range(2..=4);
    let base_serial = ctx.name_offsets.bases + locations.len();

    let graph = base::ActiveModelEx {
      common_name: Set(numbered_name(*pick(rng, BASE_SUFFIXES), base_serial)),
      long_name: Set(Some(fake_fragment(5..9))),
      warehouses: (0..warehouse_count)
        .map(|warehouse_index| warehouse::ActiveModelEx {
          common_name: Set(numbered_name("Warehouse", warehouse_index)),
          long_name: Set(Some(fake_fragment(3..6))),
          storages: (0..rng.random_range(2..=5))
            .map(|storage_index| {
              let is_type_specific = rng.random_bool(0.35);
              let product_type_id = if is_type_specific {
                Some(*pick(rng, product_type_ids))
              } else {
                None
              };

              storage::ActiveModelEx {
                common_name: Set(numbered_name(
                  STORAGE_LABELS[storage_index % STORAGE_LABELS.len()],
                  storage_index,
                )),
                long_name: Set(Some(fake_fragment(2..5))),
                capacity: Set(Some(Decimal::from(rng.random_range(4_000u64..=120_000u64)))),
                is_type_specific: Set(is_type_specific),
                product_type_id: Set(product_type_id),
                ..Default::default()
              }
            })
            .collect::<Vec<_>>()
            .into(),
          ..Default::default()
        })
        .collect::<Vec<_>>()
        .into(),
      ..Default::default()
    }
    .save(ctx.conn)
    .await?;

    let base_id = saved_uuid(graph.id, "base")?;

    let loaded = base::Entity::load()
      .filter_by_id(base_id)
      .with(warehouse::Entity)
      .with((warehouse::Entity, storage::Entity))
      .one(ctx.conn)
      .await?
      .ok_or_else(|| ApiError::Internal(anyhow!("seeded base missing after save")))?;

    locations.push(LocationSeed {
      base_id: loaded.id,
      warehouses: loaded
        .warehouses
        .into_iter()
        .map(|warehouse| WarehouseSeed {
          warehouse_id: warehouse.id,
          storage_ids: warehouse
            .storages
            .into_iter()
            .map(|storage| storage.id)
            .collect(),
        })
        .collect(),
    });
  }

  Ok(locations)
}

async fn seed_users(
  ctx: &SeedContext<'_>,
  rng: &mut rand::rngs::StdRng,
  locations: &[LocationSeed],
) -> Result<usize, ApiError> {
  let mut created = 0usize;
  let mut index = ctx.name_offsets.users;

  for role_type in RoleType::VARIANTS {
    let home_base_id = if matches!(role_type, RoleType::Admin) {
      None
    } else {
      Some(pick(rng, locations).base_id)
    };
    user::ActiveModel {
      username: Set(random_username(rng, &ctx.tag, index)),
      fullname: Set(Some(numbered_name(format!("{role_type} User"), index))),
      password_hash: Set(ctx.dev_password_hash.to_string()),
      role_id: Set(role_type.uuid()),
      home_base_id: Set(home_base_id),
      ..Default::default()
    }
    .insert(ctx.conn)
    .await?;
    created += 1;
    index += 1;
  }

  for location in locations {
    for role_type in [
      RoleType::SeniorSupervisor,
      RoleType::Supervisor,
      RoleType::Operator,
    ] {
      let repeats = if matches!(role_type, RoleType::Operator) {
        2
      } else {
        1
      };
      for _ in 0..repeats {
        user::ActiveModel {
          username: Set(random_username(rng, &ctx.tag, index)),
          fullname: Set(Some(numbered_name(format!("{role_type} User"), index))),
          password_hash: Set(ctx.dev_password_hash.to_string()),
          role_id: Set(role_type.uuid()),
          home_base_id: Set(Some(location.base_id)),
          ..Default::default()
        }
        .insert(ctx.conn)
        .await?;
        created += 1;
        index += 1;
      }
    }
  }

  Ok(created)
}

async fn seed_ledger(
  ctx: &SeedContext<'_>,
  rng: &mut rand::rngs::StdRng,
  refs: &ReferencePool,
  locations: &[LocationSeed],
) -> Result<usize, ApiError> {
  let mut created = 0usize;

  for (location_index, location) in locations.iter().enumerate() {
    for (warehouse_index, warehouse) in location.warehouses.iter().enumerate() {
      for (storage_index, storage_id) in warehouse.storage_ids.iter().copied().enumerate() {
        let product_id = refs.target_product_ids
          [(location_index + warehouse_index + storage_index) % refs.target_product_ids.len()];
        let contractor_id =
          refs.companies.contractors[(location_index * 7 + warehouse_index * 3 + storage_index)
            % refs.companies.contractors.len()];

        let amount = Decimal::from(rng.random_range(500u64..=40_000u64));

        inventory_ledger_entry::ActiveModel {
          id: Set(Uuid::now_v7()),
          storage_id: Set(storage_id),
          product_id: Set(product_id),
          contractor_id: Set(contractor_id),
          quantity_delta: Set(amount),
          source_kind: Set(LedgerEntrySourceKind::OpeningBalance),
          source_id: Set(Uuid::now_v7()),
          source_event: Set(LedgerEntrySourceEvent::OpeningBalance),
          reverses_entry_id: Set(None),
          ..Default::default()
        }
        .insert(ctx.conn)
        .await?;
        created += 1;

        if rng.random_bool(0.4) {
          let secondary_product_id =
            refs.target_product_ids[(location_index * 5 + warehouse_index + storage_index + 1)
              % refs.target_product_ids.len()];
          let secondary_contractor_id =
            refs.companies.contractors[(location_index + warehouse_index * 11 + storage_index + 1)
              % refs.companies.contractors.len()];

          let secondary_amount = Decimal::from(rng.random_range(150u64..=12_000u64));

          inventory_ledger_entry::ActiveModel {
            id: Set(Uuid::now_v7()),
            storage_id: Set(storage_id),
            product_id: Set(secondary_product_id),
            contractor_id: Set(secondary_contractor_id),
            quantity_delta: Set(secondary_amount),
            source_kind: Set(LedgerEntrySourceKind::OpeningBalance),
            source_id: Set(Uuid::now_v7()),
            source_event: Set(LedgerEntrySourceEvent::OpeningBalance),
            reverses_entry_id: Set(None),
            ..Default::default()
          }
          .insert(ctx.conn)
          .await?;
          created += 1;
        }
      }
    }
  }

  Ok(created)
}
