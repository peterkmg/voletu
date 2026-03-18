use axum::http::StatusCode;
use sea_orm::{prelude::Decimal, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;
use voletu_core::{
  endpoints::paths as api_paths,
  entities::{role, user},
  enums,
};

use crate::common::{
  fixtures::{seed_inventory_fixture, seed_ledger_balance},
  http::{
    assert_api_error,
    assert_api_success,
    delete,
    get,
    post_json,
    setup_seeded_app_with_admin_token,
    with_auth_token,
  },
  payloads::{
    auth_change_password,
    auth_login,
    auth_refresh,
    ledger_query,
    sync_push_invalid_action,
    user_create,
  },
};

const OPERATOR_USERNAME: &str = "endpoint-operator";
const OPERATOR_PASSWORD: &str = "operator-pass";
const OPERATOR_NEW_PASSWORD: &str = "operator-pass-new";

#[tokio::test]
async fn auth_and_user_endpoints_cover_login_create_password_change_delete_and_error_payloads() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let login_admin = post_json(&app, api_paths::auth::LOGIN, auth_login("admin", "admin")).await;
    let login_admin_json = assert_api_success(login_admin).await;
    assert!(login_admin_json["data"]["accessToken"]
      .as_str()
      .is_some_and(|token| !token.is_empty()));
    let admin_refresh_token = login_admin_json["data"]["refreshToken"]
      .as_str()
      .unwrap()
      .to_string();
    assert!(!admin_refresh_token.is_empty());
    assert_eq!(login_admin_json["data"]["user"]["username"], "admin");

    let refresh_once = post_json(
      &app,
      api_paths::auth::REFRESH,
      auth_refresh(&admin_refresh_token),
    )
    .await;
    let refresh_once_json = assert_api_success(refresh_once).await;
    let rotated_refresh_token = refresh_once_json["data"]["refreshToken"]
      .as_str()
      .unwrap()
      .to_string();
    assert!(refresh_once_json["data"]["accessToken"]
      .as_str()
      .is_some_and(|token| !token.is_empty()));
    assert_ne!(rotated_refresh_token, admin_refresh_token);

    let refresh_reuse = post_json(
      &app,
      api_paths::auth::REFRESH,
      auth_refresh(&admin_refresh_token),
    )
    .await;
    let refresh_reuse_json = assert_api_error(
      refresh_reuse,
      StatusCode::UNAUTHORIZED,
      "UNAUTHORIZED",
      Some("Refresh token"),
    )
    .await;
    assert_eq!(refresh_reuse_json["error"]["code"], "UNAUTHORIZED");

    let create_body = user_create(
      OPERATOR_USERNAME,
      OPERATOR_PASSWORD,
      "Endpoint Operator",
      "OPERATOR",
    );
    let create_user = post_json(&app, api_paths::users::ROOT, create_body.clone()).await;
    let create_user_json = assert_api_success(create_user).await;
    assert_eq!(create_user_json["data"]["username"], OPERATOR_USERNAME);
    assert_eq!(create_user_json["data"]["role"], "OPERATOR");

    let duplicate_user = post_json(&app, api_paths::users::ROOT, create_body).await;
    let duplicate_json = assert_api_error(
      duplicate_user,
      StatusCode::CONFLICT,
      "CONFLICT",
      Some("already taken"),
    )
    .await;
    assert_eq!(duplicate_json["error"]["code"], "CONFLICT");

    let change_password = post_json(
      &app,
      api_paths::auth::CHANGE_PASSWORD,
      auth_change_password(OPERATOR_USERNAME, OPERATOR_PASSWORD, OPERATOR_NEW_PASSWORD),
    )
    .await;
    let change_password_json = assert_api_success(change_password).await;
    assert_eq!(change_password_json["data"]["message"], "Password changed");

    let old_login = post_json(
      &app,
      api_paths::auth::LOGIN,
      auth_login(OPERATOR_USERNAME, OPERATOR_PASSWORD),
    )
    .await;
    let old_login_json = assert_api_error(
      old_login,
      StatusCode::UNAUTHORIZED,
      "UNAUTHORIZED",
      Some("Invalid credentials"),
    )
    .await;
    assert_eq!(old_login_json["error"]["code"], "UNAUTHORIZED");

    let new_login = post_json(
      &app,
      api_paths::auth::LOGIN,
      auth_login(OPERATOR_USERNAME, OPERATOR_NEW_PASSWORD),
    )
    .await;
    let new_login_json = assert_api_success(new_login).await;
    assert_eq!(
      new_login_json["data"]["user"]["username"],
      OPERATOR_USERNAME
    );

    let endpoint_user_id = user::Entity::find()
      .filter(user::Column::Username.eq(OPERATOR_USERNAME))
      .one(&*db)
      .await
      .unwrap()
      .unwrap()
      .id;

    let delete_user = delete(
      &app,
      api_paths::users::BY_ID.replace("{id}", &endpoint_user_id.to_string()),
    )
    .await;
    let delete_user_json = assert_api_success(delete_user).await;
    assert!(delete_user_json["data"].is_null());

    let delete_again = delete(
      &app,
      api_paths::users::BY_ID.replace("{id}", &endpoint_user_id.to_string()),
    )
    .await;
    let delete_again_json = assert_api_error(
      delete_again,
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
      Some("not found"),
    )
    .await;
    assert_eq!(delete_again_json["error"]["code"], "NOT_FOUND");
  })
  .await;
}

