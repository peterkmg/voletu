use std::sync::Arc;

use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityLoaderTrait, EntityTrait, QueryFilter,
};
use uuid::Uuid;
use voletu_core::{
  api::ApiError,
  context::audit::with_audit_context,
  db::seed_defaults,
  dtos::{
    ChangePasswordRequest, CompleteInitializationRequest, CreateUserRequest, LoginRequest,
    UpdateUserRequest,
  },
  entities::{database_instance, local, node_base_assignment, refresh_token, user},
  enums::{self, NodeType},
  services::{system::node_bases, SystemService},
  utils::password::hash_password,
};

use crate::common::{catalog_seed::seed_inventory_catalog, setup_db, test_config};

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
async fn system_reference_reads_are_sorted_and_filterable() {
  let db = Arc::new(setup_db().await);
  let local = seed_defaults(&db).await.unwrap();
  let mut cfg = test_config();
  cfg.node.db_id = local.local_db_id;
  let cfg = Arc::new(cfg);

  let service = SystemService::new(db.clone(), cfg.clone());

  with_audit_context(Uuid::now_v7(), local.local_db_id, || async {
    let roles = service.role_list().await.unwrap();
    assert!(roles.len() >= 2);
    assert!(roles
      .windows(2)
      .all(|pair| pair[0].common_name <= pair[1].common_name));

    let operator_roles = service
      .role_query(Some(enums::RoleType::Operator))
      .await
      .unwrap();
    assert_eq!(operator_roles.len(), 1);
    let operator_role = service.role_get(operator_roles[0].id).await.unwrap();
    assert_eq!(operator_role.common_name, "OPERATOR");

    let instances = service.database_instance_list().await.unwrap();
    assert_eq!(instances.len(), 1);
    let instance_id = instances[0].id;
    let initial_node_type = instances[0].node_type.clone();
    let instance = service.database_instance_get(instance_id).await.unwrap();
    assert_eq!(instance.id, instance_id);
    assert_eq!(instance.node_type, initial_node_type);

    let updated = service
      .database_instance_update(
        instance_id,
        "Renamed Instance".to_string(),
        NodeType::Central,
        None,
      )
      .await
      .unwrap();
    assert_eq!(updated.common_name, "Renamed Instance");
    assert_eq!(updated.node_type, "CENTRAL");

    let admin_login = service
      .authenticate(&LoginRequest {
        username: "admin".to_string(),
        password: "admin".to_string(),
      })
      .await
      .unwrap();
    let _rotated = service
      .refresh_access_token(&admin_login.refresh_token)
      .await
      .unwrap();

    let refresh_tokens = service.refresh_token_list().await.unwrap();
    assert!(refresh_tokens.len() >= 2);
    assert!(refresh_tokens
      .windows(2)
      .all(|pair| pair[0].created_at >= pair[1].created_at));

    let newest = service
      .refresh_token_get(refresh_tokens[0].id)
      .await
      .unwrap();
    assert_eq!(newest.id, refresh_tokens[0].id);

    let active_tokens = service
      .refresh_token_query(Some(admin_login.user.id), Some(false))
      .await
      .unwrap();
    assert_eq!(active_tokens.len(), 1);
    assert_eq!(active_tokens[0].user_id, admin_login.user.id);

    let revoked_tokens = service
      .refresh_token_query(Some(admin_login.user.id), Some(true))
      .await
      .unwrap();
    assert!(!revoked_tokens.is_empty());
    assert!(revoked_tokens
      .iter()
      .all(|token| token.user_id == admin_login.user.id));
  })
  .await;
}

