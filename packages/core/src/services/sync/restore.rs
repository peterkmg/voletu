use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, DbErr, EntityLoaderTrait, SqlErr};
use serde_json::Value;
use uuid::Uuid;

use crate::{
  api::ApiError,
  entities::*,
  enums::{AuditAction, AuditTable},
};

fn is_nonfatal_restore_error(error: &DbErr) -> bool {
  matches!(error, DbErr::PrimaryKeyNotSet { .. } | DbErr::AttrNotSet(_))
}

fn is_idempotent_insert_error(error: &DbErr) -> bool {
  matches!(error, DbErr::RecordNotInserted)
    || matches!(error.sql_err(), Some(SqlErr::UniqueConstraintViolation(_)))
}

fn inventory_ledger_entry_id(active_model: &inventory_ledger_entry::ActiveModel) -> Option<Uuid> {
  match &active_model.id {
    ActiveValue::Set(id) | ActiveValue::Unchanged(id) => Some(*id),
    ActiveValue::NotSet => None,
  }
}

async fn inventory_ledger_entry_exists<C: ConnectionTrait>(
  conn: &C,
  id: Uuid,
) -> Result<bool, ApiError> {
  Ok(
    inventory_ledger_entry::Entity::load()
      .filter_by_id(id)
      .one(conn)
      .await?
      .is_some(),
  )
}

async fn restore_inventory_ledger_entry<C: ConnectionTrait>(
  conn: &C,
  action: AuditAction,
  new_values: Option<&Value>,
) -> Result<(), ApiError> {
  match action {
    AuditAction::Insert => {
      if let Some(snapshot) = new_values {
        let active_model = match inventory_ledger_entry::ActiveModel::from_json(snapshot.clone()) {
          Ok(model) => model,
          Err(error) if is_nonfatal_restore_error(&error) => return Ok(()),
          Err(error) => return Err(error.into()),
        };
        let Some(row_id) = inventory_ledger_entry_id(&active_model) else {
          return Ok(());
        };

        match active_model.insert(conn).await {
          Ok(_) => Ok(()),
          Err(error) if is_nonfatal_restore_error(&error) => Ok(()),
          Err(error) if is_idempotent_insert_error(&error) => {
            if inventory_ledger_entry_exists(conn, row_id).await? {
              Ok(())
            } else {
              Err(error.into())
            }
          }
          Err(error) => Err(error.into()),
        }
      } else {
        Ok(())
      }
    }
    AuditAction::Update | AuditAction::HardDelete => Ok(()),
  }
}

