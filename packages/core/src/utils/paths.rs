use std::path::{Path, PathBuf};

use anyhow::anyhow;

/// Creates `path` and all its parents if they do not already exist.
pub fn ensure_dir(path: &Path) -> anyhow::Result<()> {
  if !path.as_os_str().is_empty() {
    std::fs::create_dir_all(path)?;
  }
  Ok(())
}

/// Creates the parent directory of `path` (and its ancestors) if they do not
/// already exist. Has no effect when `path` has no parent component.
pub fn ensure_parent_dir(path: &Path) -> anyhow::Result<()> {
  if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
    std::fs::create_dir_all(parent)?;
  }
  Ok(())
}

/// Splits `path` into its parent directory and UTF-8 file name.
/// Returns an error if either component is absent or the name is not valid UTF-8.
pub fn split_file_path(path: &Path) -> anyhow::Result<(&Path, &str)> {
  let parent = path
    .parent()
    .filter(|p| !p.as_os_str().is_empty())
    .ok_or_else(|| anyhow!("path has no parent directory: {}", path.display()))?;
  let name = path
    .file_name()
    .ok_or_else(|| anyhow!("path has no file name: {}", path.display()))?
    .to_str()
    .ok_or_else(|| anyhow!("file name is not valid UTF-8: {}", path.display()))?;
  Ok((parent, name))
}

/// Resolves `path` relative to `base` if it is relative; returns it
/// unchanged if it is already absolute.
pub fn resolve_relative(path: &Path, base: &Path) -> PathBuf {
  if path.is_absolute() {
    path.to_path_buf()
  } else {
    base.join(path)
  }
}
