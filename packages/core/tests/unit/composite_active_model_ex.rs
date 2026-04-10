use chrono::{NaiveDate, Utc};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use voletu_core::{
  dtos::{
    AcceptanceItemCompositeRequest,
    BlendingComponentCompositeRequest,
    BlendingResultCompositeRequest,
    CreateAcceptanceCompositeRequest,
    CreateAcceptanceRequest,
    CreateBlendingCompositeRequest,
    CreateDispatchCompositeRequest,
    CreateDispatchRequest,
    CreateOwnershipTransferRequest,
    CreatePhysicalTransferRequest,
    DispatchItemCompositeRequest,
    DispatchMeasurementCompositeRequest,
    OwnershipTransferItemCompositeRequest,
    PhysicalTransferItemCompositeRequest,
    RailWagonManifestCompositeRequest,
    RailWagonMeasurementCompositeRequest,
    RailWagonWeightCompositeRequest,
    RailWaybillCompositeRequest,
    TruckWaybillCompositeRequest,
    TruckWaybillItemCompositeRequest,
    TruckWeightDocCompositeRequest,
  },
  entities::{
    acceptance_document,
    blending_document,
    dispatch_document,
    ownership_transfer,
    physical_storage_transfer,
    rail_waybill,
    truck_waybill,
  },
  enums::{ArrivalType, DispatchMethod, DispatchPurpose},
};

#[test]
fn acceptance_composite_request_converts_into_active_model_ex() {
  let product_id = Uuid::now_v7();
  let storage_id = Uuid::now_v7();
  let req = CreateAcceptanceCompositeRequest {
    acceptance: CreateAcceptanceRequest {
      document_number: "ACC-001".into(),
      date_accepted: Utc::now(),
      arrival_type: ArrivalType::Truck,
      source_entity: Some("Sender".into()),
      contractor_id: Uuid::now_v7(),
      truck_waybill_id: Some(Uuid::now_v7()),
      rail_waybill_id: None,
      transit_dispatch_id: None,
    },
    items: vec![AcceptanceItemCompositeRequest {
      product_id,
      storage_id,
      accepted_amount: 123u64.into(),
    }],
  };

  let model: acceptance_document::ActiveModelEx = (&req).into();
  assert_eq!(model.document_number, Set("ACC-001".into()));
  assert_eq!(model.items.as_slice().len(), 1);
  assert!(matches!(
    &model.items.as_slice()[0].product_id,
    Set(value) if *value == product_id
  ));
  assert!(matches!(
    &model.items.as_slice()[0].storage_id,
    Set(value) if *value == storage_id
  ));
}

#[test]
fn dispatch_and_blending_requests_convert_into_nested_active_model_ex() {
  let dispatch_req = CreateDispatchCompositeRequest {
    dispatch: CreateDispatchRequest {
      document_number: "DIS-001".into(),
      date: Utc::now(),
      dispatch_purpose: DispatchPurpose::External,
      dispatch_method: DispatchMethod::Truck,
      contractor_id: Uuid::now_v7(),
      destination_base_id: None,
      receiver_entity: Some("Receiver".into()),
      start_cargo_ops: None,
      end_cargo_ops: None,
      bunker_type: None,
      exporter_id: None,
      port_id: None,
    },
    items: vec![DispatchItemCompositeRequest {
      product_id: Uuid::now_v7(),
      storage_id: Uuid::now_v7(),
      dispatched_amount: 55u64.into(),
    }],
    storage_measurements: Some(vec![DispatchMeasurementCompositeRequest {
      storage_id: Uuid::now_v7(),
      before_height: None,
      before_volume: None,
      before_density: None,
      before_mass: 10u64.into(),
      after_height: None,
      after_volume: None,
      after_density: None,
      after_mass: 5u64.into(),
    }]),
  };
  let dispatch_model: dispatch_document::ActiveModelEx = (&dispatch_req).into();
  assert_eq!(dispatch_model.items.as_slice().len(), 1);
  assert_eq!(dispatch_model.storage_measurements.as_slice().len(), 1);

  let blending_req = CreateBlendingCompositeRequest {
    document_number: "BLD-001".into(),
    date: Utc::now(),
    contractor_id: Uuid::now_v7(),
    target_product_id: Uuid::now_v7(),
    components: vec![BlendingComponentCompositeRequest {
      storage_id: Uuid::now_v7(),
      source_product_id: Uuid::now_v7(),
      amount_used: 7u64.into(),
    }],
    results: vec![BlendingResultCompositeRequest {
      storage_id: Uuid::now_v7(),
      produced_amount: 6u64.into(),
    }],
  };
  let blending_model: blending_document::ActiveModelEx = (&blending_req).into();
  assert_eq!(blending_model.components.as_slice().len(), 1);
  assert_eq!(blending_model.results.as_slice().len(), 1);
}

