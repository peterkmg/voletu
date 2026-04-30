use sea_orm::{ColumnTrait, EntityLoaderTrait, QueryFilter};
use uuid::Uuid;

use crate::{
  api::ApiError,
  dtos,
  entities::{company, rail_waybill, truck_waybill},
  services::{
    document::specs::{
      AcceptanceDocumentQuerySpec,
      BlendingDocumentQuerySpec,
      DispatchDocumentQuerySpec,
      OwnershipTransferQuerySpec,
      PhysicalTransferQuerySpec,
      RailWaybillQuerySpec,
      ReconciliationQuerySpec,
      TruckWaybillQuerySpec,
    },
    DocumentService,
  },
};

impl DocumentService {
  pub async fn dispatch_document_list_with_names(
    &self,
  ) -> Result<Vec<dtos::DispatchResponse>, ApiError> {
    self
      .dispatch_document_query_with_names(DispatchDocumentQuerySpec::default())
      .await
  }

  pub async fn dispatch_document_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::DispatchResponse, ApiError> {
    let item = self.dispatch_document_model(id).await?;
    let exporter_names = self.company_name_map(item.exporter_id).await?;
    let exporter_id_name = item
      .exporter_id
      .and_then(|exporter_id| exporter_names.get(&exporter_id).cloned());

    Ok(dtos::DispatchResponse::from_loaded(item, exporter_id_name))
  }

  pub async fn dispatch_document_query_with_names(
    &self,
    query: DispatchDocumentQuerySpec,
  ) -> Result<Vec<dtos::DispatchResponse>, ApiError> {
    let items = self.dispatch_document_query_models(&query).await?;

    let exporter_names = self
      .company_name_map(items.iter().filter_map(|item| item.exporter_id))
      .await?;

    Ok(
      items
        .into_iter()
        .map(|item| {
          let exporter_id_name = item
            .exporter_id
            .and_then(|exporter_id| exporter_names.get(&exporter_id).cloned());
          dtos::DispatchResponse::from_loaded(item, exporter_id_name)
        })
        .collect(),
    )
  }

  pub async fn dispatch_composite_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::DispatchCompositeResponse, ApiError> {
    let composite = self.dispatch_composite_model(id).await?;

    let items = composite
      .items
      .iter()
      .cloned()
      .map(dtos::DispatchItemResponse::from)
      .collect();

    let storage_measurements = composite
      .storage_measurements
      .iter()
      .cloned()
      .map(dtos::DispatchMeasurementResponse::from)
      .collect();

    let exporter_names = self.company_name_map(composite.exporter_id).await?;

    let exporter_id_name = composite
      .exporter_id
      .and_then(|exporter_id| exporter_names.get(&exporter_id).cloned());

    Ok(dtos::DispatchCompositeResponse {
      document: dtos::DispatchResponse::from_loaded(composite, exporter_id_name),
      items,
      storage_measurements,
    })
  }

  // ── Acceptance ────────────────────────────

  pub async fn acceptance_document_list_with_names(
    &self,
  ) -> Result<Vec<dtos::AcceptanceResponse>, ApiError> {
    self
      .acceptance_document_query_with_names(AcceptanceDocumentQuerySpec::default())
      .await
  }

  pub async fn acceptance_document_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::AcceptanceResponse, ApiError> {
    Ok(self.acceptance_document_model(id).await?.into())
  }

  pub async fn acceptance_document_query_with_names(
    &self,
    query: AcceptanceDocumentQuerySpec,
  ) -> Result<Vec<dtos::AcceptanceResponse>, ApiError> {
    Ok(
      self
        .acceptance_document_query_models(&query)
        .await?
        .into_iter()
        .map(Into::into)
        .collect(),
    )
  }

  pub async fn acceptance_composite_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::AcceptanceCompositeResponse, ApiError> {
    let composite = self.acceptance_composite_model(id).await?;

    let items = composite
      .items
      .iter()
      .cloned()
      .map(dtos::AcceptanceItemResponse::from)
      .collect();

    Ok(dtos::AcceptanceCompositeResponse {
      document: composite.into(),
      items,
    })
  }

  // ── Blending ──────────────────────────────

  pub async fn blending_document_list_with_names(
    &self,
  ) -> Result<Vec<dtos::BlendingResponse>, ApiError> {
    self
      .blending_document_query_with_names(BlendingDocumentQuerySpec::default())
      .await
  }