#[tokio::test]
async fn ledger_endpoints_lookup_entries_by_dimensions_and_return_matching_payloads() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;
  let fixture = seed_inventory_fixture(&db).await;
  seed_ledger_balance(
    &db,
    fixture.storage_a_id,
    fixture.product_a_id,
    fixture.contractor_a_id,
    Decimal::from(11),
  )
  .await;
  with_auth_token(token, async {
    let lookup = post_json(
      &app,
      api_paths::ledger::QUERY,
      ledger_query(
        fixture.storage_a_id,
        fixture.product_a_id,
        fixture.contractor_a_id,
      ),
    )
    .await;
    let lookup_json = assert_api_success(lookup).await;
    assert_eq!(
      lookup_json["data"]["storageId"],
      fixture.storage_a_id.to_string()
    );
    assert_eq!(lookup_json["data"]["currentAmount"], "11");

    let list = get(&app, api_paths::ledger::ROOT).await;
    let list_json = assert_api_success(list).await;
    assert_eq!(
      list_json["data"][0]["storageId"],
      fixture.storage_a_id.to_string()
    );
  })
  .await;
}

#[tokio::test]
async fn sync_push_endpoint_rejects_payload_with_invalid_action_value_using_bad_request_error_shape(
) {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let bad_action = post_json(
      &app,
      api_paths::sync::PUSH,
      sync_push_invalid_action(
        Uuid::now_v7(),
        Uuid::now_v7(),
        Uuid::now_v7(),
        Uuid::now_v7(),
      ),
    )
    .await;

    let bad_action_json = assert_api_error(
      bad_action,
      StatusCode::UNPROCESSABLE_ENTITY,
      "VALIDATION_ERROR",
      Some("action"),
    )
    .await;
    assert_eq!(bad_action_json["error"]["code"], "VALIDATION_ERROR");
  })
  .await;
}

#[tokio::test]
async fn user_delete_endpoint_returns_structured_not_found_for_unknown_uuid() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let missing_user = delete(
      &app,
      api_paths::users::BY_ID.replace("{id}", &Uuid::now_v7().to_string()),
    )
    .await;
    let missing_user_json = assert_api_error(
      missing_user,
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
      Some("not found"),
    )
    .await;
    assert_eq!(missing_user_json["error"]["code"], "NOT_FOUND");
  })
  .await;
}

#[tokio::test]
async fn user_delete_endpoint_returns_structured_validation_error_for_malformed_uuid() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let malformed_user = delete(&app, api_paths::users::BY_ID.replace("{id}", "not-a-uuid")).await;
    let malformed_user_json = assert_api_error(
      malformed_user,
      StatusCode::BAD_REQUEST,
      "VALIDATION_ERROR",
      Some("UUID"),
    )
    .await;
    assert_eq!(malformed_user_json["error"]["code"], "VALIDATION_ERROR");
  })
  .await;
}

#[tokio::test]
async fn auth_change_password_endpoint_returns_unauthorized_for_wrong_current_password() {
  let (_db, app, token) = setup_seeded_app_with_admin_token().await;

  with_auth_token(token, async {
    let create_user = post_json(
      &app,
      api_paths::users::ROOT,
      user_create("pw-check-user", "good-pass", "Pw Check", "operator"),
    )
    .await;
    let _create_user_json = assert_api_success(create_user).await;

    let wrong_change = post_json(
      &app,
      api_paths::auth::CHANGE_PASSWORD,
      auth_change_password("pw-check-user", "wrong-pass", "new-pass-123"),
    )
    .await;
    let wrong_change_json = assert_api_error(
      wrong_change,
      StatusCode::UNAUTHORIZED,
      "UNAUTHORIZED",
      Some("Invalid credentials"),
    )
    .await;
    assert_eq!(wrong_change_json["error"]["code"], "UNAUTHORIZED");
  })
  .await;
}

#[tokio::test]
async fn user_create_endpoint_returns_not_found_when_role_seed_data_is_missing() {
  let (db, app, token) = setup_seeded_app_with_admin_token().await;

  role::Entity::delete_many()
    .filter(role::Column::CommonName.eq(enums::RoleType::Operator))
    .exec(&*db)
    .await
    .unwrap();

  with_auth_token(token, async {
    let create_without_roles = post_json(
      &app,
      api_paths::users::ROOT,
      user_create("role-missing-user", "good-pass", "No Role Seed", "operator"),
    )
    .await;
    let create_without_roles_json = assert_api_error(
      create_without_roles,
      StatusCode::NOT_FOUND,
      "NOT_FOUND",
      Some("Role"),
    )
    .await;
    assert_eq!(create_without_roles_json["error"]["code"], "NOT_FOUND");
  })
  .await;
}
