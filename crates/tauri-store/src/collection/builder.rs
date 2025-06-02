use super::marker::CollectionMarker;
use super::{DefaultMarker, OnLoadFn, StoreCollection};
use crate::collection::autosave::Autosave;
use crate::error::Result;
use crate::meta::Meta;
use crate::store::{SaveStrategy, Store, StoreId};
use crate::ManagerExt;
use dashmap::{DashMap, DashSet};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;
use tauri::{Manager, Runtime};

#[cfg(feature = "plugin")]
use tauri::plugin::TauriPlugin;

#[cfg(feature = "unstable-migration")]
use crate::migration::{Migration, MigrationContext, Migrator};

/// Builder for the [`StoreCollection`](crate::collection::StoreCollection).
pub struct StoreCollectionBuilder<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
  path: Option<PathBuf>,
  default_save_strategy: SaveStrategy,
  autosave: Option<Duration>,
  on_load: Option<Box<OnLoadFn<R, C>>>,
  pretty: bool,
  save_denylist: DashSet<StoreId>,
  sync_denylist: DashSet<StoreId>,

  #[cfg(feature = "unstable-migration")]
  migrator: Migrator,
}

impl<R, C> StoreCollectionBuilder<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
  /// Creates a new builder instance with default values.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sets the autosave interval for all stores.
  #[must_use]
  pub fn autosave(mut self, duration: Duration) -> Self {
    self.autosave = Some(duration);
    self
  }

  /// Sets the default save strategy to be used by the stores.
  #[must_use]
  pub fn default_save_strategy(mut self, strategy: SaveStrategy) -> Self {
    self.default_save_strategy = strategy;
    self
  }

  /// Registers a closure to be called when a store is loaded.
  #[must_use]
  pub fn on_load<F>(mut self, f: F) -> Self
  where
    F: Fn(&Store<R, C>) -> Result<()> + Send + Sync + 'static,
  {
    self.on_load = Some(Box::new(f));
    self
  }

  /// Directory where the stores will be saved.
  #[must_use]
  pub fn path(mut self, path: impl AsRef<Path>) -> Self {
    self.path = Some(path.as_ref().to_path_buf());
    self
  }

  /// Sets whether the store files should be pretty printed.
  #[must_use]
  pub fn pretty(mut self, yes: bool) -> Self {
    self.pretty = yes;
    self
  }

  /// Sets a list of stores that should not be saved to disk.
  #[must_use]
  pub fn save_denylist<I, T>(mut self, denylist: I) -> Self
  where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
  {
    self.save_denylist.extend(
      denylist
        .into_iter()
        .map(|it| StoreId::from(it.as_ref())),
    );

    self
  }

  /// Sets a list of stores that should not be synchronized across windows.
  #[must_use]
  pub fn sync_denylist<I, T>(mut self, denylist: I) -> Self
  where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
  {
    self.sync_denylist.extend(
      denylist
        .into_iter()
        .map(|it| StoreId::from(it.as_ref())),
    );

    self
  }

  #[must_use]
  #[doc(hidden)]
  #[cfg(feature = "unstable-migration")]
  pub fn migrator(mut self, migrator: Migrator) -> Self {
    self.migrator = migrator;
    self
  }

  /// Defines a migration for a store.
  #[must_use]
  #[cfg(feature = "unstable-migration")]
  pub fn migration(mut self, id: impl Into<StoreId>, migration: Migration) -> Self {
    self.migrator.add_migration(id.into(), migration);
    self
  }

  /// Defines multiple migrations for a store.
  #[must_use]
  #[cfg(feature = "unstable-migration")]
  pub fn migrations<I>(mut self, id: impl Into<StoreId>, migrations: I) -> Self
  where
    I: IntoIterator<Item = Migration>,
  {
    self
      .migrator
      .add_migrations(id.into(), migrations);

    self
  }

  /// Sets a closure to be called before each migration step.
  #[must_use]
  #[cfg(feature = "unstable-migration")]
  pub fn on_before_each_migration<F>(mut self, f: F) -> Self
  where
    F: Fn(MigrationContext) + Send + Sync + 'static,
  {
    self.migrator.on_before_each(f);
    self
  }

  /// Builds the [`StoreCollection`](crate::collection::StoreCollection).
  ///
  /// # Panics
  ///
  /// Panics if a store collection is already initialized.
  #[doc(hidden)]
  pub fn build<M>(mut self, app: &M, plugin_name: &str) -> Result<()>
  where
    M: Manager<R>,
  {
    let app = app.app_handle();
    debug_assert!(
      app.try_state::<StoreCollection<R, C>>().is_none(),
      "store collection is already initialized"
    );

    let meta = Meta::read(app, plugin_name)?;
    let path = meta
      .inner
      .path
      .or_else(|| self.path.take())
      .unwrap_or_else(|| {
        app
          .path()
          .app_data_dir()
          .expect("failed to resolve app data dir")
          .join(plugin_name)
      });

    #[cfg(feature = "unstable-migration")]
    if let Some(history) = meta.inner.migration_history {
      self.migrator.history = history;
    }

    app.manage(StoreCollection::<R, C> {
      app: app.clone(),
      name: Box::from(plugin_name),
      path: Mutex::new(path),
      stores: DashMap::new(),
      on_load: self.on_load,
      autosave: Mutex::new(Autosave::new(self.autosave)),
      default_save_strategy: self.default_save_strategy,
      save_denylist: self.save_denylist,
      sync_denylist: self.sync_denylist,
      pretty: self.pretty,
      phantom: PhantomData,

      #[cfg(feature = "unstable-migration")]
      migrator: Mutex::new(self.migrator),
    });

    app
      .store_collection_with_marker::<C>()
      .autosave
      .lock()
      .unwrap()
      .start::<R, C>(app);

    Ok(())
  }
}

impl<R> StoreCollectionBuilder<R, DefaultMarker>
where
  R: Runtime,
{
  /// Initializes the plugin with a [`StoreCollection`](crate::collection::StoreCollection).
  ///
  /// # Panics
  ///
  /// Panics if a store collection is already initialized.
  #[cfg(feature = "plugin")]
  pub fn build_plugin(self) -> TauriPlugin<R> {
    crate::plugin::build(self)
  }
}

impl<R, C> Default for StoreCollectionBuilder<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
  fn default() -> Self {
    Self {
      path: None,
      default_save_strategy: SaveStrategy::Immediate,
      autosave: None,
      on_load: None,
      pretty: false,
      save_denylist: DashSet::new(),
      sync_denylist: DashSet::new(),

      #[cfg(feature = "unstable-migration")]
      migrator: Migrator::default(),
    }
  }
}
