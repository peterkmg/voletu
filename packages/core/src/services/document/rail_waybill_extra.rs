use std::collections::HashSet;

use sea_orm::{
  entity::prelude::*,
  ColumnTrait,
  Condition,
  EntityLoaderTrait,
  QueryFilter,
  QueryOrder,
  TransactionTrait,
};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::{self, response::flow::RailReceiptPipelineResponse},
  entities::{
    acceptance_document,
    acceptance_item,
    company,
    product,
    rail_wagon_manifest,
    rail_wagon_measurement,
    rail_wagon_weight,
    rail_waybill,
  },
  enums::PipelineStatus,
  services::{
    document::specs::{RailReceiptPipelineQuerySpec, RailWaybillQuerySpec},
    DocumentService,
  },
};

impl DocumentService {
  pub(super) async fn rail_waybill_composite_model(
    &self,
    id: Uuid,
  ) -> Result<rail_waybill::ModelEx, ApiError> {
    rail_waybill::Entity::load()
      .filter_by_id(id)
      .filter(rail_waybill::Column::DeletedAt.is_null())
      .with(company::Entity)
      .with((rail_wagon_manifest::Entity, product::Entity))
      .with((rail_wagon_manifest::Entity, rail_wagon_measurement::Entity))
      .with((rail_wagon_manifest::Entity, rail_wagon_weight::Entity))
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Rail waybill '{}' not found", id)))
  }

  pub(super) async fn rail_waybill_query_models(
    &self,
    query: &RailWaybillQuerySpec,
  ) -> Result<Vec<rail_waybill::ModelEx>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(rail_waybill::Column::DeletedAt.is_null());

    if let Some(document_number) = query.document_number.as_deref() {
      condition = condition.add(rail_waybill::Column::DocumentNumber.contains(document_number));
    }

    if let Some(sender_id) = query.sender_id {
      condition = condition.add(rail_waybill::Column::SenderId.eq(sender_id));
    }

