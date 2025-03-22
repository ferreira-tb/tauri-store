use crate::error::{Error, Result};
use crate::store::{StoreId, StoreState};
use itertools::Itertools;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use tauri_store_utils::Version as VersionTrait;

#[cfg(tauri_store_tracing)]
use tracing::debug;

type MigrationFn = dyn Fn(&mut StoreState) -> Result<()> + Send + Sync;
type BeforeEachMigrationFn = dyn Fn(MigrationContext) + Send + Sync;

#[doc(hidden)]
#[derive(Default)]
pub struct Migrator {
  migrations: HashMap<StoreId, Vec<Migration>>,
  before_each: Option<Box<BeforeEachMigrationFn>>,
  pub(crate) history: MigrationHistory,
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

  pub fn migrate(&mut self, id: &StoreId, state: &mut StoreState) -> MigrationResult {
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

    #[cfg(tauri_store_tracing)]
    debug!("{} pending migration(s) for {}", migrations.len(), id);

    if migrations.is_empty() {
      return MigrationResult::new(0);
    }

    let mut iter = migrations.iter().peekable();
    let mut previous = None;
    let mut done = 0;

    while let Some(migration) = iter.next() {
      let current = &migration.version;
      if let Some(before_each) = &self.before_each {
        let next = iter.peek().map(|it| &it.version);
        let context = MigrationContext { id, state, current, previous, next };

        #[cfg(tauri_store_tracing)]
        debug!(before_each_migration = ?context);

        before_each(context);
      }

      if let Err(err) = (migration.inner)(state) {
        return MigrationResult::with_error(done, err);
      }

      self.history.set(id, current);
      previous = Some(current);
      done += 1;

      #[cfg(tauri_store_tracing)]
      debug!("migration {current} done for {id}");
    }

    MigrationResult::new(done)
  }

  #[doc(hidden)]
  pub fn on_before_each<F>(&mut self, f: F)
  where
    F: Fn(MigrationContext) + Send + Sync + 'static,
  {
    self.before_each = Some(Box::new(f));
  }
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
  pub fn new<V, F>(version: V, up: F) -> Self
  where
    V: VersionTrait,
    F: Fn(&mut StoreState) -> Result<()> + Send + Sync + 'static,
  {
    Self {
      inner: Box::new(up),
      version: version.version(),
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

// This way the meta can be updated even if some migrations fail.
#[doc(hidden)]
pub struct MigrationResult {
  pub(crate) done: u32,
  pub(crate) error: Option<Error>,
}

impl MigrationResult {
  pub const fn new(done: u32) -> Self {
    Self { done, error: None }
  }

  pub const fn with_error(done: u32, error: Error) -> Self {
    Self { done, error: Some(error) }
  }
}