  pub async fn blending_document_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::BlendingResponse, ApiError> {
    Ok(self.blending_document_model(id).await?.into())
  }

  pub async fn blending_document_query_with_names(
    &self,
    query: BlendingDocumentQuerySpec,
  ) -> Result<Vec<dtos::BlendingResponse>, ApiError> {
    Ok(
      self
        .blending_document_query_models(&query)
        .await?
        .into_iter()
        .map(Into::into)
        .collect(),
    )
  }

  pub async fn blending_composite_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::BlendingCompositeResponse, ApiError> {
    self.blending_composite_model(id).await?.try_into()
  }

  pub async fn reconciliation_list_with_names(
    &self,
  ) -> Result<Vec<dtos::InventoryReconciliationResponse>, ApiError> {
    self
      .reconciliation_query_with_names(ReconciliationQuerySpec::default())
      .await
  }

  pub async fn reconciliation_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::InventoryReconciliationResponse, ApiError> {
    Ok(self.reconciliation_model(id).await?.into())
  }

  pub async fn reconciliation_query_with_names(
    &self,
    query: ReconciliationQuerySpec,
  ) -> Result<Vec<dtos::InventoryReconciliationResponse>, ApiError> {
    Ok(
      self
        .reconciliation_query_models(&query)
        .await?
        .into_iter()
        .map(Into::into)
        .collect(),
    )
  }

