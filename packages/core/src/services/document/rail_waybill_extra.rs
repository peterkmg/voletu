use sea_orm::{
  entity::prelude::*,
  ColumnTrait,
  Condition,
  ConnectionTrait,
  EntityLoaderTrait,
  QueryFilter,
  QueryOrder,
  TransactionTrait,
};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos::{self, response::pipeline::RailReceiptPipelineResponse},
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
    document::query::{RailReceiptPipelineQuerySpec, RailWaybillQuerySpec},
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
    conn: &impl ConnectionTrait,
    req: &dtos::RailWaybillCompositeRequest,
  ) -> Result<dtos::RailWaybillCompositeResponse, ApiError> {
    let waybill = self
      .rail_waybill_create_no_tx(conn, &dtos::CreateRailWaybillRequest::from_composite(req))
      .await?;

    let mut wagon_manifests: Option<Vec<dtos::RailWagonManifestResponse>> = None;

    if let Some(manifests_req) = &req.manifests {
      for manifest_req in manifests_req {
        let mut manifest = self
          .rail_manifest_create_no_tx(
            conn,
            &dtos::CreateRailWagonManifestRequest::from_composite(waybill.id, manifest_req),
          )
          .await?;

        let mut measurements: Option<Vec<dtos::RailWagonMeasurementResponse>> = None;
        let mut weights: Option<Vec<dtos::RailWagonWeightResponse>> = None;

        if let Some(measurement_reqs) = &manifest_req.measurements {
          for measurement_req in measurement_reqs {
            let measurement = self
              .rail_measurement_create_no_tx(
                conn,
                &dtos::CreateRailWagonMeasurementRequest::from_composite(
                  manifest.id,
                  measurement_req,
                ),
              )
              .await?;
            measurements.get_or_insert_with(Vec::new).push(measurement);
          }
        }

        if let Some(weight_reqs) = &manifest_req.weights {
          for weight_req in weight_reqs {
            let weight = self
              .rail_weight_create_no_tx(
                conn,
                &dtos::CreateRailWagonWeightRequest::from_composite(manifest.id, weight_req),
              )
              .await?;
            weights.get_or_insert_with(Vec::new).push(weight);
          }
        }

        manifest.measurements = measurements;
        manifest.weights = weights;

        wagon_manifests.get_or_insert_with(Vec::new).push(manifest);
      }
    }

    Ok(dtos::RailWaybillCompositeResponse {
      waybill,
      wagon_manifests,
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
