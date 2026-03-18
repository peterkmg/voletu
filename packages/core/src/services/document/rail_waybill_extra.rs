use sea_orm::{ConnectionTrait, TransactionTrait};

use crate::{api::ApiError, dtos, services::DocumentService};

impl DocumentService {
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
      waybill: waybill.into(),
      wagon_manifests,
    })
  }
}
