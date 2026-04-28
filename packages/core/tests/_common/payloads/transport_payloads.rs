use serde_json::json;
use uuid::Uuid;

pub fn transport_truck_waybill(
  document_number: &str,
  date: &str,
  sender_id: Uuid,
  base_id: Uuid,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "senderId": sender_id,
    "baseId": base_id,
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
  base_id: Uuid,
  product_id: Uuid,
  declared_amount: &str,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "senderId": sender_id,
    "baseId": base_id,
    "items": [
      {
        "productId": product_id,
        "declaredAmount": declared_amount,
      }
    ]
  })
  .to_string()
}

pub fn transport_rail_waybill(
  document_number: &str,
  date: &str,
  sender_id: Uuid,
  base_id: Uuid,
) -> String {
  json!({
    "documentNumber": document_number,
    "date": date,
    "senderId": sender_id,
    "baseId": base_id,
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

#[allow(clippy::too_many_arguments)]
pub fn transport_rail_intake_with_acceptance(
  document_number: &str,
  date: &str,
  sender_id: Uuid,
  base_id: Uuid,
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
    "baseId": base_id,
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
