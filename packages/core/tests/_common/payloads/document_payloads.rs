use serde_json::json;
use uuid::Uuid;

pub fn acceptance_save_truck(
  document_number: &str,
  date_accepted: &str,
  contractor_id: Uuid,
) -> String {
  json!({
    "documentNumber": document_number,
    "dateAccepted": date_accepted,
    "arrivalType": "TRUCK",
    "contractorId": contractor_id,
  })
  .to_string()
}

pub fn dispatch_save_external_truck(
  document_number: &str,
  date: &str,
  contractor_id: Uuid,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "dispatchPurpose": "EXTERNAL",
    "dispatchMethod": "TRUCK",
    "contractorId": contractor_id,
  })
  .to_string()
}

#[allow(clippy::too_many_arguments)]
pub fn dispatch_storage_measurement(
  dispatch_doc_id: Uuid,
  storage_id: Uuid,
  before_height: &str,
  before_volume: &str,
  before_density: &str,
  before_mass: &str,
  after_height: &str,
  after_volume: &str,
  after_density: &str,
  after_mass: &str,
) -> String {
  json!({
    "dispatchDocId": dispatch_doc_id,
    "storageId": storage_id,
    "beforeHeight": before_height,
    "beforeVolume": before_volume,
    "beforeDensity": before_density,
    "beforeMass": before_mass,
    "afterHeight": after_height,
    "afterVolume": after_volume,
    "afterDensity": after_density,
    "afterMass": after_mass,
  })
  .to_string()
}

pub fn blending_save(
  document_number: &str,
  date: &str,
  contractor_id: Uuid,
  target_product_id: Uuid,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "contractorId": contractor_id,
    "targetProductId": target_product_id,
  })
  .to_string()
}

#[allow(clippy::too_many_arguments)]
pub fn operations_physical_transfer(
  document_number: &str,
  date: &str,
  start_cargo_ops: &str,
  end_cargo_ops: &str,
  contractor_id: Uuid,
  product_id: Uuid,
  from_storage_id: Uuid,
  to_storage_id: Uuid,
  amount: &str,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "contractorId": contractor_id,
    "startCargoOps": start_cargo_ops,
    "endCargoOps": end_cargo_ops,
    "items": [{
      "productId": product_id,
      "fromStorageId": from_storage_id,
      "toStorageId": to_storage_id,
      "amount": amount,
    }],
  })
  .to_string()
}

pub fn operations_ownership_transfer(
  date: &str,
  storage_id: Uuid,
  product_id: Uuid,
  from_contractor_id: Uuid,
  to_contractor_id: Uuid,
  amount: &str,
) -> String {
  json!({
    "date": date,
    "items": [{
      "storageId": storage_id,
      "productId": product_id,
      "fromContractorId": from_contractor_id,
      "toContractorId": to_contractor_id,
      "amount": amount,
    }],
  })
  .to_string()
}

pub fn operations_reconciliation_save(
  document_number: &str,
  date: &str,
  contractor_id: Uuid,
  warehouse_id: Uuid,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "contractorId": contractor_id,
    "warehouseId": warehouse_id,
  })
  .to_string()
}

pub fn operations_reconciliation_adjustment(
  reconciliation_id: Uuid,
  storage_id: Uuid,
  product_id: Uuid,
  adjustment_type: &str,
  amount: &str,
  reason: &str,
) -> String {
  json!({
    "reconciliationId": reconciliation_id,
    "storageId": storage_id,
    "productId": product_id,
    "adjustmentType": adjustment_type,
    "amount": amount,
    "reason": reason,
  })
  .to_string()
}

pub fn acceptance_composite_save(
  document_number: &str,
  date_accepted: &str,
  contractor_id: Uuid,
  product_id: Uuid,
  storage_id: Uuid,
  accepted_amount: &str,
) -> String {
  json!({
    "documentNumber": document_number,
    "dateAccepted": date_accepted,
    "arrivalType": "TRUCK",
    "contractorId": contractor_id,
    "items": [
      {
        "productId": product_id,
        "storageId": storage_id,
        "acceptedAmount": accepted_amount,
      }
    ]
  })
  .to_string()
}

