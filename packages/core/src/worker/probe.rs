use super::context::WorkerContext;
use crate::{
  config::SyncConfig,
  dtos::SyncStatusResponse,
  services::sync::helpers::join_uuid_csv,
  utils::http::get_api_json,
};

pub(super) enum RemoteSyncProbe {
  Offline,
  Online(SyncStatusResponse),
}

pub(super) async fn probe_remote_sync_status(
  client: &reqwest::Client,
  config: &SyncConfig,
  context: &WorkerContext,
) -> RemoteSyncProbe {
  let base_ids_param = join_uuid_csv(&context.local_base_ids);

  match get_api_json::<SyncStatusResponse>(
    client,
    &format!(
      "{}/sync/status?baseIds={}",
      context.central_sync_api_url, base_ids_param
    ),
    config.probe_timeout,
  )
  .await
  {
    Ok(status) => RemoteSyncProbe::Online(status),
    Err(_) => RemoteSyncProbe::Offline,
  }
}
