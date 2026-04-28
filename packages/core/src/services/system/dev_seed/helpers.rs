use std::ops::Range;

use chrono::{DateTime, Duration, Utc};
use fake::{
  faker::{
    address::en::CityName,
    company::en::CompanyName,
    internet::en::Username,
    lorem::en::Sentence,
  },
  Fake,
};
use rand::RngExt;
use sea_orm::ActiveValue;
use uuid::Uuid;

use crate::api::ApiError;

const RUN_SHORT_LEN: usize = 8;

pub const PRODUCT_FAMILIES: &[&str] = &[
  "Crude Oil",
  "Gasoline",
  "Diesel",
  "Jet Fuel",
  "Lubricants",
  "LPG",
  "Bitumen",
  "Naphtha",
];

pub const BASE_SUFFIXES: &[&str] = &[
  "Terminal",
  "Depot",
  "Storage Base",
  "Tank Farm",
  "Logistics Hub",
];

pub const STORAGE_LABELS: &[&str] = &["Tank", "Cell", "Bay", "Reservoir", "Line"];
pub const COMPANY_ROLE_TAILS: &[&str] = &["Trading", "Logistics", "Terminal", "Energy", "Supply"];
pub const LEGAL_SUFFIXES: &[&str] = &["LLC", "Ltd.", "Inc.", "GmbH", "Zrt."];

#[derive(Debug, Clone)]
pub struct SeedTag {
  date_code: String,
  run_code: String,
  run_short: String,
}

impl SeedTag {
  pub fn new(now: DateTime<Utc>, run_id: Uuid) -> Self {
    let run_code = run_id.as_simple().to_string();
    let run_short = run_code[..RUN_SHORT_LEN].to_string();
    Self {
      date_code: now.format("%y%m%d").to_string(),
      run_code,
      run_short,
    }
  }

  pub fn run_short(&self) -> &str {
    &self.run_short
  }

  pub fn document_number(&self, prefix: &str, serial: usize) -> String {
    format!(
      "{prefix}-{}-{}-{:05}",
      self.date_code,
      self.run_code,
      serial + 1
    )
  }
}

pub fn pick<'a, T>(rng: &mut rand::rngs::StdRng, items: &'a [T]) -> &'a T {
  &items[rng.random_range(0..items.len())]
}

pub fn maybe<T>(
  rng: &mut rand::rngs::StdRng,
  chance: f64,
  value: impl FnOnce(&mut rand::rngs::StdRng) -> T,
) -> Option<T> {
  if rng.random_bool(chance) {
    Some(value(rng))
  } else {
    None
  }
}

pub fn random_name(rng: &mut rand::rngs::StdRng, tag: &SeedTag, base: impl Into<String>) -> String {
  format!(
    "{} {:03} {}",
    base.into(),
    rng.random_range(100..=999),
    tag.run_short()
  )
}

pub fn random_username(_rng: &mut rand::rngs::StdRng, tag: &SeedTag, index: usize) -> String {
  let raw = Username().fake::<String>();
  let cleaned: String = raw
    .chars()
    .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '.')
    .collect();
  let base = if cleaned.is_empty() {
    "devuser".to_string()
  } else {
    cleaned.to_lowercase()
  };
  format!("{base}-{}-{:02}", tag.run_short(), index + 1)
}

pub fn company_name(rng: &mut rand::rngs::StdRng, tag: &SeedTag) -> String {
  let tail = pick(rng, COMPANY_ROLE_TAILS);
  random_name(
    rng,
    tag,
    format!("{} {}", CompanyName().fake::<String>(), tail),
  )
}

pub fn location_name(rng: &mut rand::rngs::StdRng, tag: &SeedTag) -> String {
  let suffix = pick(rng, BASE_SUFFIXES);
  random_name(
    rng,
    tag,
    format!("{} {}", CityName().fake::<String>(), suffix),
  )
}

pub fn fake_fragment(words: Range<usize>) -> String {
  Sentence(words)
    .fake::<String>()
    .trim_end_matches('.')
    .to_string()
}

pub fn title_fragment(input: String) -> String {
  input
    .split_whitespace()
    .map(|word| {
      let mut chars = word.chars();
      match chars.next() {
        Some(first) => {
          let mut out = first.to_uppercase().collect::<String>();
          out.push_str(chars.as_str());
          out
        }
        None => String::new(),
      }
    })
    .collect::<Vec<_>>()
    .join(" ")
}

pub fn random_document_number(tag: &SeedTag, prefix: &str, serial: usize) -> String {
  tag.document_number(prefix, serial)
}

pub fn random_date(now: DateTime<Utc>, rng: &mut rand::rngs::StdRng) -> chrono::NaiveDate {
  (now - Duration::days(rng.random_range(0..=730) as i64)).date_naive()
}

pub fn random_datetime(now: DateTime<Utc>, rng: &mut rand::rngs::StdRng) -> DateTime<Utc> {
  now - Duration::days(rng.random_range(0..=730) as i64)
}

pub fn saved_uuid(value: ActiveValue<Uuid>, label: &'static str) -> Result<Uuid, ApiError> {
  match value {
    ActiveValue::Set(id) | ActiveValue::Unchanged(id) => Ok(id),
    ActiveValue::NotSet => Err(ApiError::Internal(anyhow::anyhow!(
      "{label} graph save returned no id"
    ))),
  }
}
