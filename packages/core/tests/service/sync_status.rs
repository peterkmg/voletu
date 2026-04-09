use std::sync::Arc;

use chrono::{Duration, Utc};
use uuid::Uuid;
use voletu_core::{
  services::sync::{query::AwaitCycleQuerySpec, SyncService},
  worker::{WorkerState, WorkerStatus},
};

use crate::common::{setup_db, test_config};

const TEST_SYNC_NODE_ID: Uuid = Uuid::from_u128(11);

fn sync_service_with_node(db: Arc<sea_orm::DatabaseConnection>, node_id: Uuid) -> SyncService {
  let mut cfg = test_config();
  cfg.node.db_id = node_id;
  SyncService::new(db, Arc::new(cfg))
}

#[tokio::test]
async fn await_cycle_returns_incomplete_immediately_when_worker_is_sleeping() {
  let db = Arc::new(setup_db().await);
  let service = sync_service_with_node(db, TEST_SYNC_NODE_ID);
  let worker_status = Arc::new(tokio::sync::RwLock::new(WorkerStatus::default()));

  let response = service
    .await_cycle(&worker_status, AwaitCycleQuerySpec::new(1, None))
    .await
    .unwrap();

  assert_eq!(response.worker_state, "Sleeping");
  assert!(!response.completed);
  assert!(response.last_sync_at.is_none());
}

#[tokio::test]
async fn await_cycle_returns_completed_immediately_when_since_is_already_satisfied() {
  let db = Arc::new(setup_db().await);
  let service = sync_service_with_node(db, TEST_SYNC_NODE_ID);
  let worker_status = Arc::new(tokio::sync::RwLock::new(WorkerStatus::default()));
  let last_sync_at = Utc::now();

  {
    let mut status = worker_status.write().await;
    status.state = WorkerState::OnlineIdle;
    status.last_sync_at = Some(last_sync_at);
  }

  let response = service
    .await_cycle(
      &worker_status,
      AwaitCycleQuerySpec::new(1, Some(last_sync_at - Duration::seconds(1))),
    )
    .await
    .unwrap();

  assert_eq!(response.worker_state, "OnlineIdle");
  assert!(response.completed);
  let expected_last_sync = last_sync_at.to_rfc3339();
  assert_eq!(
    response.last_sync_at.as_deref(),
    Some(expected_last_sync.as_str())
  );
}

#[tokio::test]
async fn await_cycle_completes_after_cycle_counter_advances() {
  let db = Arc::new(setup_db().await);
  let service = sync_service_with_node(db, TEST_SYNC_NODE_ID);
  let worker_status = Arc::new(tokio::sync::RwLock::new(WorkerStatus::default()));

  {
    let mut status = worker_status.write().await;
    status.state = WorkerState::Offline;
  }

  let worker_status_for_task = worker_status.clone();
  let expected_last_sync = Utc::now();
  let join = tokio::spawn(async move {
    tokio::time::sleep(std::time::Duration::from_millis(60)).await;
    let mut status = worker_status_for_task.write().await;
    status.state = WorkerState::Syncing;
    status.last_sync_at = Some(expected_last_sync);
    status.bump_cycle_count();
  });

  let response = service
    .await_cycle(&worker_status, AwaitCycleQuerySpec::new(1, None))
    .await
    .unwrap();

  join.await.unwrap();

  assert_eq!(response.worker_state, "Syncing");
  assert!(response.completed);
  let expected_last_sync = expected_last_sync.to_rfc3339();
  assert_eq!(
    response.last_sync_at.as_deref(),
    Some(expected_last_sync.as_str())
  );
}

#[tokio::test]
async fn await_cycle_completes_after_two_idle_ticks_without_cycle_progress() {
  let db = Arc::new(setup_db().await);
  let service = sync_service_with_node(db, TEST_SYNC_NODE_ID);
  let worker_status = Arc::new(tokio::sync::RwLock::new(WorkerStatus::default()));

  {
    let mut status = worker_status.write().await;
    status.state = WorkerState::Offline;
  }

  let worker_status_for_task = worker_status.clone();
  let expected_last_sync = Utc::now();
  let join = tokio::spawn(async move {
    tokio::time::sleep(std::time::Duration::from_millis(60)).await;
    {
      let mut status = worker_status_for_task.write().await;
      status.state = WorkerState::Offline;
      status.bump_tick_count();
    }
    tokio::time::sleep(std::time::Duration::from_millis(60)).await;
    let mut status = worker_status_for_task.write().await;
    status.state = WorkerState::OnlineIdle;
    status.last_sync_at = Some(expected_last_sync);
    status.bump_tick_count();
  });

  let response = service
    .await_cycle(&worker_status, AwaitCycleQuerySpec::new(1, None))
    .await
    .unwrap();

  join.await.unwrap();

  assert_eq!(response.worker_state, "OnlineIdle");
  assert!(response.completed);
  let expected_last_sync = expected_last_sync.to_rfc3339();
  assert_eq!(
    response.last_sync_at.as_deref(),
    Some(expected_last_sync.as_str())
  );
}