pub fn acceptance_composite_save_and_execute(
  document_number: &str,
  date_accepted: &str,
  product_id: Uuid,
  contractor_id: Uuid,
  storage_id: Uuid,
  accepted_amount: &str,
) -> String {
  json!({
    "documentNumber": document_number,
    "dateAccepted": date_accepted,
    "arrivalType": "TRUCK",
    "contractorId": contractor_id,
    "items": [
      {
        "productId": product_id,
        "storageId": storage_id,
        "acceptedAmount": accepted_amount,
      }
    ]
  })
  .to_string()
}

pub fn dispatch_composite_save(
  document_number: &str,
  date: &str,
  contractor_id: Uuid,
  product_id: Uuid,
  storage_id: Uuid,
  dispatched_amount: &str,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "dispatchPurpose": "EXTERNAL",
    "dispatchMethod": "TRUCK",
    "contractorId": contractor_id,
    "items": [
      {
        "productId": product_id,
        "storageId": storage_id,
        "dispatchedAmount": dispatched_amount,
      }
    ]
  })
  .to_string()
}

pub fn dispatch_composite_save_and_execute(
  document_number: &str,
  date: &str,
  contractor_id: Uuid,
  product_id: Uuid,
  storage_id: Uuid,
  dispatched_amount: &str,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "dispatchPurpose": "EXTERNAL",
    "dispatchMethod": "TRUCK",
    "contractorId": contractor_id,
    "items": [
      {
        "productId": product_id,
        "storageId": storage_id,
        "dispatchedAmount": dispatched_amount,
      }
    ]
  })
  .to_string()
}

#[allow(clippy::too_many_arguments)]
pub fn dispatch_composite_save_and_execute_with_measurement(
  document_number: &str,
  date: &str,
  contractor_id: Uuid,
  product_id: Uuid,
  storage_id: Uuid,
  dispatched_amount: &str,
  before_mass: &str,
  after_mass: &str,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "dispatchPurpose": "EXTERNAL",
    "dispatchMethod": "TRUCK",
    "contractorId": contractor_id,
    "items": [
      {
        "productId": product_id,
        "storageId": storage_id,
        "dispatchedAmount": dispatched_amount,
      }
    ],
    "storageMeasurements": [
      {
        "storageId": storage_id,
        "beforeHeight": "0.0",
        "beforeVolume": "0.0",
        "beforeDensity": "0.0",
        "beforeMass": before_mass,
        "afterHeight": "0.0",
        "afterVolume": "0.0",
        "afterDensity": "0.0",
        "afterMass": after_mass,
      }
    ]
  })
  .to_string()
}

#[allow(clippy::too_many_arguments)]
pub fn blending_composite_save(
  document_number: &str,
  date: &str,
  contractor_id: Uuid,
  target_product_id: Uuid,
  source_storage_id: Uuid,
  source_product_id: Uuid,
  amount_used: &str,
  result_storage_id: Uuid,
  produced_amount: &str,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "contractorId": contractor_id,
    "targetProductId": target_product_id,
    "components": [
      {
        "storageId": source_storage_id,
        "sourceProductId": source_product_id,
        "amountUsed": amount_used,
      }
    ],
    "results": [
      {
        "storageId": result_storage_id,
        "producedAmount": produced_amount,
      }
    ]
  })
  .to_string()
}

#[allow(clippy::too_many_arguments)]
pub fn blending_composite_save_and_execute(
  document_number: &str,
  date: &str,
  contractor_id: Uuid,
  target_product_id: Uuid,
  source_storage_id: Uuid,
  source_product_id: Uuid,
  amount_used: &str,
  result_storage_id: Uuid,
  produced_amount: &str,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "contractorId": contractor_id,
    "targetProductId": target_product_id,
    "components": [
      {
        "storageId": source_storage_id,
        "sourceProductId": source_product_id,
        "amountUsed": amount_used,
      }
    ],
    "results": [
      {
        "storageId": result_storage_id,
        "producedAmount": produced_amount,
      }
    ]
  })
  .to_string()
}
