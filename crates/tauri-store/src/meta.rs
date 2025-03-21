use crate::collection::StoreCollection;
use crate::error::Result;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};
use tauri::{Manager, Runtime};
use tauri_store_utils::{read_file, write_file};

#[cfg(feature = "unstable-migration")]
use crate::migration::MigrationHistory;

#[cfg(debug_assertions)]
const FILENAME: &str = "meta.dev.tauristore";
#[cfg(not(debug_assertions))]
const FILENAME: &str = "meta.tauristore";

// We cannot use `LazyLock` because our MSRV is 1.77.2.
static LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Meta {
  pub path: Option<PathBuf>,
  pub version: Version,

  #[cfg(feature = "unstable-migration")]
  pub migration_history: Option<MigrationHistory>,
}

impl Meta {
  pub(crate) fn read<R, M>(app: &M, name: &str) -> Result<MetaGuard>
  where
    R: Runtime,
    M: Manager<R>,
  {
    let guard = LOCK
      .get_or_init(Mutex::default)
      .lock()
      .expect("meta lock is poisoned");

    let path = meta_file_path(app, name)?;
    let meta = read_file(&path)
      .create(true)
      .create_pretty(true)
      .create_sync(cfg!(feature = "file-sync-all"))
      .call()?;

    Ok(MetaGuard { inner: meta, _guard: guard })
  }

  pub(crate) fn write<R>(collection: &StoreCollection<R>) -> Result<()>
  where
    R: Runtime,
  {
    let mut meta = Self::read(&collection.app, &collection.name)?;
    meta.inner.path = Some(collection.path());
    meta.inner.version = current_version();

    let path = meta_file_path(&collection.app, &collection.name)?;
    write_file(path, &meta.inner)
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
      migration_history: None,
    }
  }
}

pub(crate) struct MetaGuard {
  pub(crate) inner: Meta,
  _guard: MutexGuard<'static, ()>,
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
