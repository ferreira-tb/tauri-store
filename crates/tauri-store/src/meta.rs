use crate::collection::{CollectionMarker, StoreCollection};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
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

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Meta {
  pub path: Option<PathBuf>,

  #[cfg(feature = "unstable-migration")]
  pub migration_history: Option<MigrationHistory>,
}

impl Meta {
  #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
  pub(crate) fn read<R, M>(app: &M, name: &str) -> Result<MetaGuard>
  where
    R: Runtime,
    M: Manager<R>,
  {
    read(&meta_file_path(app, name)?)
  }

  pub(crate) fn write<R, C>(collection: &StoreCollection<R, C>) -> Result<()>
  where
    R: Runtime,
    C: CollectionMarker,
  {
    let app = collection.handle.app();
    let path = meta_file_path(app, &collection.name)?;
    let mut meta = read(&path)?;
    meta.inner.path = Some(collection.path());

    #[cfg(feature = "unstable-migration")]
    {
      let history = migration_history(collection);
      meta.inner.migration_history = Some(history);
    }

    write_file(path, &meta.inner)
      .pretty(true)
      .sync(cfg!(feature = "file-sync-all"))
      .call()?;

    Ok(())
  }
}

pub(crate) struct MetaGuard {
  pub(crate) inner: Meta,
  _guard: MutexGuard<'static, ()>,
}

fn read(path: &Path) -> Result<MetaGuard> {
  let guard = LOCK
    .get_or_init(Mutex::default)
    .lock()
    .expect("meta lock is poisoned");

  let meta = read_file(path)
    .create(true)
    .create_pretty(true)
    .create_sync(cfg!(feature = "file-sync-all"))
    .call()?;

  Ok(MetaGuard { inner: meta, _guard: guard })
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

#[cfg(feature = "unstable-migration")]
fn migration_history<R, C>(collection: &StoreCollection<R, C>) -> MigrationHistory
where
  R: Runtime,
  C: CollectionMarker,
{
  collection
    .migrator
    .lock()
    .unwrap()
    .history
    .clone()
}
