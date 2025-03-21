use crate::error::Result;
use crate::store::{StoreId, StoreState};
use itertools::Itertools;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use tauri_store_utils::Version as VersionTrait;

type MigrationFn = dyn Fn(&mut StoreState) -> Result<()> + Send + Sync;
type BeforeEachMigrationFn = dyn Fn(&StoreState, MigrationContext) -> Result<()> + Send + Sync;

#[derive(Default)]
pub(crate) struct Migrator {
  migrations: HashMap<StoreId, Vec<Migration>>,
  pub(crate) history: MigrationHistory,
  pub(crate) before_each: Option<Box<BeforeEachMigrationFn>>,
}

impl Migrator {
  pub(crate) fn add_migration(&mut self, id: StoreId, migration: Migration) {
    self
      .migrations
      .entry(id)
      .or_default()
      .push(migration);
  }

  pub(crate) fn add_migrations<I>(&mut self, id: StoreId, migrations: I)
  where
    I: IntoIterator<Item = Migration>,
  {
    self
      .migrations
      .entry(id)
      .or_default()
      .extend(migrations);
  }

  pub(crate) fn migrate(&mut self, id: &StoreId, state: &mut StoreState) -> Result<u32> {
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

    let mut iter = migrations.iter().peekable();
    let mut previous = None;
    let mut done = 0;

    while let Some(migration) = iter.next() {
      let current = &migration.version;
      if let Some(before_each) = &self.before_each {
        let next = iter.peek().map(|it| &it.version);
        let context = MigrationContext { id, current, previous, next };
        before_each(state, context)?;
      }

      (migration.inner)(state)?;

      self.history.set(id, current);
      previous = Some(current);
      done += 1;
    }

    Ok(done)
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
  pub fn new<V, F>(version: &V, up: F) -> Self
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
  pub current: &'a Version,
  pub previous: Option<&'a Version>,
  pub next: Option<&'a Version>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct MigrationHistory(HashMap<StoreId, Version>);

impl MigrationHistory {
  pub fn get(&self, id: &StoreId) -> Option<&Version> {
    self.0.get(id)
  }

  pub fn set(&mut self, id: &StoreId, version: &Version) {
    self.0.insert(id.clone(), version.clone());
  }
}
