use std::time::Duration;

use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::api::ApiResponse;

pub fn normalize_base_url(value: &str) -> String {
  value.trim_end_matches('/').to_string()
}

pub async fn get_api_json<T: DeserializeOwned>(
  client: &Client,
  url: &str,
  timeout: Duration,
) -> anyhow::Result<T> {
  let response = client.get(url).timeout(timeout).send().await?;
  parse_api_response(response, "GET", url).await
}

pub async fn post_api_json<Req: Serialize, Res: DeserializeOwned>(
  client: &Client,
  url: &str,
  body: &Req,
  timeout: Duration,
) -> anyhow::Result<Res> {
  let response = client.post(url).json(body).timeout(timeout).send().await?;
  parse_api_response(response, "POST", url).await
}

async fn parse_api_response<T: DeserializeOwned>(
  response: reqwest::Response,
  method: &str,
  url: &str,
) -> anyhow::Result<T> {
  let status = response.status();
  let body = response.text().await?;

  let envelope = serde_json::from_str::<ApiResponse<T>>(&body).map_err(|e| {
    anyhow::anyhow!(
      "failed to parse API response (status: {}, body: {}): {e:#}",
      status,
      body
    )
  })?;

  envelope.into_anyhow_data(method, url, status)
}
