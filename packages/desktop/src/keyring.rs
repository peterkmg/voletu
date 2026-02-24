use anyhow::{anyhow, Context};
use keyring::Entry;

use crate::constants::{KEYRING_SERVICE, KEYRING_USERNAME};

fn keyring_entry() -> anyhow::Result<Entry> {
  Entry::new(KEYRING_SERVICE, KEYRING_USERNAME).context("failed to create keyring entry")
}

pub fn load_db_password() -> anyhow::Result<Option<String>> {
  let entry = keyring_entry()?;
  match entry.get_password() {
    Ok(password) => Ok(Some(password)),
    Err(keyring::Error::NoEntry) => Ok(None),
    Err(err) => Err(anyhow!("failed to read db password from keyring: {err}")),
  }
}

pub fn save_db_password(password: &str) -> anyhow::Result<()> {
  let entry = keyring_entry()?;
  entry
    .set_password(password)
    .context("failed to save db password into keyring")
}

pub fn clear_db_password() -> anyhow::Result<()> {
  let entry = keyring_entry()?;
  match entry.delete_credential() {
    Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
    Err(err) => Err(anyhow!("failed to clear db password from keyring: {err}")),
  }
}
