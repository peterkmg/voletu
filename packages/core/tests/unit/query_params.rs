use uuid::Uuid;
use voletu_core::{
  dtos::{
    request::query::NullableFilter,
    AcceptanceDocumentQueryParams,
    AcceptanceFlatQueryParams,
    CargoFlowQueryParams,
    DispatchDocumentQueryParams,
    DispatchFlatQueryParams,
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
  },
  enums::{DispatchMethod, DispatchPurpose, DocumentStatus, PipelineStatus},
  services::document::specs::{
    AcceptanceDocumentQuerySpec,
    AcceptanceFlatQuerySpec,
    CargoFlowQuerySpec,
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
fn shared_document_and_flow_convert_into_specs() {
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

  let truck_dispatch_pipeline: TruckDispatchPipelineQuerySpec = TruckDispatchPipelineQueryParams {
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

  let cargo_flow: CargoFlowQuerySpec = CargoFlowQueryParams {
    page: Some(14),
    per_page: Some(8),
    filter: Some("truck".into()),
  }
  .into();
  assert_eq!(cargo_flow.page, Some(14));
  assert_eq!(cargo_flow.per_page, Some(8));
  assert_eq!(cargo_flow.filter.as_deref(), Some("truck"));
}
