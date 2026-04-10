use std::{sync::Arc, time::Duration};

use axum::{
  routing::{get, post},
  Json,
  Router,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use tokio::sync::RwLock;
use voletu_core::{
  api::ApiResponse,
  context::audit::with_audit_context,
  dtos::{PullAuditLogsResponse, PushAuditLogsRequest, PushAuditLogsResponse, SyncStatusResponse},
  entities::{database_instance, local},
  worker::{spawn_sync_worker_with_config, WorkerState, WorkerStatus},
  SyncConfig,
};

use crate::common::{setup_db, test_config};

fn test_sync_config() -> SyncConfig {
  SyncConfig {
    tick_interval: Duration::from_millis(50),
    probe_timeout: Duration::from_millis(100),
    request_timeout: Duration::from_millis(100),
    sync_batch_limit: 100,
  }
}

async fn spawn_sync_test_server(
  status: SyncStatusResponse,
) -> (String, tokio::task::JoinHandle<()>) {
  async fn sync_status(status: Arc<SyncStatusResponse>) -> Json<ApiResponse<SyncStatusResponse>> {
    Json(ApiResponse::success(SyncStatusResponse {
      node_id: status.node_id,
      node_type: status.node_type.clone(),
      highest_audit_log_id: status.highest_audit_log_id,
      highest_matching_id: status.highest_matching_id,
    }))
  }

  async fn sync_pull() -> Json<ApiResponse<PullAuditLogsResponse>> {
    Json(ApiResponse::success(PullAuditLogsResponse {
      highest_evaluated_id: uuid::Uuid::nil(),
      logs: Vec::new(),
    }))
  }

  async fn sync_push(
    Json(req): Json<PushAuditLogsRequest>,
  ) -> Json<ApiResponse<PushAuditLogsResponse>> {
    Json(ApiResponse::success(PushAuditLogsResponse {
      accepted: req.logs.len() as u64,
      rejected: 0,
    }))
  }

  let status = Arc::new(status);
  let app = Router::new()
    .route(
      "/sync/status",
      get({
        let status = Arc::clone(&status);
        move || sync_status(status.clone())
      }),
    )
    .route("/sync/pull", get(sync_pull))
    .route("/sync/push", post(sync_push));

  let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
  let address = listener.local_addr().unwrap();
  let handle = tokio::spawn(async move {
    axum::serve(listener, app).await.unwrap();
  });

  (format!("http://{}", address), handle)
}

async fn wait_for_tick(
  worker_status: &Arc<RwLock<WorkerStatus>>,
  expected: WorkerState,
  timeout: Duration,
) -> WorkerStatus {
  let deadline = tokio::time::Instant::now() + timeout;
  loop {
    let snapshot = worker_status.read().await.clone();
    if snapshot.state == expected && snapshot.tick_count() > 0 {
      return snapshot;
    }

    assert!(
      tokio::time::Instant::now() < deadline,
      "worker did not reach {:?} with observed tick within {:?}; current={:?}, ticks={}",
      expected,
      timeout,
      snapshot.state,
      snapshot.tick_count()
    );
    tokio::time::sleep(Duration::from_millis(25)).await;
  }
}

#[tokio::test]
async fn worker_stays_sleeping_for_non_peripheral_topology_and_bumps_ticks() {
  let db = Arc::new(setup_db().await);
  let instance = with_audit_context(uuid::Uuid::now_v7(), uuid::Uuid::now_v7(), || async {
    let instance = database_instance::ActiveModel {
      common_name: Set("Local Central".to_string()),
      node_type: Set(voletu_core::enums::NodeType::Central),
      base_id: Set(None),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    local::ActiveModel {
      id: Set(1),
      is_initialized: Set(true),
      local_db_id: Set(instance.id),
      jwt_secret: Set("test-secret".to_string()),
      central_api_url: Set(None),
    }
    .insert(&*db)
    .await
    .unwrap();

    instance
  })
  .await;

  let mut cfg = test_config();
  cfg.node.db_id = instance.id;
  let worker_status = Arc::new(RwLock::new(WorkerStatus::default()));
  let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
  let handle = spawn_sync_worker_with_config(
    db,
    Arc::new(cfg),
    shutdown_rx,
    test_sync_config(),
    worker_status.clone(),
  );

  let _snapshot = wait_for_tick(
    &worker_status,
    WorkerState::Sleeping,
    Duration::from_secs(2),
  )
  .await;

  let _ = shutdown_tx.send(());
  handle.await.unwrap();
}

#[tokio::test]
async fn worker_marks_offline_when_central_probe_fails() {
  let db = Arc::new(setup_db().await);
  let instance = with_audit_context(uuid::Uuid::now_v7(), uuid::Uuid::now_v7(), || async {
    let instance = database_instance::ActiveModel {
      common_name: Set("Local Peripheral".to_string()),
      node_type: Set(voletu_core::enums::NodeType::Peripheral),
      base_id: Set(None),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    local::ActiveModel {
      id: Set(1),
      is_initialized: Set(true),
      local_db_id: Set(instance.id),
      jwt_secret: Set("test-secret".to_string()),
      central_api_url: Set(Some("http://127.0.0.1:9".to_string())),
    }
    .insert(&*db)
    .await
    .unwrap();

    instance
  })
  .await;

  let mut cfg = test_config();
  cfg.node.db_id = instance.id;
  let worker_status = Arc::new(RwLock::new(WorkerStatus::default()));
  let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
  let handle = spawn_sync_worker_with_config(
    db,
    Arc::new(cfg),
    shutdown_rx,
    test_sync_config(),
    worker_status.clone(),
  );

  let _snapshot = wait_for_tick(&worker_status, WorkerState::Offline, Duration::from_secs(2)).await;

  let _ = shutdown_tx.send(());
  handle.await.unwrap();
}

#[tokio::test]
async fn worker_completes_noop_cycle_and_settles_online_idle() {
  let db = Arc::new(setup_db().await);
  let remote_node_id = uuid::Uuid::now_v7();
  let (central_api_url, server_handle) = spawn_sync_test_server(SyncStatusResponse {
    node_id: remote_node_id,
    node_type: "CENTRAL".to_string(),
    highest_audit_log_id: uuid::Uuid::nil(),
    highest_matching_id: uuid::Uuid::nil(),
  })
  .await;

  let instance = with_audit_context(uuid::Uuid::now_v7(), uuid::Uuid::now_v7(), || async {
    let instance = database_instance::ActiveModel {
      common_name: Set("Local Peripheral".to_string()),
      node_type: Set(voletu_core::enums::NodeType::Peripheral),
      base_id: Set(None),
      ..Default::default()
    }
    .insert(&*db)
    .await
    .unwrap();

    local::ActiveModel {
      id: Set(1),
      is_initialized: Set(true),
      local_db_id: Set(instance.id),
      jwt_secret: Set("test-secret".to_string()),
      central_api_url: Set(Some(central_api_url)),
    }
    .insert(&*db)
    .await
    .unwrap();

    instance
  })
  .await;

  let mut cfg = test_config();
  cfg.node.db_id = instance.id;
  let worker_status = Arc::new(RwLock::new(WorkerStatus::default()));
  let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
  let handle = spawn_sync_worker_with_config(
    db,
    Arc::new(cfg),
    shutdown_rx,
    test_sync_config(),
    worker_status.clone(),
  );

  let snapshot = wait_for_tick(
    &worker_status,
    WorkerState::OnlineIdle,
    Duration::from_secs(2),
  )
  .await;
  assert!(snapshot.cycle_count() >= 1);

  let _ = shutdown_tx.send(());
  handle.await.unwrap();
  server_handle.abort();
}
