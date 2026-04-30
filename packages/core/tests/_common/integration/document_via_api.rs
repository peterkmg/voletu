use reqwest::{Client, StatusCode};
use serde_json::{json, Value};
use uuid::Uuid;

use super::api_post;

#[allow(clippy::too_many_arguments)]
pub async fn create_acceptance_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_number: &str,
  contractor_id: Uuid,
  product_id: Uuid,
  storage_id: Uuid,
  amount: &str,
) -> Value {
  api_post(
    client,
    &format!("{base_url}/acceptance/composite/save"),
    token,
    json!({
      "documentNumber": doc_number,
      "dateAccepted": "2026-01-15T10:00:00Z",
      "arrivalType": "TRUCK",
      "sourceEntity": null,
      "contractorId": contractor_id,
      "truckWaybillId": null,
      "railWaybillId": null,
      "transitDispatchId": null,
      "items": [{
        "productId": product_id,
        "storageId": storage_id,
        "acceptedAmount": amount,
      }]
    }),
  )
  .await
}

#[allow(clippy::too_many_arguments)]
pub async fn create_physical_transfer_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_number: &str,
  contractor_id: Uuid,
  product_id: Uuid,
  from_storage_id: Uuid,
  to_storage_id: Uuid,
  amount: &str,
) -> Value {
  api_post(
    client,
    &format!("{base_url}/physical-transfers/save"),
    token,
    json!({
      "documentNumber": doc_number,
      "date": "2026-01-15T10:00:00Z",
      "contractorId": contractor_id,
      "startCargoOps": "2026-01-15T08:00:00Z",
      "endCargoOps": "2026-01-15T16:00:00Z",
      "items": [{
        "productId": product_id,
        "fromStorageId": from_storage_id,
        "toStorageId": to_storage_id,
        "amount": amount,
      }]
    }),
  )
  .await
}

#[allow(clippy::too_many_arguments)]
pub async fn create_dispatch_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_number: &str,
  contractor_id: Uuid,
  product_id: Uuid,
  storage_id: Uuid,
  amount: &str,
  destination_base_id: Option<Uuid>,
) -> Value {
  api_post(
    client,
    &format!("{base_url}/dispatch/composite/save"),
    token,
    json!({
      "documentNumber": doc_number,
      "date": "2026-01-15T10:00:00Z",
      "dispatchPurpose": "EXTERNAL",
      "dispatchMethod": "TRUCK",
      "contractorId": contractor_id,
      "destinationBaseId": destination_base_id,
      "receiverEntity": null,
      "startCargoOps": null,
      "endCargoOps": null,
      "bunkerType": null,
      "exporterId": null,
      "portId": null,
      "items": [{
        "productId": product_id,
        "storageId": storage_id,
        "dispatchedAmount": amount,
      }],
      "storageMeasurements": null,
    }),
  )
  .await
}

#[allow(clippy::too_many_arguments)]
pub async fn create_blending_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_number: &str,
  contractor_id: Uuid,
  target_product_id: Uuid,
  component_storage_id: Uuid,
  source_product_id: Uuid,
  component_amount: &str,
  result_storage_id: Uuid,
  result_amount: &str,
) -> Value {
  api_post(
    client,
    &format!("{base_url}/blending/composite/save"),
    token,
    json!({
      "documentNumber": doc_number,
      "date": "2026-01-15T10:00:00Z",
      "contractorId": contractor_id,
      "targetProductId": target_product_id,
      "components": [{
        "storageId": component_storage_id,
        "sourceProductId": source_product_id,
        "amountUsed": component_amount,
      }],
      "results": [{
        "storageId": result_storage_id,
        "producedAmount": result_amount,
      }]
    }),
  )
  .await
}

#[allow(clippy::too_many_arguments)]
pub async fn create_ownership_transfer_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  storage_id: Uuid,
  product_id: Uuid,
  from_contractor_id: Uuid,
  to_contractor_id: Uuid,
  amount: &str,
) -> Value {
  api_post(
    client,
    &format!("{base_url}/ownership-transfers/save"),
    token,
    json!({
      "date": "2026-01-15T10:00:00Z",
      "items": [{
        "storageId": storage_id,
        "productId": product_id,
        "fromContractorId": from_contractor_id,
        "toContractorId": to_contractor_id,
        "amount": amount,
      }]
    }),
  )
  .await
}

pub async fn create_reconciliation_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  doc_number: &str,
  contractor_id: Uuid,
  warehouse_id: Uuid,
) -> Value {
  api_post(
    client,
    &format!("{base_url}/reconciliations/save"),
    token,
    json!({
      "documentNumber": doc_number,
      "date": "2026-01-15T10:00:00Z",
      "contractorId": contractor_id,
      "warehouseId": warehouse_id,
    }),
  )
  .await
}

pub async fn execute_document_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  execute_path: &str,
  doc_id: Uuid,
) -> Value {
  let url = format!(
    "{base_url}{}",
    execute_path.replace("{id}", &doc_id.to_string())
  );
  let response = client
    .post(&url)
    .bearer_auth(token)
    .header("idempotency-key", Uuid::now_v7().to_string())
    .send()
    .await
    .unwrap();
  let status = response.status();
  let body: Value = response.json().await.unwrap();
  assert_eq!(
    status,
    StatusCode::OK,
    "POST {url} returned {status}; body: {body}"
  );
  assert_eq!(body["success"], Value::Bool(true));
  body["data"].clone()
}

pub async fn revert_document_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  revert_path: &str,
  doc_id: Uuid,
) {
  execute_document_via_api(client, base_url, token, revert_path, doc_id).await;
}

pub async fn soft_delete_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  path: &str,
  id: Uuid,
) {
  let url = format!("{base_url}{}", path.replace("{id}", &id.to_string()));
  let response = client
    .delete(&url)
    .bearer_auth(token)
    .header("idempotency-key", Uuid::now_v7().to_string())
    .send()
    .await
    .unwrap();
  let status = response.status();
  let body: Value = response.json().await.unwrap();
  assert_eq!(
    status,
    StatusCode::OK,
    "DELETE {url} returned {status}; body: {body}"
  );
}

pub async fn hard_delete_via_api(
  client: &Client,
  base_url: &str,
  token: &str,
  path: &str,
  id: Uuid,
) {
  let url = format!("{base_url}{}", path.replace("{id}", &id.to_string()));
  let response = client
    .delete(&url)
    .bearer_auth(token)
    .header("idempotency-key", Uuid::now_v7().to_string())
    .send()
    .await
    .unwrap();
  let status = response.status();
  let body: Value = response.json().await.unwrap();
  assert_eq!(
    status,
    StatusCode::OK,
    "DELETE {url} returned {status}; body: {body}"
  );
}

pub async fn dev_seed_via_api(client: &Client, base_url: &str, token: &str) -> Value {
  api_post(client, &format!("{base_url}/dev/seed"), token, json!({})).await
}
