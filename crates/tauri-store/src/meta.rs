use crate::collection::StoreCollection;
use crate::error::Result;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{Manager, Runtime};
use tauri_store_utils::{read_file, write_file};

#[cfg(feature = "unstable-migration")]
use crate::migration::MigrationHistory;

#[cfg(debug_assertions)]
const FILENAME: &str = "meta.dev.tauristore";
#[cfg(not(debug_assertions))]
const FILENAME: &str = "meta.tauristore";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Meta {
  pub(crate) path: Option<PathBuf>,
  version: Version,

  #[cfg(feature = "unstable-migration")]
  migration: MigrationHistory,
}

impl Meta {
  pub(crate) fn read<R, M>(app: &M, name: &str) -> Result<Self>
  where
    R: Runtime,
    M: Manager<R>,
  {
    let path = meta_file_path(app, name)?;
    let meta = read_file(&path)
      .create(true)
      .create_pretty(true)
      .create_sync(cfg!(feature = "file-sync-all"))
      .call()?;

    Ok(meta)
  }

  pub(crate) fn write<R>(collection: &StoreCollection<R>) -> Result<()>
  where
    R: Runtime,
  {
    let mut meta = Self::read(&collection.app, &collection.name)?;
    meta.path = Some(collection.path());
    meta.version = current_version();

    let path = meta_file_path(&collection.app, &collection.name)?;
    write_file(path, &meta)
      .pretty(true)
      .sync(cfg!(feature = "file-sync-all"))
      .call()?;

    Ok(())
  }
}

impl Default for Meta {
  fn default() -> Self {
    Self {
      path: None,
      version: current_version(),

      #[cfg(feature = "unstable-migration")]
      migration: MigrationHistory::default(),
    }
  }
}

fn meta_file_path<R, M>(app: &M, name: &str) -> Result<PathBuf>
where
  R: Runtime,
  M: Manager<R>,
{
  let path = app
    .path()
    .app_config_dir()?
    .join(name)
    .join(FILENAME);

  Ok(path)
}

fn current_version() -> Version {
  Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}
