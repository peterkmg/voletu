use anyhow::{anyhow, Context};
use keyring_core::{Entry, Error};

use crate::constants::{KEYRING_SERVICE, KEYRING_USERNAME};

pub fn initialize_keyring() -> anyhow::Result<()> {
  #[cfg(windows)]
  {
    keyring_core::set_default_store(
      windows_native_keyring_store::Store::new()
        .context("failed to initialize Windows credential store")?,
    );
    return Ok(());
  }

  #[cfg(target_os = "macos")]
  {
    keyring_core::set_default_store(
      apple_native_keyring_store::keychain::Store::new()
        .context("failed to initialize macOS keychain store")?,
    );
    return Ok(());
  }

  #[cfg(target_os = "linux")]
  {
    keyring_core::set_default_store(
      dbus_secret_service_keyring_store::Store::new()
        .context("failed to initialize Linux Secret Service store")?,
    );
    return Ok(());
  }

  #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
  {
    anyhow::bail!("native keyring store is unsupported on this platform");
  }
}

fn keyring_entry() -> anyhow::Result<Entry> {
  Entry::new(KEYRING_SERVICE, KEYRING_USERNAME).context("failed to create keyring entry")
}

pub fn load_db_password() -> anyhow::Result<Option<String>> {
  let entry = keyring_entry()?;
  match entry.get_password() {
    Ok(password) => Ok(Some(password)),
    Err(Error::NoEntry) => Ok(None),
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
    Ok(()) | Err(Error::NoEntry) => Ok(()),
    Err(err) => Err(anyhow!("failed to clear db password from keyring: {err}")),
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Mutex;

  use keyring_core::{mock, set_default_store, unset_default_store};

  use super::*;

  static KEYRING_TEST_LOCK: Mutex<()> = Mutex::new(());

  fn with_mock_keyring(test: impl FnOnce()) {
    let _guard = KEYRING_TEST_LOCK
      .lock()
      .expect("keyring test lock poisoned");
    let previous_store = unset_default_store();
    set_default_store(mock::Store::new().expect("mock keyring store should initialize"));

    test();

    unset_default_store();
    if let Some(previous_store) = previous_store {
      set_default_store(previous_store);
    }
  }

  #[test]
  fn db_password_can_be_saved_loaded_and_cleared() {
    with_mock_keyring(|| {
      assert_eq!(load_db_password().unwrap(), None);
      clear_db_password().unwrap();

      save_db_password("test-password").unwrap();
      assert_eq!(
        load_db_password().unwrap().as_deref(),
        Some("test-password")
      );

      clear_db_password().unwrap();
      assert_eq!(load_db_password().unwrap(), None);
    });
  }
}
