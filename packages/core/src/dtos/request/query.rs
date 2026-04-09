use std::fmt;

use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use crate::{
  enums::{DispatchMethod, DispatchPurpose, DocumentStatus, PipelineStatus},
  services::document::query::{
    AcceptanceDocumentQuerySpec,
    AcceptanceFlatQuerySpec,
    BlendingDocumentQuerySpec,
    BlendingFlatQuerySpec,
    DispatchDocumentQuerySpec,
    DispatchFlatQuerySpec,
    OwnershipTransferFlatQuerySpec,
    OwnershipTransferQuerySpec,
    PhysicalTransferFlatQuerySpec,
    PhysicalTransferQuerySpec,
    RailReceiptPipelineQuerySpec,
    RailWaybillQuerySpec,
    ReconciliationFlatQuerySpec,
    ReconciliationQuerySpec,
    TruckDispatchPipelineQuerySpec,
    TruckReceiptPipelineQuerySpec,
    TruckWaybillQuerySpec,
  },
};

/// Filter for nullable FK columns: `?field=isNull` or `?field=isNotNull`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullableFilter {
  IsNull,
  IsNotNull,
}

impl<'de> Deserialize<'de> for NullableFilter {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
      "isNull" => Ok(NullableFilter::IsNull),
      "isNotNull" => Ok(NullableFilter::IsNotNull),
      other => Err(serde::de::Error::custom(format!(
        "invalid NullableFilter value '{}', expected 'isNull' or 'isNotNull'",
        other
      ))),
    }
  }
}

impl fmt::Display for NullableFilter {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      NullableFilter::IsNull => write!(f, "isNull"),
      NullableFilter::IsNotNull => write!(f, "isNotNull"),
    }
  }
}

fn deserialize_optional_u64_from_string<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
  D: Deserializer<'de>,
{
  #[derive(Deserialize)]
  #[serde(untagged)]
  enum U64OrString {
    U64(u64),
    String(String),
  }

  match Option::<U64OrString>::deserialize(deserializer)? {
    None => Ok(None),
    Some(U64OrString::U64(value)) => Ok(Some(value)),
    Some(U64OrString::String(value)) => value
      .parse::<u64>()
      .map(Some)
      .map_err(serde::de::Error::custom),
  }
}

