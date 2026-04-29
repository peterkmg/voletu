use std::ops::Range;

use chrono::{DateTime, Duration, Utc};
use fake::{faker::lorem::en::Sentence, Fake};
use rand::RngExt;
use sea_orm::ActiveValue;
use uuid::Uuid;

use crate::api::ApiError;

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
  display_sequence: usize,
}

impl SeedTag {
  pub fn new(now: DateTime<Utc>, run_id: Uuid, display_sequence: usize) -> Self {
    let run_code = run_id.as_simple().to_string();
    Self {
      date_code: now.format("%y%m%d").to_string(),
      run_code,
      display_sequence,
    }
  }

  pub fn document_number(&self, prefix: &str, serial: usize) -> String {
    format!(
      "{prefix}-{}-{}-{:05}",
      self.date_code,
      self.run_code,
      serial + 1
    )
  }

  pub fn display_sequence(&self) -> usize {
    self.display_sequence
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

pub fn versioned_name(tag: &SeedTag, base: impl Into<String>) -> String {
  let base = base.into();
  if tag.display_sequence() <= 1 {
    base
  } else {
    format!("{base} {}", tag.display_sequence())
  }
}

pub fn numbered_name(base: impl AsRef<str>, serial: usize) -> String {
  format!("{} {}", base.as_ref(), serial + 1)
}

pub fn random_username(_rng: &mut rand::rngs::StdRng, _tag: &SeedTag, index: usize) -> String {
  format!("devuser{:03}", index + 1)
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
