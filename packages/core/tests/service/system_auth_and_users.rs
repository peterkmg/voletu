use std::sync::Arc;

use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use uuid::Uuid;
use voletu_core::{
  api::ApiError,
  context::audit::with_audit_context,
  db::seed_defaults,
  dtos::{ChangePasswordRequest, CompleteInitializationRequest, CreateUserRequest, LoginRequest},
  entities::{database_instance, local, user},
  enums::{self, InitializeAdminAction, NodeType},
  services::SystemService,
  utils::password::hash_password,
};

use crate::common::{setup_db, test_config};

#[tokio::test]
async fn auth_and_user_services_handle_login_user_lifecycle_and_password_rotation() {
  let db = Arc::new(setup_db().await);
  let local = seed_defaults(&db).await.unwrap();
  let mut cfg = test_config();
  cfg.node.db_id = local.local_db_id;
  let cfg = Arc::new(cfg);

  let service = SystemService::new(db.clone(), cfg.clone());

  with_audit_context(Uuid::now_v7(), local.local_db_id, || async {
    let admin_login = service
      .authenticate(&LoginRequest {
        username: "admin".to_string(),
        password: "admin".to_string(),
      })
      .await
      .unwrap();
    assert!(!admin_login.access_token.is_empty());
    assert!(!admin_login.refresh_token.is_empty());
    service
      .verify_access(&admin_login.access_token)
      .await
      .unwrap();

    let refreshed_admin = service
      .refresh_access_token(&admin_login.refresh_token)
      .await
      .unwrap();
    assert!(!refreshed_admin.access_token.is_empty());
    assert!(!refreshed_admin.refresh_token.is_empty());
    assert_ne!(refreshed_admin.refresh_token, admin_login.refresh_token);

    let reused_admin_refresh = service
      .refresh_access_token(&admin_login.refresh_token)
      .await
      .unwrap_err();
    assert!(matches!(reused_admin_refresh, ApiError::Unauthorized(_)));

    let created_user = service
      .user_create(&CreateUserRequest {
        username: "operator1".to_string(),
        password: "operator-pass".to_string(),
        fullname: Some("Operator One".to_string()),
        role_name: "operator".to_string(),
      })
      .await
      .unwrap();
    assert_eq!(created_user.username, "operator1");

    let duplicate = service
      .user_create(&CreateUserRequest {
        username: "operator1".to_string(),
        password: "another-pass".to_string(),
        fullname: None,
        role_name: "operator".to_string(),
      })
      .await
      .unwrap_err();
    assert!(matches!(duplicate, ApiError::Conflict(_)));

    let users = service.user_list().await.unwrap();
    assert!(users.iter().any(|u| u.username == "operator1"));

    service
      .change_password(&ChangePasswordRequest {
        username: "operator1".to_string(),
        current_password: "operator-pass".to_string(),
        new_password: "operator-pass-new".to_string(),
      })
      .await
      .unwrap();

    let old_login = service
      .authenticate(&LoginRequest {
        username: "operator1".to_string(),
        password: "operator-pass".to_string(),
      })
      .await
      .unwrap_err();
    assert!(matches!(old_login, ApiError::Unauthorized(_)));

    let new_login = service
      .authenticate(&LoginRequest {
        username: "operator1".to_string(),
        password: "operator-pass-new".to_string(),
      })
      .await
      .unwrap();
    assert!(!new_login.access_token.is_empty());
    assert!(!new_login.refresh_token.is_empty());

    service.user_soft_delete(created_user.id).await.unwrap();
    let second_delete = service.user_soft_delete(created_user.id).await.unwrap_err();
    assert!(matches!(second_delete, ApiError::NotFound(_)));
  })
  .await;
}

