use axum::http::StatusCode;
use voletu_core::endpoints::paths as api_paths;

use super::seed_inventory_context;
use crate::common::{
  http::{
    assert_api_error,
    assert_api_success,
    get,
    post_json,
    setup_seeded_app_with_admin_token,
    with_auth_token,
  },
  payloads::{
    acceptance_save_truck,
    blending_save,
    dispatch_save_external_truck,
    operations_ownership_transfer,
    operations_physical_transfer,
    operations_reconciliation_save,
  },
};

const ACCEPTANCE_PAGINATION_DOC_NUMBER_1: &str = "ACC-PAG-1";
const ACCEPTANCE_PAGINATION_DOC_NUMBER_2: &str = "ACC-PAG-2";
const ACCEPTANCE_PAGINATION_DOC_NUMBER_3: &str = "ACC-PAG-3";
const DISPATCH_PAGINATION_DOC_NUMBER_1: &str = "DISP-PAG-1";
const DISPATCH_PAGINATION_DOC_NUMBER_2: &str = "DISP-PAG-2";
const DISPATCH_PAGINATION_DOC_NUMBER_3: &str = "DISP-PAG-3";
const RECONCILIATION_PAGINATION_DOC_NUMBER_1: &str = "REC-PAG-1";
const RECONCILIATION_PAGINATION_DOC_NUMBER_2: &str = "REC-PAG-2";
const RECONCILIATION_PAGINATION_DOC_NUMBER_3: &str = "REC-PAG-3";
const OWNERSHIP_PAGINATION_DATE_1: &str = "2026-01-13T09:00:00Z";
const OWNERSHIP_PAGINATION_DATE_2: &str = "2026-01-13T10:00:00Z";
const OWNERSHIP_PAGINATION_DATE_3: &str = "2026-01-13T11:00:00Z";
const PHYSICAL_PAGINATION_DOC_NUMBER_1: &str = "PT-PAG-1";
const PHYSICAL_PAGINATION_DOC_NUMBER_2: &str = "PT-PAG-2";
const PHYSICAL_PAGINATION_DOC_NUMBER_3: &str = "PT-PAG-3";
const BLENDING_PAGINATION_DOC_NUMBER_1: &str = "BLD-PAG-1";
const BLENDING_PAGINATION_DOC_NUMBER_2: &str = "BLD-PAG-2";
const BLENDING_PAGINATION_DOC_NUMBER_3: &str = "BLD-PAG-3";