pub(super) async fn apply_audit_log_restore<C: ConnectionTrait>(
  conn: &C,
  table: AuditTable,
  action: AuditAction,
  old_values: Option<&Value>,
  new_values: Option<&Value>,
) -> Result<(), ApiError> {
  macro_rules! restore_table {
    ($active_model:path) => {{
      match action {
        AuditAction::Insert | AuditAction::Update => {
          if let Some(snapshot) = new_values {
            let active_model =
              match <$active_model as ActiveModelTrait>::from_json(snapshot.clone()) {
                Ok(model) => model,
                Err(error) if is_nonfatal_restore_error(&error) => return Ok(()),
                Err(error) => return Err(error.into()),
              };

            match active_model.update(conn).await {
              Ok(_) => Ok(()),
              Err(DbErr::RecordNotFound(_)) | Err(DbErr::RecordNotUpdated) => {
                let insert_model =
                  match <$active_model as ActiveModelTrait>::from_json(snapshot.clone()) {
                    Ok(model) => model,
                    Err(error) if is_nonfatal_restore_error(&error) => return Ok(()),
                    Err(error) => return Err(error.into()),
                  };
                match insert_model.insert(conn).await {
                  Ok(_) => {}
                  Err(error) if is_nonfatal_restore_error(&error) => return Ok(()),
                  Err(error) => return Err(error.into()),
                }
                Ok(())
              }
              Err(error) if is_nonfatal_restore_error(&error) => Ok(()),
              Err(error) => Err(error.into()),
            }
          } else {
            Ok(())
          }
        }
        AuditAction::HardDelete => {
          if let Some(snapshot) = old_values.or(new_values) {
            let active_model =
              match <$active_model as ActiveModelTrait>::from_json(snapshot.clone()) {
                Ok(model) => model,
                Err(error) if is_nonfatal_restore_error(&error) => return Ok(()),
                Err(error) => return Err(error.into()),
              };

            match active_model.delete(conn).await {
              Ok(_) | Err(DbErr::RecordNotFound(_)) => Ok(()),
              Err(error) if is_nonfatal_restore_error(&error) => Ok(()),
              Err(error) => Err(error.into()),
            }
          } else {
            Ok(())
          }
        }
      }
    }};
  }

  match table {
    AuditTable::AuditLogs | AuditTable::Local | AuditTable::Roles => Ok(()),
    AuditTable::AcceptanceDocuments => restore_table!(acceptance_document::ActiveModel),
    AuditTable::AcceptanceItems => restore_table!(acceptance_item::ActiveModel),
    AuditTable::Bases => restore_table!(base::ActiveModel),
    AuditTable::BlendingComponents => restore_table!(blending_component::ActiveModel),
    AuditTable::BlendingDocuments => restore_table!(blending_document::ActiveModel),
    AuditTable::BlendingResults => restore_table!(blending_result::ActiveModel),
    AuditTable::Companies => restore_table!(company::ActiveModel),
    AuditTable::DatabaseInstances => restore_table!(database_instance::ActiveModel),
    AuditTable::DispatchDocuments => restore_table!(dispatch_document::ActiveModel),
    AuditTable::DispatchItems => restore_table!(dispatch_item::ActiveModel),
    AuditTable::DispatchStorageMeasurements => {
      restore_table!(dispatch_storage_measurement::ActiveModel)
    }
    AuditTable::InventoryAdjustments => restore_table!(inventory_adjustment::ActiveModel),
    AuditTable::InventoryLedgerEntries => {
      restore_inventory_ledger_entry(conn, action, new_values).await
    }
    AuditTable::InventoryReconciliations => restore_table!(inventory_reconciliation::ActiveModel),
    AuditTable::OwnershipTransfers => restore_table!(ownership_transfer::ActiveModel),
    AuditTable::OwnershipTransferItems => restore_table!(ownership_transfer_item::ActiveModel),
    AuditTable::PhysicalStorageTransfers => restore_table!(physical_storage_transfer::ActiveModel),
    AuditTable::PhysicalTransferItems => restore_table!(physical_transfer_item::ActiveModel),
    AuditTable::Ports => restore_table!(port::ActiveModel),
    AuditTable::Products => restore_table!(product::ActiveModel),
    AuditTable::ProductGroups => restore_table!(product_group::ActiveModel),
    AuditTable::ProductTypes => restore_table!(product_type::ActiveModel),
    AuditTable::RailWagonManifests => restore_table!(rail_wagon_manifest::ActiveModel),
    AuditTable::RailWagonMeasurements => restore_table!(rail_wagon_measurement::ActiveModel),
    AuditTable::RailWagonWeights => restore_table!(rail_wagon_weight::ActiveModel),
    AuditTable::RailWaybills => restore_table!(rail_waybill::ActiveModel),
    AuditTable::RefreshTokens => restore_table!(refresh_token::ActiveModel),
    AuditTable::Storages => restore_table!(storage::ActiveModel),
    AuditTable::SyncWatermarks => restore_table!(sync_watermark::ActiveModel),
    AuditTable::TruckWaybills => restore_table!(truck_waybill::ActiveModel),
    AuditTable::TruckWaybillItems => restore_table!(truck_waybill_item::ActiveModel),
    AuditTable::TruckWeightDocs => restore_table!(truck_weight_doc::ActiveModel),
    AuditTable::Users => restore_table!(user::ActiveModel),
    AuditTable::Warehouses => restore_table!(warehouse::ActiveModel),
  }
}
