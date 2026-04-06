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

pub fn acceptance_item(
  acceptance_doc_id: Uuid,
  product_id: Uuid,
  storage_id: Uuid,
  accepted_amount: &str,
) -> String {
  json!({
    "acceptanceDocId": acceptance_doc_id,
    "productId": product_id,
    "storageId": storage_id,
    "acceptedAmount": accepted_amount,
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

pub fn dispatch_item(
  dispatch_doc_id: Uuid,
  product_id: Uuid,
  storage_id: Uuid,
  dispatched_amount: &str,
) -> String {
  json!({
    "dispatchDocId": dispatch_doc_id,
    "productId": product_id,
    "storageId": storage_id,
    "dispatchedAmount": dispatched_amount,
  })
  .to_string()
}

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

pub fn blending_component(
  blending_doc_id: Uuid,
  storage_id: Uuid,
  source_product_id: Uuid,
  amount_used: &str,
) -> String {
  json!({
    "blendingDocId": blending_doc_id,
    "storageId": storage_id,
    "sourceProductId": source_product_id,
    "amountUsed": amount_used,
  })
  .to_string()
}

pub fn blending_result(blending_doc_id: Uuid, storage_id: Uuid, produced_amount: &str) -> String {
  json!({
    "blendingDocId": blending_doc_id,
    "storageId": storage_id,
    "producedAmount": produced_amount,
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

pub fn catalog_company(
  common_name: &str,
  legal_name: Option<&str>,
  is_contractor: bool,
  is_exporter: bool,
  is_manufacturer: bool,
  is_sender: bool,
) -> String {
  json!({
    "commonName": common_name,
    "legalName": legal_name,
    "isContractor": is_contractor,
    "isExporter": is_exporter,
    "isManufacturer": is_manufacturer,
    "isSender": is_sender,
  })
  .to_string()
}

pub fn catalog_product_type(common_name: &str, long_name: Option<&str>) -> String {
  json!({
    "commonName": common_name,
    "longName": long_name,
  })
  .to_string()
}

pub fn catalog_product_group(
  product_type_id: Uuid,
  common_name: &str,
  long_name: Option<&str>,
) -> String {
  json!({
    "productTypeId": product_type_id,
    "commonName": common_name,
    "longName": long_name,
  })
  .to_string()
}

pub fn catalog_product(
  product_group_id: Uuid,
  manufacturer_id: Option<Uuid>,
  common_name: &str,
  long_name: Option<&str>,
  add_identification: Option<&str>,
  is_component: bool,
) -> String {
  json!({
    "productGroupId": product_group_id,
    "manufacturerId": manufacturer_id,
    "commonName": common_name,
    "longName": long_name,
    "addIdentification": add_identification,
    "isComponent": is_component,
  })
  .to_string()
}

pub fn catalog_base(common_name: &str, long_name: Option<&str>) -> String {
  json!({
    "commonName": common_name,
    "longName": long_name,
  })
  .to_string()
}

pub fn catalog_warehouse(base_id: Uuid, common_name: &str, long_name: Option<&str>) -> String {
  json!({
    "baseId": base_id,
    "commonName": common_name,
    "longName": long_name,
  })
  .to_string()
}

pub fn catalog_storage(
  warehouse_id: Uuid,
  common_name: &str,
  long_name: Option<&str>,
  capacity: Option<&str>,
  is_type_specific: bool,
  product_type_id: Option<Uuid>,
) -> String {
  json!({
    "warehouseId": warehouse_id,
    "commonName": common_name,
    "longName": long_name,
    "capacity": capacity,
    "isTypeSpecific": is_type_specific,
    "productTypeId": product_type_id,
  })
  .to_string()
}

pub fn catalog_port(common_name: &str, country: &str) -> String {
  json!({
    "commonName": common_name,
    "country": country,
  })
  .to_string()
}

pub fn auth_login(username: &str, password: &str) -> String {
  json!({
    "username": username,
    "password": password,
  })
  .to_string()
}

pub fn auth_refresh(refresh_token: &str) -> String {
  json!({
    "refreshToken": refresh_token,
  })
  .to_string()
}

pub fn auth_change_password(username: &str, current_password: &str, new_password: &str) -> String {
  json!({
    "username": username,
    "currentPassword": current_password,
    "newPassword": new_password,
  })
  .to_string()
}

pub fn user_create(username: &str, password: &str, fullname: &str, role_name: &str) -> String {
  json!({
    "username": username,
    "password": password,
    "fullname": fullname,
    "roleName": role_name,
  })
  .to_string()
}

pub fn ledger_query(storage_id: Uuid, product_id: Uuid, contractor_id: Uuid) -> String {
  json!({
    "storageId": storage_id,
    "productId": product_id,
    "contractorId": contractor_id,
  })
  .to_string()
}

pub fn sync_push_invalid_action(
  record_id: Uuid,
  target_base_id: Uuid,
  user_id: Uuid,
  origin_db_id: Uuid,
) -> String {
  json!({
    "logs": [
      {
        "id": Uuid::now_v7(),
        "tableName": "companies",
        "recordId": record_id,
        "action": "BAD_ACTION",
        "oldValuesJson": null,
        "newValuesJson": "{}",
        "targetBaseIds": target_base_id.to_string(),
        "userRoleWeight": 1,
        "userId": user_id,
        "timestamp": "2026-01-01T00:00:00Z",
        "originDbId": origin_db_id,
      }
    ]
  })
  .to_string()
}

pub fn sync_push_insert_company(
  log_id: Uuid,
  record_id: Uuid,
  target_base_id: Uuid,
  user_id: Uuid,
  origin_db_id: Uuid,
  common_name: &str,
) -> String {
  let new_values_json = json!({
    "id": record_id,
    "common_name": common_name,
    "legal_name": null,
    "is_contractor": true,
    "is_exporter": false,
    "is_manufacturer": false,
    "is_sender": false,
    "created_at": "2026-01-01T00:00:00Z",
    "updated_at": "2026-01-01T00:00:00Z",
    "deleted_at": null,
    "created_by": user_id,
    "updated_by": user_id,
    "deleted_by": null,
    "origin_db_id": origin_db_id,
  })
  .to_string();

  json!({
    "logs": [
      {
        "id": log_id,
        "tableName": "companies",
        "recordId": record_id,
        "action": "INSERT",
        "oldValuesJson": null,
        "newValuesJson": new_values_json,
        "targetBaseIds": target_base_id.to_string(),
        "userRoleWeight": 40,
        "userId": user_id,
        "timestamp": "2026-01-01T00:00:00Z",
        "originDbId": origin_db_id,
      }
    ]
  })
  .to_string()
}

pub fn sync_watermark_upsert(
  target_node_id: Uuid,
  direction: &str,
  last_audit_log_id: Uuid,
) -> String {
  json!({
    "targetNodeId": target_node_id,
    "direction": direction,
    "lastAuditLogId": last_audit_log_id,
  })
  .to_string()
}

pub fn node_initialize_replace(new_username: &str, new_password: &str, fullname: &str) -> String {
  json!({
    "action": "REPLACE",
    "newUsername": new_username,
    "newPassword": new_password,
    "fullname": fullname,
  })
  .to_string()
}

pub fn node_initialize_replace_with_node_type(
  new_username: &str,
  new_password: &str,
  fullname: &str,
  node_type: &str,
) -> String {
  json!({
    "action": "REPLACE",
    "newUsername": new_username,
    "newPassword": new_password,
    "fullname": fullname,
    "nodeType": node_type,
  })
  .to_string()
}

pub fn transport_truck_waybill(document_number: &str, date: &str, sender_id: Uuid) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "senderId": sender_id,
  })
  .to_string()
}

pub fn transport_truck_item(
  truck_waybill_id: Uuid,
  product_id: Uuid,
  declared_amount: &str,
) -> String {
  json!({
    "truckWaybillId": truck_waybill_id,
    "productId": product_id,
    "declaredAmount": declared_amount,
  })
  .to_string()
}

pub fn transport_truck_weight_doc(truck_waybill_id: Uuid, total_weight: &str) -> String {
  json!({
    "truckWaybillId": truck_waybill_id,
    "totalWeight": total_weight,
  })
  .to_string()
}

pub fn transport_truck_intake_save(
  document_number: &str,
  date: &str,
  sender_id: Uuid,
  product_id: Uuid,
  declared_amount: &str,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "senderId": sender_id,
    "items": [
      {
        "productId": product_id,
        "declaredAmount": declared_amount,
      }
    ]
  })
  .to_string()
}

pub fn transport_rail_waybill(document_number: &str, date: &str, sender_id: Uuid) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "senderId": sender_id,
  })
  .to_string()
}

