use rand::{rngs::StdRng, Rng, SeedableRng};
use uuid::Uuid;

pub const SUITE_SEED: u64 = 0x9E3779B97F4A7C15;

pub struct SeedFactory {
  rng: StdRng,
}

impl SeedFactory {
  pub fn with_suite_seed() -> Self {
    Self {
      rng: StdRng::seed_from_u64(SUITE_SEED),
    }
  }

  pub fn with_seed(seed: u64) -> Self {
    Self {
      rng: StdRng::seed_from_u64(seed),
    }
  }

  pub fn next_uuid(&mut self) -> Uuid {
    let mut bytes = [0_u8; 16];
    self.rng.fill_bytes(&mut bytes);
    Uuid::from_bytes(bytes)
  }

  pub fn alpha_num(&mut self, prefix: &str, len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let suffix: String = (0..len)
      .map(|_| {
        let idx = (self.rng.next_u32() as usize) % CHARSET.len();
        CHARSET[idx] as char
      })
      .collect();
    format!("{prefix}-{suffix}")
  }

  pub fn bounded_u32(&mut self, min: u32, max_inclusive: u32) -> u32 {
    let range = max_inclusive.saturating_sub(min).saturating_add(1);
    min + (self.rng.next_u32() % range)
  }
}
