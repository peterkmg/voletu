use sea_orm::{
  ColumnTrait,
  Condition,
  ConnectionTrait,
  EntityTrait,
  PaginatorTrait,
  QueryFilter,
  TransactionTrait,
};
use uuid::Uuid;

use crate::{api::ApiError, dtos, entities::rail_waybill, services::DocumentService};

impl DocumentService {
  pub async fn rail_waybill_query(
    &self,
    document_number: Option<&str>,
    sender_id: Option<Uuid>,
    page: Option<u64>,
    per_page: Option<u64>,
  ) -> Result<Vec<dtos::RailWaybillResponse>, ApiError> {
    let (page, per_page) = crate::services::common::normalize_pagination(page, per_page)?;

    let mut condition = Condition::all();
    condition = condition.add(rail_waybill::Column::DeletedAt.is_null());

    if let Some(document_number) = document_number {
      condition = condition.add(rail_waybill::Column::DocumentNumber.contains(document_number));
    }

    if let Some(sender_id) = sender_id {
      condition = condition.add(rail_waybill::Column::SenderId.eq(sender_id));
    }

    let docs = rail_waybill::Entity::find()
      .filter(condition)
      .paginate(self.db.as_ref(), per_page)
      .fetch_page(page - 1)
      .await?;

    Ok(
      docs
        .into_iter()
        .map(dtos::RailWaybillResponse::from)
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
}
