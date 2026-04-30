use std::sync::Arc;

use uuid::Uuid;
use voletu_core::{
  config::NodeConfig,
  services::SystemService,
  utils::{
    jwt::{generate_secret, Claims},
    password::{hash_password, verify_password},
  },
};

use crate::common::{setup_db, test_config};

#[tokio::test]
async fn password_hash_verifies_correct_and_rejects_wrong() {
  let password = "super-secret-pass";
  let hash = hash_password(password).await.unwrap();

  assert_ne!(hash, password);
  assert!(verify_password(password, &hash).await.unwrap());
  assert!(!verify_password("wrong-password", &hash).await.unwrap());
}

#[tokio::test]
async fn token_create_verifies_correct_secret_and_rejects_mismatch() {
  let db = Arc::new(setup_db().await);
  let mut cfg = test_config();
  cfg.node = NodeConfig::new(
    Uuid::now_v7(),
    "PERIPHERAL".to_string(),
    generate_secret(),
    None,
  );

  let shared_cfg = Arc::new(cfg.clone());
  let service = SystemService::new(db.clone(), shared_cfg);
  let user_id = Uuid::now_v7();
  let token = service
    .access_create(user_id, "operator-a", "operator")
    .await
    .unwrap();
  let claims = service.verify_access(&token).await.unwrap();

  assert_eq!(claims.uid, user_id);
  assert_eq!(claims.sub, "operator-a");
  assert_eq!(claims.role, "operator");
  assert!(claims.exp >= claims.iat);

  cfg.node.jwt_secret = generate_secret();
  let other_cfg = Arc::new(cfg);
  let other = SystemService::new(db, other_cfg);
  assert!(other.verify_access(&token).await.is_err());
}

#[tokio::test]
async fn claims_new_rejects_invalid_expiration_overflow() {
  let err = Claims::new(
    Uuid::now_v7(),
    "subject".to_string(),
    "operator".to_string(),
    i64::MAX,
  )
  .unwrap_err();
  assert!(err.to_string().contains("Invalid expiration time"));
}
