use crate::collection::StoreCollection;
use crate::error::Result;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{Manager, Runtime};
use tauri_store_utils::{read_file, write_file};

#[cfg(debug_assertions)]
const FILENAME: &str = "meta.dev.tauristore";
#[cfg(not(debug_assertions))]
const FILENAME: &str = "meta.tauristore";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Meta {
  pub(crate) path: Option<PathBuf>,
  pub(crate) version: Version,
}

impl Default for Meta {
  fn default() -> Self {
    Self {
      path: None,
      version: current_version(),
    }
  }
}

pub(crate) fn load<R>(collection: &StoreCollection<R>) -> Result<()>
where
  R: Runtime,
{
  let path = path(collection)?;
  let meta: Meta = read_file(&path)
    .create(true)
    .create_pretty(true)
    .create_sync(cfg!(feature = "file-sync-all"))
    .call()?;

  if let Some(path) = meta.path {
    *collection.path.lock().unwrap() = path;
  }

  Ok(())
}

pub(crate) fn save<R>(collection: &StoreCollection<R>) -> Result<()>
where
  R: Runtime,
{
  let meta = Meta {
    path: Some(collection.path()),
    version: current_version(),
  };

  write_file(path(collection)?, &meta)
    .sync(cfg!(feature = "file-sync-all"))
    .pretty(true)
    .call()?;

  Ok(())
}

fn path<R>(collection: &StoreCollection<R>) -> Result<PathBuf>
where
  R: Runtime,
{
  let path = collection
    .app
    .path()
    .app_config_dir()?
    .join(collection.name.as_ref())
    .join(FILENAME);

  Ok(path)
}

fn current_version() -> Version {
  Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}
