use super::autosave::Autosave;
use super::{OnLoadFn, OnLoadResult, StoreCollection, RESOURCE_ID};
use crate::store::{SaveStrategy, Store};
use dashmap::DashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Manager, Runtime};

#[cfg(feature = "ahash")]
use ahash::HashSet;
#[cfg(not(feature = "ahash"))]
use std::collections::HashSet;

#[cfg(tauri_store_tracing)]
use tracing::trace;

pub struct StoreCollectionBuilder<R: Runtime> {
  path: Option<PathBuf>,
  default_save_strategy: SaveStrategy,
  autosave: Option<Duration>,
  on_load: Option<Box<OnLoadFn<R>>>,
  pretty: bool,
  save_denylist: Option<HashSet<String>>,
  sync_denylist: Option<HashSet<String>>,
}

impl<R: Runtime> StoreCollectionBuilder<R> {
  pub fn new() -> Self {
    Self::default()
  }

  #[must_use]
  pub fn path(mut self, path: impl AsRef<Path>) -> Self {
    self.path = Some(path.as_ref().to_path_buf());
    self
  }

  #[must_use]
  pub fn default_save_strategy(mut self, strategy: SaveStrategy) -> Self {
    self.default_save_strategy = strategy;
    self
  }

  #[must_use]
  pub fn autosave(mut self, duration: Duration) -> Self {
    self.autosave = Some(duration);
    self
  }

  #[must_use]
  pub fn on_load<F>(mut self, f: F) -> Self
  where
    F: Fn(&Store<R>) -> OnLoadResult + Send + Sync + 'static,
  {
    self.on_load = Some(Box::new(f));
    self
  }

  #[must_use]
  pub fn pretty(mut self, yes: bool) -> Self {
    self.pretty = yes;
    self
  }

  #[must_use]
  pub fn save_denylist(mut self, save_denylist: HashSet<String>) -> Self {
    self.save_denylist = Some(save_denylist);
    self
  }

  #[must_use]
  pub fn sync_denylist(mut self, sync_denylist: HashSet<String>) -> Self {
    self.sync_denylist = Some(sync_denylist);
    self
  }

  pub fn build(mut self, app: &AppHandle<R>) -> Arc<StoreCollection<R>> {
    let path = self.path.take().unwrap_or_else(|| {
      app
        .path()
        .app_data_dir()
        .expect("failed to resolve app data dir")
        .join("tauri-store")
    });

    self.save_denylist = self.save_denylist.filter(|it| !it.is_empty());
    self.sync_denylist = self.sync_denylist.filter(|it| !it.is_empty());

    let autosave = Autosave::new(self.autosave);

    let collection = Arc::new(StoreCollection::<R> {
      app: app.clone(),
      path,
      stores: DashMap::new(),
      default_save_strategy: self.default_save_strategy,
      autosave: Mutex::new(autosave),
      on_load: self.on_load,
      pretty: self.pretty,
      save_denylist: self.save_denylist,
      sync_denylist: self.sync_denylist,
    });

    #[cfg(tauri_store_tracing)]
    trace!(?collection);

    let rid = app
      .resources_table()
      .add_arc(Arc::clone(&collection));

    let _ = RESOURCE_ID.set(rid);

    collection.autosave.lock().unwrap().start(app);

    collection
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
