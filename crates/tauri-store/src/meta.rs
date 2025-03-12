use crate::collection::StoreCollection;
use crate::error::Result;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};
use tauri_store_utils::{read_file, write_file, WriteFileOptions};

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
  let path = path(&collection.app)?;
  let meta = read_file::<Meta>(&path)?;

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

  write_file(path(&collection.app)?, &meta, &WriteFileOptions::default())?;

  Ok(())
}

fn path<R>(app: &AppHandle<R>) -> Result<PathBuf>
where
  R: Runtime,
{
  app
    .path()
    .app_config_dir()
    .map(|dir| dir.join(FILENAME))
    .map_err(Into::into)
}

fn current_version() -> Version {
  Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}