  pub async fn truck_waybill_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::TruckWaybillResponse, ApiError> {
    let item = truck_waybill::Entity::load()
      .filter_by_id(id)
      .filter(truck_waybill::Column::DeletedAt.is_null())
      .with(company::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Truck waybill '{}' not found", id)))?;

    Ok(item.into())
  }

  pub async fn truck_waybill_query_with_names(
    &self,
    query: TruckWaybillQuerySpec,
  ) -> Result<Vec<dtos::TruckWaybillResponse>, ApiError> {
    Ok(
      self
        .truck_waybill_query_models(&query)
        .await?
        .into_iter()
        .map(Into::into)
        .collect(),
    )
  }

  pub async fn truck_waybill_composite_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::TruckWaybillCompositeResponse, ApiError> {
    let composite = self.truck_waybill_composite_model(id).await?;

    let items = (!composite.items.is_empty()).then(|| {
      composite
        .items
        .iter()
        .cloned()
        .map(dtos::TruckWaybillItemResponse::from)
        .collect()
    });

    let weight_docs = (!composite.weight_docs.is_empty()).then(|| {
      composite
        .weight_docs
        .iter()
        .cloned()
        .map(dtos::TruckWeightDocResponse::from)
        .collect()
    });

    Ok(dtos::TruckWaybillCompositeResponse {
      waybill: composite.into(),
      items,
      weight_docs,
    })
  }

  pub async fn rail_waybill_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::RailWaybillResponse, ApiError> {
    let item = rail_waybill::Entity::load()
      .filter_by_id(id)
      .filter(rail_waybill::Column::DeletedAt.is_null())
      .with(company::Entity)
      .one(self.db.as_ref())
      .await?
      .ok_or_else(|| ApiError::NotFound(format!("Rail waybill '{}' not found", id)))?;

    Ok(item.into())
  }

  pub async fn rail_waybill_query_with_names(
    &self,
    query: RailWaybillQuerySpec,
  ) -> Result<Vec<dtos::RailWaybillResponse>, ApiError> {
    Ok(
      self
        .rail_waybill_query_models(&query)
        .await?
        .into_iter()
        .map(Into::into)
        .collect(),
    )
  }

  pub async fn rail_waybill_composite_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::RailWaybillCompositeResponse, ApiError> {
    let composite = self.rail_waybill_composite_model(id).await?;

    let wagon_manifests = (!composite.wagon_manifests.is_empty()).then(|| {
      composite
        .wagon_manifests
        .iter()
        .cloned()
        .map(dtos::RailWagonManifestResponse::from)
        .collect()
    });

    Ok(dtos::RailWaybillCompositeResponse {
      waybill: composite.into(),
      wagon_manifests,
    })
  }

  pub async fn physical_transfer_list_with_names(
    &self,
  ) -> Result<Vec<dtos::PhysicalTransferResponse>, ApiError> {
    self
      .physical_transfer_composite_query_with_names(PhysicalTransferQuerySpec::default())
      .await
  }

  pub async fn physical_transfer_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::PhysicalTransferResponse, ApiError> {
    let response = self.physical_transfer_model(id).await?;

    let to_storage_ids = response
      .items
      .iter()
      .map(|item| item.to_storage_id)
      .collect::<Vec<_>>();

    let to_storage_names = self.storage_name_map(to_storage_ids).await?;

    Ok(dtos::PhysicalTransferResponse::from_loaded_with_names(
      response,
      &to_storage_names,
    ))
  }

  pub async fn physical_transfer_composite_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::PhysicalTransferResponse, ApiError> {
    let response = self.physical_transfer_model(id).await?;

    let to_storage_ids = response
      .items
      .iter()
      .map(|item| item.to_storage_id)
      .collect::<Vec<_>>();

    let to_storage_names = self.storage_name_map(to_storage_ids).await?;

    Ok(dtos::PhysicalTransferResponse::from_loaded_with_names(
      response,
      &to_storage_names,
    ))
  }

  pub async fn physical_transfer_composite_list_with_names(
    &self,
  ) -> Result<Vec<dtos::PhysicalTransferResponse>, ApiError> {
    self
      .physical_transfer_composite_query_with_names(PhysicalTransferQuerySpec::default())
      .await
  }

  pub async fn physical_transfer_composite_query_with_names(
    &self,
    query: PhysicalTransferQuerySpec,
  ) -> Result<Vec<dtos::PhysicalTransferResponse>, ApiError> {
    let responses = self.physical_transfer_query_models(&query).await?;

    let to_storage_ids = responses
      .iter()
      .flat_map(|response| response.items.iter().map(|item| item.to_storage_id))
      .collect::<Vec<_>>();

    let to_storage_names = self.storage_name_map(to_storage_ids).await?;

    Ok(
      responses
        .into_iter()
        .map(|response| {
          dtos::PhysicalTransferResponse::from_loaded_with_names(response, &to_storage_names)
        })
        .collect(),
    )
  }

  pub async fn ownership_transfer_list_with_names(
    &self,
  ) -> Result<Vec<dtos::OwnershipTransferResponse>, ApiError> {
    self
      .ownership_transfer_composite_query_with_names(OwnershipTransferQuerySpec::default())
      .await
  }

  pub async fn ownership_transfer_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::OwnershipTransferResponse, ApiError> {
    let response = self.ownership_transfer_model(id).await?;

    let contractor_ids = response
      .items
      .iter()
      .flat_map(|item| [item.from_contractor_id, item.to_contractor_id])
      .collect::<Vec<_>>();

    let contractor_names = self.company_name_map(contractor_ids).await?;

    Ok(dtos::OwnershipTransferResponse::from_loaded_with_names(
      response,
      &contractor_names,
    ))
  }

  pub async fn ownership_transfer_composite_get_with_names(
    &self,
    id: Uuid,
  ) -> Result<dtos::OwnershipTransferResponse, ApiError> {
    let response = self.ownership_transfer_model(id).await?;

    let contractor_ids = response
      .items
      .iter()
      .flat_map(|item| [item.from_contractor_id, item.to_contractor_id])
      .collect::<Vec<_>>();

    let contractor_names = self.company_name_map(contractor_ids).await?;

    Ok(dtos::OwnershipTransferResponse::from_loaded_with_names(
      response,
      &contractor_names,
    ))
  }

  pub async fn ownership_transfer_composite_list_with_names(
    &self,
  ) -> Result<Vec<dtos::OwnershipTransferResponse>, ApiError> {
    self
      .ownership_transfer_composite_query_with_names(OwnershipTransferQuerySpec::default())
      .await
  }

  pub async fn ownership_transfer_composite_query_with_names(
    &self,
    query: OwnershipTransferQuerySpec,
  ) -> Result<Vec<dtos::OwnershipTransferResponse>, ApiError> {
    let responses = self.ownership_transfer_query_models(&query).await?;

    let contractor_ids = responses
      .iter()
      .flat_map(|response| {
        response
          .items
          .iter()
          .flat_map(|item| [item.from_contractor_id, item.to_contractor_id])
      })
      .collect::<Vec<_>>();

    let contractor_names = self.company_name_map(contractor_ids).await?;

    Ok(
      responses
        .into_iter()
        .map(|response| {
          dtos::OwnershipTransferResponse::from_loaded_with_names(response, &contractor_names)
        })
        .collect(),
    )
  }
}