#[tokio::test]
async fn acceptance_query_supports_params_and_rejects_malformed_values() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::acceptance::SAVE,
        acceptance_save_truck(
          ACCEPTANCE_PAGINATION_DOC_NUMBER_1,
          "2026-01-13T00:00:00Z",
          ctx.contractor_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::acceptance::SAVE,
        acceptance_save_truck(
          ACCEPTANCE_PAGINATION_DOC_NUMBER_2,
          "2026-01-13T01:00:00Z",
          ctx.contractor_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::acceptance::SAVE,
        acceptance_save_truck(
          ACCEPTANCE_PAGINATION_DOC_NUMBER_3,
          "2026-01-13T02:00:00Z",
          ctx.contractor_id,
        ),
      )
      .await,
    )
    .await;

    let first_page = get(
      &app,
      format!("{}?page=1&per_page=2", api_paths::acceptance::QUERY),
    )
    .await;
    let first_page_json = assert_api_success(first_page).await;
    let first_page_rows = first_page_json["data"]
      .as_array()
      .expect("acceptance pagination query data must be array");
    assert_eq!(first_page_rows.len(), 2);

    let second_page = get(
      &app,
      format!("{}?page=2&per_page=2", api_paths::acceptance::QUERY),
    )
    .await;
    let second_page_json = assert_api_success(second_page).await;
    let second_page_rows = second_page_json["data"]
      .as_array()
      .expect("acceptance pagination query data must be array");
    assert_eq!(second_page_rows.len(), 1);

    let malformed_page = get(&app, format!("{}?page=abc", api_paths::acceptance::QUERY)).await;
    let _ = assert_api_error(
      malformed_page,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      None,
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn dispatch_query_supports_params_and_rejects_malformed_values() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::dispatch::SAVE,
        dispatch_save_external_truck(
          DISPATCH_PAGINATION_DOC_NUMBER_1,
          "2026-01-13T03:00:00Z",
          ctx.contractor_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::dispatch::SAVE,
        dispatch_save_external_truck(
          DISPATCH_PAGINATION_DOC_NUMBER_2,
          "2026-01-13T04:00:00Z",
          ctx.contractor_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::dispatch::SAVE,
        dispatch_save_external_truck(
          DISPATCH_PAGINATION_DOC_NUMBER_3,
          "2026-01-13T05:00:00Z",
          ctx.contractor_id,
        ),
      )
      .await,
    )
    .await;

    let first_page = get(
      &app,
      format!("{}?page=1&per_page=2", api_paths::dispatch::QUERY),
    )
    .await;
    let first_page_json = assert_api_success(first_page).await;
    let first_page_rows = first_page_json["data"]
      .as_array()
      .expect("dispatch pagination query data must be array");
    assert_eq!(first_page_rows.len(), 2);

    let second_page = get(
      &app,
      format!("{}?page=2&per_page=2", api_paths::dispatch::QUERY),
    )
    .await;
    let second_page_json = assert_api_success(second_page).await;
    let second_page_rows = second_page_json["data"]
      .as_array()
      .expect("dispatch pagination query data must be array");
    assert_eq!(second_page_rows.len(), 1);

    let malformed_per_page = get(
      &app,
      format!("{}?per_page=oops", api_paths::dispatch::QUERY),
    )
    .await;
    let _ = assert_api_error(
      malformed_per_page,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      None,
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn reconciliation_query_supports_params_and_rejects_malformed_values() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::RECONCILIATIONS_SAVE,
        operations_reconciliation_save(
          RECONCILIATION_PAGINATION_DOC_NUMBER_1,
          "2026-01-13T06:00:00Z",
          ctx.contractor_id,
          ctx.warehouse_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::RECONCILIATIONS_SAVE,
        operations_reconciliation_save(
          RECONCILIATION_PAGINATION_DOC_NUMBER_2,
          "2026-01-13T07:00:00Z",
          ctx.contractor_id,
          ctx.warehouse_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::RECONCILIATIONS_SAVE,
        operations_reconciliation_save(
          RECONCILIATION_PAGINATION_DOC_NUMBER_3,
          "2026-01-13T08:00:00Z",
          ctx.contractor_id,
          ctx.warehouse_id,
        ),
      )
      .await,
    )
    .await;

    let first_page = get(
      &app,
      format!(
        "{}?page=1&per_page=2",
        api_paths::operations::RECONCILIATIONS_QUERY
      ),
    )
    .await;
    let first_page_json = assert_api_success(first_page).await;
    let first_page_rows = first_page_json["data"]
      .as_array()
      .expect("reconciliation pagination query data must be array");
    assert_eq!(first_page_rows.len(), 2);

    let second_page = get(
      &app,
      format!(
        "{}?page=2&per_page=2",
        api_paths::operations::RECONCILIATIONS_QUERY
      ),
    )
    .await;
    let second_page_json = assert_api_success(second_page).await;
    let second_page_rows = second_page_json["data"]
      .as_array()
      .expect("reconciliation pagination query data must be array");
    assert_eq!(second_page_rows.len(), 1);

    let malformed_page = get(
      &app,
      format!("{}?page=NaN", api_paths::operations::RECONCILIATIONS_QUERY),
    )
    .await;
    let _ = assert_api_error(
      malformed_page,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      None,
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn ownership_transfer_query_supports_params_and_rejects_malformed_values() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::OWNERSHIP_TRANSFERS_SAVE,
        operations_ownership_transfer(
          OWNERSHIP_PAGINATION_DATE_1,
          ctx.storage_a,
          ctx.product_id,
          ctx.contractor_id,
          ctx.contractor_id,
          "1.0",
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::OWNERSHIP_TRANSFERS_SAVE,
        operations_ownership_transfer(
          OWNERSHIP_PAGINATION_DATE_2,
          ctx.storage_a,
          ctx.product_id,
          ctx.contractor_id,
          ctx.contractor_id,
          "2.0",
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::OWNERSHIP_TRANSFERS_SAVE,
        operations_ownership_transfer(
          OWNERSHIP_PAGINATION_DATE_3,
          ctx.storage_a,
          ctx.product_id,
          ctx.contractor_id,
          ctx.contractor_id,
          "3.0",
        ),
      )
      .await,
    )
    .await;

    let first_page = get(
      &app,
      format!(
        "{}?page=1&per_page=2",
        api_paths::operations::OWNERSHIP_TRANSFERS_QUERY
      ),
    )
    .await;
    let first_page_json = assert_api_success(first_page).await;
    let first_page_rows = first_page_json["data"]
      .as_array()
      .expect("ownership pagination query data must be array");
    assert_eq!(first_page_rows.len(), 2);

    let second_page = get(
      &app,
      format!(
        "{}?page=2&per_page=2",
        api_paths::operations::OWNERSHIP_TRANSFERS_QUERY
      ),
    )
    .await;
    let second_page_json = assert_api_success(second_page).await;
    let second_page_rows = second_page_json["data"]
      .as_array()
      .expect("ownership pagination query data must be array");
    assert_eq!(second_page_rows.len(), 1);

    let malformed_page = get(
      &app,
      format!(
        "{}?page=oops",
        api_paths::operations::OWNERSHIP_TRANSFERS_QUERY
      ),
    )
    .await;
    let _ = assert_api_error(
      malformed_page,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      None,
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn physical_transfer_query_supports_params_and_rejects_malformed_values() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::PHYSICAL_TRANSFERS_SAVE,
        operations_physical_transfer(
          PHYSICAL_PAGINATION_DOC_NUMBER_1,
          "2026-01-13T12:00:00Z",
          "2026-01-13T12:10:00Z",
          "2026-01-13T12:20:00Z",
          ctx.contractor_id,
          ctx.product_id,
          ctx.storage_a,
          ctx.storage_b,
          "1.0",
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::PHYSICAL_TRANSFERS_SAVE,
        operations_physical_transfer(
          PHYSICAL_PAGINATION_DOC_NUMBER_2,
          "2026-01-13T13:00:00Z",
          "2026-01-13T13:10:00Z",
          "2026-01-13T13:20:00Z",
          ctx.contractor_id,
          ctx.product_id,
          ctx.storage_a,
          ctx.storage_b,
          "2.0",
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::operations::PHYSICAL_TRANSFERS_SAVE,
        operations_physical_transfer(
          PHYSICAL_PAGINATION_DOC_NUMBER_3,
          "2026-01-13T14:00:00Z",
          "2026-01-13T14:10:00Z",
          "2026-01-13T14:20:00Z",
          ctx.contractor_id,
          ctx.product_id,
          ctx.storage_a,
          ctx.storage_b,
          "3.0",
        ),
      )
      .await,
    )
    .await;

    let first_page = get(
      &app,
      format!(
        "{}?page=1&per_page=2",
        api_paths::operations::PHYSICAL_TRANSFERS_QUERY
      ),
    )
    .await;
    let first_page_json = assert_api_success(first_page).await;
    let first_page_rows = first_page_json["data"]
      .as_array()
      .expect("physical transfer pagination query data must be array");
    assert_eq!(first_page_rows.len(), 2);

    let second_page = get(
      &app,
      format!(
        "{}?page=2&per_page=2",
        api_paths::operations::PHYSICAL_TRANSFERS_QUERY
      ),
    )
    .await;
    let second_page_json = assert_api_success(second_page).await;
    let second_page_rows = second_page_json["data"]
      .as_array()
      .expect("physical transfer pagination query data must be array");
    assert_eq!(second_page_rows.len(), 1);

    let malformed_per_page = get(
      &app,
      format!(
        "{}?per_page=invalid",
        api_paths::operations::PHYSICAL_TRANSFERS_QUERY
      ),
    )
    .await;
    let _ = assert_api_error(
      malformed_per_page,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      None,
    )
    .await;
  })
  .await;
}

#[tokio::test]
async fn blending_query_supports_params_and_rejects_malformed_values() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let ctx = seed_inventory_context(&db).await;

  with_auth_token(token, async {
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::blending::SAVE,
        blending_save(
          BLENDING_PAGINATION_DOC_NUMBER_1,
          "2026-01-13T15:00:00Z",
          ctx.contractor_id,
          ctx.second_product_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::blending::SAVE,
        blending_save(
          BLENDING_PAGINATION_DOC_NUMBER_2,
          "2026-01-13T16:00:00Z",
          ctx.contractor_id,
          ctx.second_product_id,
        ),
      )
      .await,
    )
    .await;
    let _ = assert_api_success(
      post_json(
        &app,
        api_paths::blending::SAVE,
        blending_save(
          BLENDING_PAGINATION_DOC_NUMBER_3,
          "2026-01-13T17:00:00Z",
          ctx.contractor_id,
          ctx.second_product_id,
        ),
      )
      .await,
    )
    .await;

    let first_page = get(
      &app,
      format!("{}?page=1&per_page=2", api_paths::blending::QUERY),
    )
    .await;
    let first_page_json = assert_api_success(first_page).await;
    let first_page_rows = first_page_json["data"]
      .as_array()
      .expect("blending pagination query data must be array");
    assert_eq!(first_page_rows.len(), 2);

    let second_page = get(
      &app,
      format!("{}?page=2&per_page=2", api_paths::blending::QUERY),
    )
    .await;
    let second_page_json = assert_api_success(second_page).await;
    let second_page_rows = second_page_json["data"]
      .as_array()
      .expect("blending pagination query data must be array");
    assert_eq!(second_page_rows.len(), 1);

    let malformed_per_page =
      get(&app, format!("{}?per_page=bad", api_paths::blending::QUERY)).await;
    let _ = assert_api_error(
      malformed_per_page,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      None,
    )
    .await;
  })
  .await;
}