pub fn transport_rail_manifest(
  rail_waybill_id: Uuid,
  wagon_number: &str,
  product_id: Uuid,
  declared_volume: &str,
  declared_density: &str,
  declared_mass: &str,
) -> String {
  json!({
    "railWaybillId": rail_waybill_id,
    "wagonNumber": wagon_number,
    "productId": product_id,
    "declaredVolume": declared_volume,
    "declaredDensity": declared_density,
    "declaredMass": declared_mass,
  })
  .to_string()
}

pub fn transport_rail_measurement(
  wagon_manifest_id: Uuid,
  measured_height: &str,
  lab_density: &str,
  calculated_mass: &str,
) -> String {
  json!({
    "wagonManifestId": wagon_manifest_id,
    "measuredHeight": measured_height,
    "labDensity": lab_density,
    "calculatedMass": calculated_mass,
  })
  .to_string()
}

pub fn transport_rail_weight(
  wagon_manifest_id: Uuid,
  gross_weight: &str,
  tare_weight: &str,
  net_product_weight: &str,
) -> String {
  json!({
    "wagonManifestId": wagon_manifest_id,
    "grossWeight": gross_weight,
    "tareWeight": tare_weight,
    "netProductWeight": net_product_weight,
  })
  .to_string()
}

pub fn transport_rail_intake_with_acceptance(
  document_number: &str,
  date: &str,
  sender_id: Uuid,
  wagon_number: &str,
  acceptance_document_number: &str,
  product_id: Uuid,
  contractor_id: Uuid,
  storage_id: Uuid,
  include_measurements_and_weights: bool,
) -> String {
  let mut payload = json!({
    "documentNumber": document_number,
    "date": date,
    "senderId": sender_id,
    "manifests": [
      {
        "wagonNumber": wagon_number,
        "productId": product_id,
        "declaredVolume": "20.0",
        "declaredDensity": "0.8",
        "declaredMass": "16.0",
      }
    ],
    "acceptance": {
      "documentNumber": acceptance_document_number,
      "dateAccepted": "2026-01-02T10:00:00Z",
      "sourceEntity": "Rail Sender",
      "items": [
        {
          "productId": product_id,
          "contractorId": contractor_id,
          "storageId": storage_id,
          "acceptedAmount": "20.0",
        }
      ]
    }
  });

  if include_measurements_and_weights {
    payload["manifests"][0]["measurements"] = json!([
      {
        "wagonNumber": wagon_number,
        "measuredHeight": "2.0",
        "labDensity": "0.79",
        "calculatedMass": "15.8",
      }
    ]);
    payload["manifests"][0]["weights"] = json!([
      {
        "wagonNumber": wagon_number,
        "grossWeight": "40.0",
        "tareWeight": "20.0",
        "netProductWeight": "20.0",
      }
    ]);
  }

  payload.to_string()
}
