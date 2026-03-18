use serde::{Deserialize, Deserializer};

fn deserialize_optional_u64_from_string<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
  D: Deserializer<'de>,
{
  #[derive(Deserialize)]
  #[serde(untagged)]
  enum U64OrString {
    U64(u64),
    String(String),
  }

  match Option::<U64OrString>::deserialize(deserializer)? {
    None => Ok(None),
    Some(U64OrString::U64(value)) => Ok(Some(value)),
    Some(U64OrString::String(value)) => value
      .parse::<u64>()
      .map(Some)
      .map_err(serde::de::Error::custom),
  }
}

#[derive(Debug, Default, Deserialize)]
pub struct PaginationParams {
  #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
  pub page: Option<u64>,
  #[serde(default, deserialize_with = "deserialize_optional_u64_from_string")]
  pub per_page: Option<u64>,
}