#[test]
fn transfer_requests_convert_into_active_model_ex_graphs() {
  let physical_req = CreatePhysicalTransferRequest {
    document_number: "PST-001".into(),
    date: Utc::now(),
    contractor_id: Uuid::now_v7(),
    start_cargo_ops: Utc::now(),
    end_cargo_ops: Utc::now(),
    items: vec![PhysicalTransferItemCompositeRequest {
      product_id: Uuid::now_v7(),
      from_storage_id: Uuid::now_v7(),
      to_storage_id: Uuid::now_v7(),
      amount: 12u64.into(),
    }],
  };
  let physical_model: physical_storage_transfer::ActiveModelEx = (&physical_req).into();
  assert_eq!(physical_model.items.as_slice().len(), 1);

  let ownership_req = CreateOwnershipTransferRequest {
    date: Utc::now(),
    items: vec![OwnershipTransferItemCompositeRequest {
      storage_id: Uuid::now_v7(),
      product_id: Uuid::now_v7(),
      from_contractor_id: Uuid::now_v7(),
      to_contractor_id: Uuid::now_v7(),
      amount: 8u64.into(),
    }],
  };
  let ownership_model: ownership_transfer::ActiveModelEx = (&ownership_req).into();
  assert_eq!(ownership_model.items.as_slice().len(), 1);
}

#[test]
fn transport_requests_convert_into_deep_active_model_ex_graphs() {
  let truck_req = TruckWaybillCompositeRequest {
    document_number: "TW-001".into(),
    date: NaiveDate::from_ymd_opt(2026, 4, 10).unwrap(),
    sender_id: Uuid::now_v7(),
    base_id: Uuid::now_v7(),
    items: Some(vec![TruckWaybillItemCompositeRequest {
      product_id: Uuid::now_v7(),
      declared_amount: 44u64.into(),
    }]),
    weight_docs: Some(vec![TruckWeightDocCompositeRequest {
      total_weight: 66u64.into(),
    }]),
  };
  let truck_model: truck_waybill::ActiveModelEx = (&truck_req).into();
  assert_eq!(truck_model.items.as_slice().len(), 1);
  assert_eq!(truck_model.weight_docs.as_slice().len(), 1);

  let rail_req = RailWaybillCompositeRequest {
    document_number: "RW-001".into(),
    date: NaiveDate::from_ymd_opt(2026, 4, 10).unwrap(),
    sender_id: Uuid::now_v7(),
    base_id: Uuid::now_v7(),
    manifests: Some(vec![RailWagonManifestCompositeRequest {
      wagon_number: "12345678".into(),
      product_id: Uuid::now_v7(),
      declared_volume: 10u64.into(),
      declared_density: 2u64.into(),
      declared_mass: 20u64.into(),
      measurements: Some(vec![RailWagonMeasurementCompositeRequest {
        wagon_number: "12345678".into(),
        measured_height: 3u64.into(),
        lab_density: Some(2u64.into()),
        calculated_mass: 6u64.into(),
      }]),
      weights: Some(vec![RailWagonWeightCompositeRequest {
        wagon_number: "12345678".into(),
        gross_weight: 100u64.into(),
        tare_weight: 40u64.into(),
        net_product_weight: 60u64.into(),
      }]),
    }]),
  };
  let rail_model: rail_waybill::ActiveModelEx = (&rail_req).into();
  assert_eq!(rail_model.wagon_manifests.as_slice().len(), 1);
  let manifest = &rail_model.wagon_manifests.as_slice()[0];
  assert_eq!(manifest.measurements.as_slice().len(), 1);
  assert_eq!(manifest.weights.as_slice().len(), 1);
}
