use std::fmt;

use serde::{Deserialize, Deserializer};

/// Filter for nullable FK columns: `?field=isNull` or `?field=isNotNull`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullableFilter {
  IsNull,
  IsNotNull,
}

impl<'de> Deserialize<'de> for NullableFilter {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
      "isNull" => Ok(NullableFilter::IsNull),
      "isNotNull" => Ok(NullableFilter::IsNotNull),
      other => Err(serde::de::Error::custom(format!(
        "invalid NullableFilter value '{}', expected 'isNull' or 'isNotNull'",
        other
      ))),
    }
  }
}

impl fmt::Display for NullableFilter {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      NullableFilter::IsNull => write!(f, "isNull"),
      NullableFilter::IsNotNull => write!(f, "isNotNull"),
    }
  }
}

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

#[derive(Debug, Default, Deserialize)]
pub struct EmbedParams {
  pub embed: Option<String>,
}

impl EmbedParams {
  pub fn wants_names(&self) -> bool {
    self.embed.as_deref() == Some("names")
  }
}
