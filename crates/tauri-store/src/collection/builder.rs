use super::{OnLoadFn, StoreCollection, RESOURCE_ID};
use crate::error::Result;

use crate::store::{SaveStrategy, Store, StoreId};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tauri::{Manager, Runtime};

#[cfg(feature = "plugin")]
use tauri::plugin::TauriPlugin;

/// Builder for the [`StoreCollection`](crate::collection::StoreCollection).
pub struct StoreCollectionBuilder<R: Runtime> {
  path: Option<PathBuf>,
  default_save_strategy: SaveStrategy,
  autosave: Option<Duration>,
  on_load: Option<Box<OnLoadFn<R>>>,
  pretty: bool,
  save_denylist: Option<HashSet<StoreId>>,
  sync_denylist: Option<HashSet<StoreId>>,
}

impl<R: Runtime> StoreCollectionBuilder<R> {
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
    F: Fn(&Store<R>) -> Result<()> + Send + Sync + 'static,
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
  pub fn save_denylist(mut self, save_denylist: HashSet<StoreId>) -> Self {
    self.save_denylist = Some(save_denylist);
    self
  }

  /// Sets a list of stores that should not be synchronized across windows.
  #[must_use]
  pub fn sync_denylist(mut self, sync_denylist: HashSet<StoreId>) -> Self {
    self.sync_denylist = Some(sync_denylist);
    self
  }

  /// Builds the [`StoreCollection`](crate::collection::StoreCollection).
  ///
  /// # Panics
  ///
  /// Panics if a store collection is already initialized.
  pub fn build<M>(self, app: &M) -> Result<Arc<StoreCollection<R>>>
  where
    M: Manager<R>,
  {
    private::build(self, app)
  }

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

mod private {
  use super::RESOURCE_ID;
  use crate::collection::autosave::Autosave;
  use crate::collection::{StoreCollection, StoreCollectionBuilder};
  use crate::error::Result;
  use crate::meta;
  use dashmap::DashMap;
  use std::sync::{Arc, Mutex};
  use tauri::{Manager, Runtime};

  #[cfg(tauri_store_tracing)]
  use tracing::trace;

  type Builder<R> = StoreCollectionBuilder<R>;

  pub(super) fn build<R, M>(mut builder: Builder<R>, app: &M) -> Result<Arc<StoreCollection<R>>>
  where
    R: Runtime,
    M: Manager<R>,
  {
    assert!(
      RESOURCE_ID.get().is_none(),
      "store collection is already initialized"
    );

    let path = builder.path.take().unwrap_or_else(|| {
      app
        .path()
        .app_data_dir()
        .expect("failed to resolve app data dir")
        .join(env!("CARGO_PKG_NAME"))
    });

    builder.save_denylist = builder.save_denylist.filter(|it| !it.is_empty());
    builder.sync_denylist = builder.sync_denylist.filter(|it| !it.is_empty());

    let autosave = Autosave::new(builder.autosave);

    let app = app.app_handle();
    let collection = Arc::new(StoreCollection::<R> {
      app: app.clone(),
      path: Mutex::new(path),
      stores: DashMap::new(),
      on_load: builder.on_load,
      autosave: Mutex::new(autosave),
      default_save_strategy: builder.default_save_strategy,
      save_denylist: builder.save_denylist,
      sync_denylist: builder.sync_denylist,
      pretty: builder.pretty,
    });

    #[cfg(tauri_store_tracing)]
    trace!(?collection);

    let rid = app
      .resources_table()
      .add_arc(Arc::clone(&collection));

    let _ = RESOURCE_ID.set(rid);

    meta::load(&collection)?;
    collection.autosave.lock().unwrap().start(app);

    Ok(collection)
  }
}

impl<R: Runtime> Default for StoreCollectionBuilder<R> {
  fn default() -> Self {
    Self {
      path: None,
      default_save_strategy: SaveStrategy::Immediate,
      autosave: None,
      on_load: None,
      pretty: false,
      save_denylist: None,
      sync_denylist: None,
    }
  }
}
