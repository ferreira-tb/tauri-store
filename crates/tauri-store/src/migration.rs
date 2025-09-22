use crate::collection::CollectionMarker;
use crate::error::Result;
use crate::store::{StoreId, StoreState};
use crate::ManagerExt;
use itertools::Itertools;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_vec};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, Runtime};
use tauri_store_utils::Semver;

// We cannot use `LazyLock` because our MSRV is 1.77.2.
static LOCK: OnceLock<Mutex<()>> = OnceLock::new();

type MigrationFn = dyn Fn(&mut StoreState) -> Result<()> + Send + Sync;
type BeforeEachMigrationFn = dyn Fn(MigrationContext) + Send + Sync;

#[doc(hidden)]
#[derive(Default)]
pub struct Migrator {
  migrations: HashMap<StoreId, Vec<Migration>>,
  before_each: Option<Box<BeforeEachMigrationFn>>,
  history: MigrationHistory,
}

impl Migrator {
  pub fn add_migration(&mut self, id: StoreId, migration: Migration) {
    self
      .migrations
      .entry(id)
      .or_default()
      .push(migration);
  }

  pub fn add_migrations<I>(&mut self, id: StoreId, migrations: I)
  where
    I: IntoIterator<Item = Migration>,
  {
    self
      .migrations
      .entry(id)
      .or_default()
      .extend(migrations);
  }

  pub fn migrate<R, C>(
    &mut self,
    app: &AppHandle<R>,
    id: &StoreId,
    state: &mut StoreState,
  ) -> Result<()>
  where
    R: Runtime,
    C: CollectionMarker,
  {
    let mut migrations = self
      .migrations
      .get(id)
      .map(Vec::as_slice)
      .unwrap_or_default()
      .iter()
      .sorted()
      .collect_vec();

    if let Some(last) = self.history.get(id) {
      migrations.retain(|migration| migration.version > *last);
    }

    if migrations.is_empty() {
      return Ok(());
    }

    let mut iter = migrations.iter().peekable();
    let mut previous = None;
    let mut done = 0;
    let mut last_err = None;

    while let Some(migration) = iter.next() {
      let current = &migration.version;
      if let Some(before_each) = &self.before_each {
        let next = iter.peek().map(|it| &it.version);
        let context = MigrationContext { id, state, current, previous, next };
        before_each(context);
      }

      if let Err(err) = (migration.inner)(state) {
        last_err = Some(err);
        break;
      }

      self.history.set(id, current);
      previous = Some(current);
      done += 1;
    }

    if done > 0 {
      self.write::<R, C>(app)?;
    }

    match last_err {
      Some(err) => Err(err),
      None => Ok(()),
    }
  }

  #[doc(hidden)]
  pub fn on_before_each<F>(&mut self, f: F)
  where
    F: Fn(MigrationContext) + Send + Sync + 'static,
  {
    self.before_each = Some(Box::new(f));
  }

  pub(crate) fn read<R, C>(&mut self, app: &AppHandle<R>) -> Result<()>
  where
    R: Runtime,
    C: CollectionMarker,
  {
    let path = path::<R, C>(app);
    if let Ok(bytes) = fs::read(&path) {
      self.history = from_slice(&bytes)?;
    }

    Ok(())
  }

  fn write<R, C>(&self, app: &AppHandle<R>) -> Result<()>
  where
    R: Runtime,
    C: CollectionMarker,
  {
    let path = path::<R, C>(app);
    let lock = LOCK
      .get_or_init(Mutex::default)
      .lock()
      .expect("migrator file lock is poisoned");

    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
    }

    let bytes = to_vec(&self.history)?;
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    file.flush()?;

    if cfg!(feature = "file-sync-all") {
      file.sync_all()?;
    }

    drop(lock);

    Ok(())
  }
}

fn path<R, C>(app: &AppHandle<R>) -> PathBuf
where
  R: Runtime,
  C: CollectionMarker,
{
  app
    .store_collection_with_marker::<C>()
    .path()
    .join("migration.tauristore")
}

/// A migration step.
pub struct Migration {
  inner: Box<MigrationFn>,
  version: Version,
}

impl Migration {
  /// Creates a new migration.
  ///
  /// # Panics
  ///
  /// Panics if the version is not a valid [semver](https://semver.org/).
  #[allow(clippy::needless_pass_by_value)]
  pub fn new<F>(version: impl Semver, up: F) -> Self
  where
    F: Fn(&mut StoreState) -> Result<()> + Send + Sync + 'static,
  {
    Self {
      inner: Box::new(up),
      version: version.semver(),
    }
  }

  /// Version of the migration.
  pub fn version(&self) -> &Version {
    &self.version
  }
}

impl PartialOrd for Migration {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Migration {
  fn cmp(&self, other: &Self) -> Ordering {
    self.version.cmp(&other.version)
  }
}

impl PartialEq for Migration {
  fn eq(&self, other: &Self) -> bool {
    self.version == other.version
  }
}

impl Eq for Migration {}

/// Context for a migration step.
#[derive(Debug)]
pub struct MigrationContext<'a> {
  pub id: &'a StoreId,
  pub state: &'a StoreState,
  pub current: &'a Version,
  pub previous: Option<&'a Version>,
  pub next: Option<&'a Version>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct MigrationHistory(HashMap<StoreId, Version>);

impl MigrationHistory {
  pub fn get(&self, id: &StoreId) -> Option<&Version> {
    self.0.get(id)
  }

  pub fn set(&mut self, id: &StoreId, version: &Version) {
    self.0.insert(id.clone(), version.clone());
  }
}