    Ok(
      rail_waybill::Entity::load()
        .filter(condition)
        .with(company::Entity)
        .paginate(self.db.as_ref(), per_page)
        .fetch_page(page - 1)
        .await?,
    )
  }

  pub async fn rail_waybill_query(
    &self,
    query: RailWaybillQuerySpec,
  ) -> Result<Vec<dtos::RailWaybillResponse>, ApiError> {
    Ok(
      self
        .rail_waybill_query_models(&query)
        .await?
        .into_iter()
        .map(|doc| dtos::RailWaybillResponse::from(rail_waybill::Model::from(doc)))
        .collect(),
    )
  }

  pub async fn rail_waybill_composite_create(
    &self,
    req: &dtos::RailWaybillCompositeRequest,
  ) -> Result<dtos::RailWaybillCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;
    let response = self.rail_waybill_composite_create_no_tx(&txn, req).await?;
    txn.commit().await?;
    Ok(response)
  }

  pub(crate) async fn rail_waybill_composite_create_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    req: &dtos::RailWaybillCompositeRequest,
  ) -> Result<dtos::RailWaybillCompositeResponse, ApiError> {
    let saved = rail_waybill::ActiveModelEx::from(req).save(conn).await?;
    let waybill_id = match saved.id {
      sea_orm::ActiveValue::Set(id) | sea_orm::ActiveValue::Unchanged(id) => id,
      sea_orm::ActiveValue::NotSet => {
        return Err(ApiError::Internal(anyhow::anyhow!(
          "rail waybill graph save returned no id"
        )));
      }
    };
    let doc = rail_waybill::Entity::load()
      .filter_by_id(waybill_id)
      .filter(rail_waybill::Column::DeletedAt.is_null())
      .with(company::Entity)
      .with((rail_wagon_manifest::Entity, product::Entity))
      .with((rail_wagon_manifest::Entity, rail_wagon_measurement::Entity))
      .with((rail_wagon_manifest::Entity, rail_wagon_weight::Entity))
      .one(conn)
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Rail waybill '{}' not found", waybill_id)))?;

    let manifests: Vec<dtos::RailWagonManifestResponse> = doc
      .wagon_manifests
      .iter()
      .cloned()
      .map(dtos::RailWagonManifestResponse::from)
      .collect();

    let waybill = dtos::RailWaybillResponse::from(rail_waybill::Model::from(doc));

    Ok(dtos::RailWaybillCompositeResponse {
      waybill,
      wagon_manifests: if manifests.is_empty() {
        None
      } else {
        Some(manifests)
      },
    })
  }

  /// Composite update: applies a header partial update plus a recursive diff
  /// over manifests and their nested measurements / weights.
  ///
  /// Diff semantics at every level:
  /// - rows with `id: Some(uuid)` matching an existing row are updated;
  /// - rows with `id: None` are inserted;
  /// - existing rows not present in the request are hard-deleted.
  ///
  /// When a manifest is deleted, its measurements and weights are deleted
  /// inline (no reliance on FK ON DELETE CASCADE) so audit log entries are
  /// produced for every removed row.
  pub async fn rail_waybill_composite_update(
    &self,
    rail_waybill_id: Uuid,
    req: &dtos::UpdateRailWaybillCompositeRequest,
  ) -> Result<dtos::RailWaybillCompositeResponse, ApiError> {
    let txn = self.db.begin().await?;
    let res = self
      .rail_waybill_composite_update_no_tx(&txn, rail_waybill_id, req)
      .await?;
    txn.commit().await?;
    Ok(res)
  }

  pub(crate) async fn rail_waybill_composite_update_no_tx(
    &self,
    conn: &sea_orm::DatabaseTransaction,
    rail_waybill_id: Uuid,
    req: &dtos::UpdateRailWaybillCompositeRequest,
  ) -> Result<dtos::RailWaybillCompositeResponse, ApiError> {
    // 1. Header update via the macro-generated per-row updater.
    self
      .rail_waybill_update_no_tx(conn, rail_waybill_id, &req.waybill)
      .await?;

    // 2. Reject duplicate `Some(id)` entries at every nesting level before
    //    touching the database. The HashSets double as the dedup guards.
    let mut kept_manifest_ids: HashSet<Uuid> = HashSet::new();
    for manifest in &req.manifests {
      if let Some(manifest_id) = manifest.id {
        if !kept_manifest_ids.insert(manifest_id) {
          return Err(ApiError::BadRequest(format!(
            "duplicate manifest id in request: {}",
            manifest_id
          )));
        }
      }
      let mut kept_measurement_ids: HashSet<Uuid> = HashSet::new();
      for measurement in &manifest.measurements {
        if let Some(row_id) = measurement.id {
          if !kept_measurement_ids.insert(row_id) {
            return Err(ApiError::BadRequest(format!(
              "duplicate measurement id in request: {}",
              row_id
            )));
          }
        }
      }
      let mut kept_weight_ids: HashSet<Uuid> = HashSet::new();
      for weight in &manifest.weights {
        if let Some(row_id) = weight.id {
          if !kept_weight_ids.insert(row_id) {
            return Err(ApiError::BadRequest(format!(
              "duplicate weight id in request: {}",
              row_id
            )));
          }
        }
      }
    }

    // 3. Build the nested graph and persist with a single graph-save call.
    //    `HasManyModel::Replace(_)` at every nesting level deletes existing
    //    related rows missing from the new set; SeaORM recurses into each
    //    surviving child's own `action()` so the diff propagates from the
    //    waybill down through manifests to measurements and weights.
    let manifests: Vec<rail_wagon_manifest::ActiveModelEx> = req
      .manifests
      .iter()
      .map(|manifest| {
        let measurements: Vec<rail_wagon_measurement::ActiveModelEx> = manifest
          .measurements
          .iter()
          .map(|measurement| rail_wagon_measurement::ActiveModelEx {
            id: match measurement.id {
              Some(id) => sea_orm::ActiveValue::Unchanged(id),
              None => sea_orm::ActiveValue::NotSet,
            },
            measured_height: sea_orm::ActiveValue::Set(measurement.measured_height),
            lab_density: sea_orm::ActiveValue::Set(measurement.lab_density),
            calculated_mass: sea_orm::ActiveValue::Set(measurement.calculated_mass),
            ..Default::default()
          })
          .collect();
        let weights: Vec<rail_wagon_weight::ActiveModelEx> = manifest
          .weights
          .iter()
          .map(|weight| rail_wagon_weight::ActiveModelEx {
            id: match weight.id {
              Some(id) => sea_orm::ActiveValue::Unchanged(id),
              None => sea_orm::ActiveValue::NotSet,
            },
            gross_weight: sea_orm::ActiveValue::Set(weight.gross_weight),
            tare_weight: sea_orm::ActiveValue::Set(weight.tare_weight),
            net_product_weight: sea_orm::ActiveValue::Set(weight.net_product_weight),
            ..Default::default()
          })
          .collect();

        rail_wagon_manifest::ActiveModelEx {
          id: match manifest.id {
            Some(id) => sea_orm::ActiveValue::Unchanged(id),
            None => sea_orm::ActiveValue::NotSet,
          },
          wagon_number: sea_orm::ActiveValue::Set(manifest.wagon_number.clone()),
          product_id: sea_orm::ActiveValue::Set(manifest.product_id),
          declared_volume: sea_orm::ActiveValue::Set(manifest.declared_volume),
          declared_density: sea_orm::ActiveValue::Set(manifest.declared_density),
          declared_mass: sea_orm::ActiveValue::Set(manifest.declared_mass),
          measurements: sea_orm::HasManyModel::Replace(measurements),
          weights: sea_orm::HasManyModel::Replace(weights),
          ..Default::default()
        }
      })
      .collect();

    rail_waybill::ActiveModelEx {
      id: sea_orm::ActiveValue::Unchanged(rail_waybill_id),
      wagon_manifests: sea_orm::HasManyModel::Replace(manifests),
      ..Default::default()
    }
    .save(conn)
    .await?;

    // 4. Reload the full composite using the same eager-loading shape as create.
    let doc = rail_waybill::Entity::load()
      .filter_by_id(rail_waybill_id)
      .filter(rail_waybill::Column::DeletedAt.is_null())
      .with(company::Entity)
      .with((rail_wagon_manifest::Entity, product::Entity))
      .with((rail_wagon_manifest::Entity, rail_wagon_measurement::Entity))
      .with((rail_wagon_manifest::Entity, rail_wagon_weight::Entity))
      .one(conn)
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Rail waybill '{}' not found", rail_waybill_id)))?;

    let manifests: Vec<dtos::RailWagonManifestResponse> = doc
      .wagon_manifests
      .iter()
      .cloned()
      .map(dtos::RailWagonManifestResponse::from)
      .collect();

    let waybill = dtos::RailWaybillResponse::from(rail_waybill::Model::from(doc));

    Ok(dtos::RailWaybillCompositeResponse {
      waybill,
      wagon_manifests: if manifests.is_empty() {
        None
      } else {
        Some(manifests)
      },
    })
  }

  pub async fn rail_waybill_composite_get(
    &self,
    id: Uuid,
  ) -> Result<dtos::RailWaybillCompositeResponse, ApiError> {
    let doc = self.rail_waybill_composite_model(id).await?;

    let manifests: Vec<dtos::RailWagonManifestResponse> = doc
      .wagon_manifests
      .iter()
      .cloned()
      .map(dtos::RailWagonManifestResponse::from)
      .collect();

    let waybill = dtos::RailWaybillResponse::from(rail_waybill::Model::from(doc));

    Ok(dtos::RailWaybillCompositeResponse {
      waybill,
      wagon_manifests: if manifests.is_empty() {
        None
      } else {
        Some(manifests)
      },
    })
  }

  pub async fn rail_receipt_pipeline_query(
    &self,
    query: RailReceiptPipelineQuerySpec,
  ) -> Result<Vec<RailReceiptPipelineResponse>, ApiError> {
    let (page, per_page) =
      crate::services::common::normalize_pagination(query.page, query.per_page)?;
    let db = self.db.as_ref();

    let mut cond = Condition::all().add(rail_waybill::Column::DeletedAt.is_null());
    if let Some(cid) = query.contractor_id {
      cond = cond.add(rail_waybill::Column::SenderId.eq(cid));
    }

    let waybills: Vec<rail_waybill::ModelEx> = rail_waybill::Entity::load()
      .filter(cond)
      .with(company::Entity)
      .with((rail_wagon_manifest::Entity, product::Entity))
      .with((acceptance_document::Entity, acceptance_item::Entity))
      .order_by_desc(rail_waybill::Column::Date)
      .paginate(db, per_page)
      .fetch_page(page - 1)
      .await?;

    let mut rows = Vec::with_capacity(waybills.len());
    for wb in &waybills {
      let acc = wb.acceptances.get(0);
      let status = PipelineStatus::from_doc_status(acc.map(|a| &a.status));

      if query.pipeline_status.is_some() && query.pipeline_status != Some(status) {
        continue;
      }

      let manifest = wb.wagon_manifests.get(0);
      let actual_quantity: Decimal = acc
        .map(|a| a.items.iter().map(|i| i.accepted_amount).sum())
        .unwrap_or(Decimal::ZERO);

      rows.push(RailReceiptPipelineResponse {
        id: wb.id,
        basis_document_number: wb.document_number.clone(),
        basis_date: wb.date.to_string(),
        contractor_id: wb.sender_id,
        contractor_name: wb
          .sender
          .as_ref()
          .map(|s| s.common_name.clone())
          .unwrap_or_default(),
        product_name: manifest.and_then(|m| m.product.as_ref().map(|p| p.common_name.clone())),
        expected_quantity: manifest.map(|m| m.declared_mass),
        action_id: acc.map(|a| a.id),
        action_document_number: acc.map(|a| a.document_number.clone()),
        actual_quantity: if actual_quantity > Decimal::ZERO {
          Some(actual_quantity)
        } else {
          None
        },
        pipeline_status: status,
      });
    }

    Ok(rows)
  }
}