#[tokio::test]
async fn synced_users_are_not_authenticable_on_non_origin_node_and_do_not_block_local_username_creation(
) {
  let db = Arc::new(setup_db().await);
  let local = seed_defaults(&db).await.unwrap();
  let mut cfg = test_config();
  cfg.node.db_id = local.local_db_id;
  let cfg = Arc::new(cfg);

  let service = SystemService::new(db.clone(), cfg.clone());

  let remote_origin_db_id = Uuid::now_v7();
  let remote_actor_id = Uuid::now_v7();

  with_audit_context(remote_actor_id, remote_origin_db_id, || async {
    user::ActiveModel {
      username: Set("remote-admin".to_string()),
      fullname: Set(Some("Remote Admin".to_string())),
      password_hash: Set(hash_password("admin").await.unwrap()),
      role_id: Set(enums::RoleType::Admin.uuid()),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();
  })
  .await;

  with_audit_context(Uuid::now_v7(), local.local_db_id, || async {
    let remote_admin_login = service
      .authenticate(&LoginRequest {
        username: "remote-admin".to_string(),
        password: "admin".to_string(),
      })
      .await
      .unwrap_err();
    assert!(matches!(remote_admin_login, ApiError::Unauthorized(_)));

    let local_admin_login = service
      .authenticate(&LoginRequest {
        username: "admin".to_string(),
        password: "admin".to_string(),
      })
      .await
      .unwrap();
    assert_eq!(local_admin_login.user.username, "admin");

    with_audit_context(remote_actor_id, remote_origin_db_id, || async {
      user::ActiveModel {
        username: Set("operator-same".to_string()),
        fullname: Set(Some("Remote Operator".to_string())),
        password_hash: Set(hash_password("remote-pass").await.unwrap()),
        role_id: Set(enums::RoleType::Operator.uuid()),
        ..Default::default()
      }
      .insert(&*db)
      .await
      .unwrap();
    })
    .await;

    let local_same_username = service
      .user_create(&CreateUserRequest {
        username: "operator-same".to_string(),
        password: "local-pass".to_string(),
        fullname: Some("Local Operator".to_string()),
        role_name: "operator".to_string(),
      })
      .await
      .unwrap();
    assert_eq!(local_same_username.username, "operator-same");

    let visible_users = service.user_list().await.unwrap();
    assert_eq!(visible_users.len(), 2);
    assert!(visible_users.iter().any(|u| u.username == "operator-same"));
    assert!(visible_users.iter().any(|u| u.username == "admin"));

    assert_ne!(local.local_db_id, remote_origin_db_id);
  })
  .await;
}

#[tokio::test]
async fn complete_initialization_replace_rotates_bootstrap_admin_and_marks_local_initialized() {
  let db = Arc::new(setup_db().await);
  let local_row = seed_defaults(&db).await.unwrap();
  let mut cfg = test_config();
  cfg.node.db_id = local_row.local_db_id;
  let cfg = Arc::new(cfg);

  let service = SystemService::new(db.clone(), cfg.clone());

  with_audit_context(Uuid::now_v7(), local_row.local_db_id, || async {
    service
      .complete_initialization(&CompleteInitializationRequest {
        action: InitializeAdminAction::Replace,
        node_type: None,
        new_username: Some("root".to_string()),
        new_password: Some("root-password".to_string()),
        fullname: Some("Root User".to_string()),
      })
      .await
      .unwrap();

    let local_state = local::Entity::find_by_id(1)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert!(local_state.is_initialized);

    let old_login = service
      .authenticate(&LoginRequest {
        username: "admin".to_string(),
        password: "admin".to_string(),
      })
      .await
      .unwrap_err();
    assert!(matches!(old_login, ApiError::Unauthorized(_)));

    let new_login = service
      .authenticate(&LoginRequest {
        username: "root".to_string(),
        password: "root-password".to_string(),
      })
      .await
      .unwrap();
    assert_eq!(new_login.user.username, "root");
  })
  .await;
}

#[tokio::test]
async fn complete_initialization_delete_removes_bootstrap_admin_when_another_local_admin_exists() {
  let db = Arc::new(setup_db().await);
  let local_row = seed_defaults(&db).await.unwrap();
  let mut cfg = test_config();
  cfg.node.db_id = local_row.local_db_id;
  let cfg = Arc::new(cfg);

  let service = SystemService::new(db.clone(), cfg.clone());

  with_audit_context(Uuid::now_v7(), local_row.local_db_id, || async {
    service
      .user_create(&CreateUserRequest {
        username: "main-admin".to_string(),
        password: "main-admin-pass".to_string(),
        fullname: Some("Main Admin".to_string()),
        role_name: "admin".to_string(),
      })
      .await
      .unwrap();

    service
      .complete_initialization(&CompleteInitializationRequest {
        action: InitializeAdminAction::Delete,
        node_type: None,
        new_username: None,
        new_password: None,
        fullname: None,
      })
      .await
      .unwrap();

    let local_state = local::Entity::find_by_id(1)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert!(local_state.is_initialized);

    let old_login = service
      .authenticate(&LoginRequest {
        username: "admin".to_string(),
        password: "admin".to_string(),
      })
      .await
      .unwrap_err();
    assert!(matches!(old_login, ApiError::Unauthorized(_)));

    let remaining_admin_login = service
      .authenticate(&LoginRequest {
        username: "main-admin".to_string(),
        password: "main-admin-pass".to_string(),
      })
      .await
      .unwrap();
    assert_eq!(remaining_admin_login.user.username, "main-admin");
  })
  .await;
}

#[tokio::test]
async fn complete_initialization_updates_node_type_in_db() {
  let db = Arc::new(setup_db().await);
  let local_row = seed_defaults(&db).await.unwrap();
  let mut cfg = test_config();
  cfg.node.db_id = local_row.local_db_id;
  let shared_cfg = Arc::new(cfg);

  let service = SystemService::new(db.clone(), shared_cfg.clone());

  with_audit_context(Uuid::now_v7(), local_row.local_db_id, || async {
    service
      .complete_initialization(&CompleteInitializationRequest {
        action: InitializeAdminAction::Replace,
        node_type: Some(NodeType::Central),
        new_username: Some("root".to_string()),
        new_password: Some("root-password".to_string()),
        fullname: Some("Root User".to_string()),
      })
      .await
      .unwrap();

    let local_state = local::Entity::find_by_id(1)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert!(local_state.is_initialized);

    let instance = database_instance::Entity::find_by_id(local_state.local_db_id)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(instance.node_type.to_string(), "CENTRAL");
  })
  .await;
}