#[derive(Debug, Default, Deserialize)]
pub struct PaginationParams {
  #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
  pub page: Option<u64>,
  #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
  pub per_page: Option<u64>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EmbedParams {
  pub embed: Option<String>,
}

impl EmbedParams {
  pub fn wants_names(&self) -> bool {
    self.embed.as_deref() == Some("names")
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptanceDocumentQueryParams {
  pub document_number: Option<String>,
  pub status: Option<DocumentStatus>,
  pub truck_waybill_id: Option<NullableFilter>,
  pub rail_waybill_id: Option<NullableFilter>,
  pub transit_dispatch_id: Option<NullableFilter>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<AcceptanceDocumentQueryParams> for AcceptanceDocumentQuerySpec {
  fn from(value: AcceptanceDocumentQueryParams) -> Self {
    Self {
      document_number: value.document_number,
      status: value.status,
      truck_waybill_id: value.truck_waybill_id,
      rail_waybill_id: value.rail_waybill_id,
      transit_dispatch_id: value.transit_dispatch_id,
      page: value.pagination.page,
      per_page: value.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DispatchDocumentQueryParams {
  pub document_number: Option<String>,
  pub status: Option<DocumentStatus>,
  pub contractor_id: Option<Uuid>,
  pub dispatch_method: Option<DispatchMethod>,
  pub dispatch_purpose: Option<DispatchPurpose>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<DispatchDocumentQueryParams> for DispatchDocumentQuerySpec {
  fn from(value: DispatchDocumentQueryParams) -> Self {
    Self {
      document_number: value.document_number,
      status: value.status,
      contractor_id: value.contractor_id,
      dispatch_method: value.dispatch_method,
      dispatch_purpose: value.dispatch_purpose,
      page: value.pagination.page,
      per_page: value.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlendingDocumentQueryParams {
  pub document_number: Option<String>,
  pub status: Option<DocumentStatus>,
  pub contractor_id: Option<Uuid>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<BlendingDocumentQueryParams> for BlendingDocumentQuerySpec {
  fn from(value: BlendingDocumentQueryParams) -> Self {
    Self {
      document_number: value.document_number,
      status: value.status,
      contractor_id: value.contractor_id,
      page: value.pagination.page,
      per_page: value.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReconciliationDocumentQueryParams {
  pub document_number: Option<String>,
  pub status: Option<DocumentStatus>,
  pub warehouse_id: Option<Uuid>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<ReconciliationDocumentQueryParams> for ReconciliationQuerySpec {
  fn from(value: ReconciliationDocumentQueryParams) -> Self {
    Self {
      document_number: value.document_number,
      status: value.status,
      warehouse_id: value.warehouse_id,
      page: value.pagination.page,
      per_page: value.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalTransferDocumentQueryParams {
  pub document_number: Option<String>,
  pub status: Option<DocumentStatus>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<PhysicalTransferDocumentQueryParams> for PhysicalTransferQuerySpec {
  fn from(value: PhysicalTransferDocumentQueryParams) -> Self {
    Self {
      document_number: value.document_number,
      status: value.status,
      page: value.pagination.page,
      per_page: value.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipTransferDocumentQueryParams {
  pub status: Option<DocumentStatus>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<OwnershipTransferDocumentQueryParams> for OwnershipTransferQuerySpec {
  fn from(value: OwnershipTransferDocumentQueryParams) -> Self {
    Self {
      status: value.status,
      page: value.pagination.page,
      per_page: value.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TruckWaybillDocumentQueryParams {
  pub document_number: Option<String>,
  pub sender_id: Option<Uuid>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<TruckWaybillDocumentQueryParams> for TruckWaybillQuerySpec {
  fn from(value: TruckWaybillDocumentQueryParams) -> Self {
    Self {
      document_number: value.document_number,
      sender_id: value.sender_id,
      page: value.pagination.page,
      per_page: value.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RailWaybillDocumentQueryParams {
  pub document_number: Option<String>,
  pub sender_id: Option<Uuid>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<RailWaybillDocumentQueryParams> for RailWaybillQuerySpec {
  fn from(value: RailWaybillDocumentQueryParams) -> Self {
    Self {
      document_number: value.document_number,
      sender_id: value.sender_id,
      page: value.pagination.page,
      per_page: value.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptanceFlatQueryParams {
  pub status: Option<DocumentStatus>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<AcceptanceFlatQueryParams> for AcceptanceFlatQuerySpec {
  fn from(params: AcceptanceFlatQueryParams) -> Self {
    Self {
      status: params.status,
      page: params.pagination.page,
      per_page: params.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DispatchFlatQueryParams {
  pub status: Option<DocumentStatus>,
  pub dispatch_method: Option<DispatchMethod>,
  pub dispatch_purpose: Option<DispatchPurpose>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<DispatchFlatQueryParams> for DispatchFlatQuerySpec {
  fn from(params: DispatchFlatQueryParams) -> Self {
    Self {
      status: params.status,
      dispatch_method: params.dispatch_method,
      dispatch_purpose: params.dispatch_purpose,
      page: params.pagination.page,
      per_page: params.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlendingFlatQueryParams {
  pub status: Option<DocumentStatus>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<BlendingFlatQueryParams> for BlendingFlatQuerySpec {
  fn from(params: BlendingFlatQueryParams) -> Self {
    Self {
      status: params.status,
      page: params.pagination.page,
      per_page: params.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReconciliationFlatQueryParams {
  pub status: Option<DocumentStatus>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<ReconciliationFlatQueryParams> for ReconciliationFlatQuerySpec {
  fn from(params: ReconciliationFlatQueryParams) -> Self {
    Self {
      status: params.status,
      page: params.pagination.page,
      per_page: params.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalTransferFlatQueryParams {
  pub status: Option<DocumentStatus>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<PhysicalTransferFlatQueryParams> for PhysicalTransferFlatQuerySpec {
  fn from(params: PhysicalTransferFlatQueryParams) -> Self {
    Self {
      status: params.status,
      page: params.pagination.page,
      per_page: params.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipTransferFlatQueryParams {
  pub status: Option<DocumentStatus>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<OwnershipTransferFlatQueryParams> for OwnershipTransferFlatQuerySpec {
  fn from(params: OwnershipTransferFlatQueryParams) -> Self {
    Self {
      status: params.status,
      page: params.pagination.page,
      per_page: params.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TruckDispatchPipelineQueryParams {
  pub pipeline_status: Option<PipelineStatus>,
  pub contractor_id: Option<Uuid>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<TruckDispatchPipelineQueryParams> for TruckDispatchPipelineQuerySpec {
  fn from(params: TruckDispatchPipelineQueryParams) -> Self {
    Self {
      pipeline_status: params.pipeline_status,
      contractor_id: params.contractor_id,
      page: params.pagination.page,
      per_page: params.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TruckReceiptPipelineQueryParams {
  pub pipeline_status: Option<PipelineStatus>,
  pub contractor_id: Option<Uuid>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<TruckReceiptPipelineQueryParams> for TruckReceiptPipelineQuerySpec {
  fn from(params: TruckReceiptPipelineQueryParams) -> Self {
    Self {
      pipeline_status: params.pipeline_status,
      contractor_id: params.contractor_id,
      page: params.pagination.page,
      per_page: params.pagination.per_page,
    }
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RailReceiptPipelineQueryParams {
  pub pipeline_status: Option<PipelineStatus>,
  pub contractor_id: Option<Uuid>,
  #[serde(flatten)]
  pub pagination: PaginationParams,
}

impl From<RailReceiptPipelineQueryParams> for RailReceiptPipelineQuerySpec {
  fn from(params: RailReceiptPipelineQueryParams) -> Self {
    Self {
      pipeline_status: params.pipeline_status,
      contractor_id: params.contractor_id,
      page: params.pagination.page,
      per_page: params.pagination.per_page,
    }
  }
}

#[cfg(test)]
mod tests {
  use uuid::Uuid;

  use super::{
    AcceptanceDocumentQueryParams,
    AcceptanceFlatQueryParams,
    DispatchDocumentQueryParams,
    DispatchFlatQueryParams,
    NullableFilter,
    OwnershipTransferDocumentQueryParams,
    OwnershipTransferFlatQueryParams,
    PaginationParams,
    PhysicalTransferDocumentQueryParams,
    PhysicalTransferFlatQueryParams,
    RailReceiptPipelineQueryParams,
    RailWaybillDocumentQueryParams,
    TruckDispatchPipelineQueryParams,
    TruckReceiptPipelineQueryParams,
    TruckWaybillDocumentQueryParams,
  };
  use crate::{
    enums::{DispatchMethod, DispatchPurpose, DocumentStatus, PipelineStatus},
    services::document::query::{
      AcceptanceDocumentQuerySpec,
      AcceptanceFlatQuerySpec,
      DispatchDocumentQuerySpec,
      DispatchFlatQuerySpec,
      OwnershipTransferFlatQuerySpec,
      OwnershipTransferQuerySpec,
      PhysicalTransferFlatQuerySpec,
      PhysicalTransferQuerySpec,
      RailReceiptPipelineQuerySpec,
      RailWaybillQuerySpec,
      TruckDispatchPipelineQuerySpec,
      TruckReceiptPipelineQuerySpec,
      TruckWaybillQuerySpec,
    },
  };

  #[test]
  fn shared_document_and_flow_query_params_convert_into_specs() {
    let acceptance_document: AcceptanceDocumentQuerySpec = AcceptanceDocumentQueryParams {
      document_number: Some("ACC-42".into()),
      status: Some(DocumentStatus::Draft),
      truck_waybill_id: Some(NullableFilter::IsNull),
      rail_waybill_id: Some(NullableFilter::IsNotNull),
      transit_dispatch_id: None,
      pagination: PaginationParams {
        page: Some(2),
        per_page: Some(25),
      },
    }
    .into();
    assert_eq!(
      acceptance_document.document_number.as_deref(),
      Some("ACC-42")
    );
    assert_eq!(acceptance_document.status, Some(DocumentStatus::Draft));
    assert_eq!(
      acceptance_document.truck_waybill_id,
      Some(NullableFilter::IsNull)
    );
    assert_eq!(
      acceptance_document.rail_waybill_id,
      Some(NullableFilter::IsNotNull)
    );
    assert_eq!(acceptance_document.page, Some(2));
    assert_eq!(acceptance_document.per_page, Some(25));

    let dispatch_document: DispatchDocumentQuerySpec = DispatchDocumentQueryParams {
      document_number: Some("DSP-7".into()),
      status: Some(DocumentStatus::Executed),
      contractor_id: Some(Uuid::nil()),
      dispatch_method: Some(DispatchMethod::Truck),
      dispatch_purpose: Some(DispatchPurpose::External),
      pagination: PaginationParams {
        page: Some(1),
        per_page: Some(10),
      },
    }
    .into();
    assert_eq!(dispatch_document.status, Some(DocumentStatus::Executed));
    assert_eq!(dispatch_document.contractor_id, Some(Uuid::nil()));
    assert_eq!(
      dispatch_document.dispatch_method,
      Some(DispatchMethod::Truck)
    );
    assert_eq!(
      dispatch_document.dispatch_purpose,
      Some(DispatchPurpose::External)
    );
    assert_eq!(dispatch_document.page, Some(1));
    assert_eq!(dispatch_document.per_page, Some(10));

    let acceptance_flat: AcceptanceFlatQuerySpec = AcceptanceFlatQueryParams {
      status: Some(DocumentStatus::Draft),
      pagination: PaginationParams {
        page: Some(3),
        per_page: Some(15),
      },
    }
    .into();
    assert_eq!(acceptance_flat.status, Some(DocumentStatus::Draft));
    assert_eq!(acceptance_flat.page, Some(3));
    assert_eq!(acceptance_flat.per_page, Some(15));

    let dispatch_flat: DispatchFlatQuerySpec = DispatchFlatQueryParams {
      status: Some(DocumentStatus::Executed),
      dispatch_method: Some(DispatchMethod::Truck),
      dispatch_purpose: Some(DispatchPurpose::Internal),
      pagination: PaginationParams {
        page: Some(4),
        per_page: Some(20),
      },
    }
    .into();
    assert_eq!(dispatch_flat.status, Some(DocumentStatus::Executed));
    assert_eq!(dispatch_flat.dispatch_method, Some(DispatchMethod::Truck));
    assert_eq!(
      dispatch_flat.dispatch_purpose,
      Some(DispatchPurpose::Internal)
    );
    assert_eq!(dispatch_flat.page, Some(4));
    assert_eq!(dispatch_flat.per_page, Some(20));

    let physical_document: PhysicalTransferQuerySpec = PhysicalTransferDocumentQueryParams {
      document_number: Some("PHY-9".into()),
      status: Some(DocumentStatus::Draft),
      pagination: PaginationParams {
        page: Some(5),
        per_page: Some(30),
      },
    }
    .into();
    assert_eq!(physical_document.document_number.as_deref(), Some("PHY-9"));
    assert_eq!(physical_document.status, Some(DocumentStatus::Draft));
    assert_eq!(physical_document.page, Some(5));
    assert_eq!(physical_document.per_page, Some(30));

    let ownership_document: OwnershipTransferQuerySpec = OwnershipTransferDocumentQueryParams {
      status: Some(DocumentStatus::Executed),
      pagination: PaginationParams {
        page: Some(6),
        per_page: Some(12),
      },
    }
    .into();
    assert_eq!(ownership_document.status, Some(DocumentStatus::Executed));
    assert_eq!(ownership_document.page, Some(6));
    assert_eq!(ownership_document.per_page, Some(12));

    let truck_waybill: TruckWaybillQuerySpec = TruckWaybillDocumentQueryParams {
      document_number: Some("TW-1".into()),
      sender_id: Some(Uuid::nil()),
      pagination: PaginationParams {
        page: Some(7),
        per_page: Some(18),
      },
    }
    .into();
    assert_eq!(truck_waybill.document_number.as_deref(), Some("TW-1"));
    assert_eq!(truck_waybill.sender_id, Some(Uuid::nil()));
    assert_eq!(truck_waybill.page, Some(7));
    assert_eq!(truck_waybill.per_page, Some(18));

    let rail_waybill: RailWaybillQuerySpec = RailWaybillDocumentQueryParams {
      document_number: Some("RW-2".into()),
      sender_id: Some(Uuid::nil()),
      pagination: PaginationParams {
        page: Some(8),
        per_page: Some(22),
      },
    }
    .into();
    assert_eq!(rail_waybill.document_number.as_deref(), Some("RW-2"));
    assert_eq!(rail_waybill.sender_id, Some(Uuid::nil()));
    assert_eq!(rail_waybill.page, Some(8));
    assert_eq!(rail_waybill.per_page, Some(22));

    let physical_flat: PhysicalTransferFlatQuerySpec = PhysicalTransferFlatQueryParams {
      status: Some(DocumentStatus::Draft),
      pagination: PaginationParams {
        page: Some(9),
        per_page: Some(16),
      },
    }
    .into();
    assert_eq!(physical_flat.status, Some(DocumentStatus::Draft));
    assert_eq!(physical_flat.page, Some(9));
    assert_eq!(physical_flat.per_page, Some(16));

    let ownership_flat: OwnershipTransferFlatQuerySpec = OwnershipTransferFlatQueryParams {
      status: Some(DocumentStatus::Executed),
      pagination: PaginationParams {
        page: Some(10),
        per_page: Some(14),
      },
    }
    .into();
    assert_eq!(ownership_flat.status, Some(DocumentStatus::Executed));
    assert_eq!(ownership_flat.page, Some(10));
    assert_eq!(ownership_flat.per_page, Some(14));

    let truck_dispatch_pipeline: TruckDispatchPipelineQuerySpec =
      TruckDispatchPipelineQueryParams {
        pipeline_status: Some(PipelineStatus::Draft),
        contractor_id: Some(Uuid::nil()),
        pagination: PaginationParams {
          page: Some(11),
          per_page: Some(13),
        },
      }
      .into();
    assert_eq!(
      truck_dispatch_pipeline.pipeline_status,
      Some(PipelineStatus::Draft)
    );
    assert_eq!(truck_dispatch_pipeline.contractor_id, Some(Uuid::nil()));
    assert_eq!(truck_dispatch_pipeline.page, Some(11));
    assert_eq!(truck_dispatch_pipeline.per_page, Some(13));

    let truck_receipt_pipeline: TruckReceiptPipelineQuerySpec = TruckReceiptPipelineQueryParams {
      pipeline_status: Some(PipelineStatus::Executed),
      contractor_id: Some(Uuid::nil()),
      pagination: PaginationParams {
        page: Some(12),
        per_page: Some(11),
      },
    }
    .into();
    assert_eq!(
      truck_receipt_pipeline.pipeline_status,
      Some(PipelineStatus::Executed)
    );
    assert_eq!(truck_receipt_pipeline.contractor_id, Some(Uuid::nil()));
    assert_eq!(truck_receipt_pipeline.page, Some(12));
    assert_eq!(truck_receipt_pipeline.per_page, Some(11));

    let rail_receipt_pipeline: RailReceiptPipelineQuerySpec = RailReceiptPipelineQueryParams {
      pipeline_status: Some(PipelineStatus::Pending),
      contractor_id: Some(Uuid::nil()),
      pagination: PaginationParams {
        page: Some(13),
        per_page: Some(9),
      },
    }
    .into();
    assert_eq!(
      rail_receipt_pipeline.pipeline_status,
      Some(PipelineStatus::Pending)
    );
    assert_eq!(rail_receipt_pipeline.contractor_id, Some(Uuid::nil()));
    assert_eq!(rail_receipt_pipeline.page, Some(13));
    assert_eq!(rail_receipt_pipeline.per_page, Some(9));
  }
}