#[tokio::test]
async fn user_lifecycle_mutations_only_touch_local_users_and_restore_cleanly() {
  let db = Arc::new(setup_db().await);
  let local = seed_defaults(&db).await.unwrap();
  let mut cfg = test_config();
  cfg.node.db_id = local.local_db_id;
  let cfg = Arc::new(cfg);

  let service = SystemService::new(db.clone(), cfg.clone());
  let remote_origin_db_id = Uuid::now_v7();
  let remote_actor_id = Uuid::now_v7();

  let remote_user_id = with_audit_context(remote_actor_id, remote_origin_db_id, || async {
    user::ActiveModel {
      username: Set("remote-edit".to_string()),
      fullname: Set(Some("Remote Edit".to_string())),
      password_hash: Set(hash_password("remote-pass").await.unwrap()),
      role_id: Set(enums::RoleType::Operator.uuid()),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap()
    .id
  })
  .await;

  with_audit_context(Uuid::now_v7(), local.local_db_id, || async {
    let created_user = service
      .user_create(&CreateUserRequest {
        username: "local-edit".to_string(),
        password: "local-pass".to_string(),
        fullname: Some("Local Edit".to_string()),
        role_name: "operator".to_string(),
      })
      .await
      .unwrap();

    let remote_update = service
      .user_update(
        remote_user_id,
        &UpdateUserRequest {
          username: Some("remote-edit-2".to_string()),
          fullname: None,
          password: None,
          role_name: None,
        },
      )
      .await
      .unwrap_err();
    assert!(matches!(remote_update, ApiError::NotFound(_)));

    let updated_user = service
      .user_update(
        created_user.id,
        &UpdateUserRequest {
          username: Some("local-edit-updated".to_string()),
          fullname: Some("Local Edit Updated".to_string()),
          password: Some("local-pass-new".to_string()),
          role_name: Some("admin".to_string()),
        },
      )
      .await
      .unwrap();
    assert_eq!(updated_user.username, "local-edit-updated");
    assert_eq!(updated_user.fullname.as_deref(), Some("Local Edit Updated"));
    assert_eq!(updated_user.role, "ADMIN");

    service.user_soft_delete(created_user.id).await.unwrap();
    let deleted_login = service
      .authenticate(&LoginRequest {
        username: "local-edit-updated".to_string(),
        password: "local-pass-new".to_string(),
      })
      .await
      .unwrap_err();
    assert!(matches!(deleted_login, ApiError::Unauthorized(_)));

    service
      .user_soft_delete_undo(created_user.id)
      .await
      .unwrap();
    let restored_login = service
      .authenticate(&LoginRequest {
        username: "local-edit-updated".to_string(),
        password: "local-pass-new".to_string(),
      })
      .await
      .unwrap();
    assert_eq!(restored_login.user.id, created_user.id);

    refresh_token::Entity::delete_many()
      .filter(refresh_token::Column::UserId.eq(created_user.id))
      .exec(&*db)
      .await
      .unwrap();

    service.user_hard_delete(created_user.id).await.unwrap();
    let missing_after_hard_delete = service
      .authenticate(&LoginRequest {
        username: "local-edit-updated".to_string(),
        password: "local-pass-new".to_string(),
      })
      .await
      .unwrap_err();
    assert!(matches!(
      missing_after_hard_delete,
      ApiError::Unauthorized(_)
    ));
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
async fn node_base_helpers_load_assignments_and_ids_for_node() {
  let db = Arc::new(setup_db().await);
  let local = seed_defaults(&db).await.unwrap();
  let catalog = seed_inventory_catalog(&db).await;

  node_base_assignment::ActiveModel {
    node_id: Set(local.local_db_id),
    base_id: Set(catalog.base_id),
    ..Default::default()
  }
  .insert(&*db)
  .await
  .unwrap();

  let rows = node_bases::load_node_base_assignments(&*db, local.local_db_id)
    .await
    .unwrap();
  assert_eq!(rows.len(), 1);
  assert_eq!(rows[0].base_id, catalog.base_id);

  let base_ids = node_bases::load_node_base_ids(&*db, local.local_db_id)
    .await
    .unwrap();
  assert_eq!(base_ids, vec![catalog.base_id]);

  let row = node_bases::load_node_base_assignment(&*db, local.local_db_id, catalog.base_id)
    .await
    .unwrap()
    .unwrap();
  assert_eq!(row.base_id, catalog.base_id);
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
        node_type: None,
        node_name: None,
        central_api_url: None,
        new_username: "root".to_string(),
        new_password: "root-password".to_string(),
        fullname: Some("Root User".to_string()),
      })
      .await
      .unwrap();

    let local_state = local::Entity::load()
      .filter_by_id(1)
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
async fn complete_initialization_conflicts_when_new_username_is_already_taken_locally() {
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

    let duplicate_target_username = service
      .complete_initialization(&CompleteInitializationRequest {
        node_type: None,
        node_name: None,
        central_api_url: None,
        new_username: "main-admin".to_string(),
        new_password: "main-admin-pass-new".to_string(),
        fullname: None,
      })
      .await;

    assert!(matches!(
      duplicate_target_username,
      Err(ApiError::Conflict(_))
    ));

    let local_state = local::Entity::load()
      .filter_by_id(1)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert!(!local_state.is_initialized);

    let bootstrap_login = service
      .authenticate(&LoginRequest {
        username: "admin".to_string(),
        password: "admin".to_string(),
      })
      .await
      .unwrap();
    assert_eq!(bootstrap_login.user.username, "admin");

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
        node_type: Some(NodeType::Central),
        node_name: None,
        central_api_url: None,
        new_username: "root".to_string(),
        new_password: "root-password".to_string(),
        fullname: Some("Root User".to_string()),
      })
      .await
      .unwrap();

    let local_state = local::Entity::load()
      .filter_by_id(1)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert!(local_state.is_initialized);

    let instance = database_instance::Entity::load()
      .filter_by_id(local_state.local_db_id)
      .one(&*db)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(instance.node_type.to_string(), "CENTRAL");
  })
  .await;
}
