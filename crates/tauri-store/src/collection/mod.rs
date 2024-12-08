mod autosave;

#[cfg(feature = "unstable-async")]
mod unstable_async;

use crate::error::Result;
use crate::store::{SaveStrategy, Store, StoreResource};
use autosave::Autosave;
use dashmap::DashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use tauri::{AppHandle, Manager, Resource, ResourceId, Runtime};

#[cfg(not(feature = "unstable-async"))]
use {
  crate::event::{emit_all, STORE_UNLOADED_EVENT},
  crate::io_err,
  crate::store::StoreState,
  serde::de::DeserializeOwned,
  serde_json::Value as Json,
};

#[cfg(feature = "unstable-async")]
use futures::future::BoxFuture;

#[cfg(feature = "ahash")]
use ahash::HashSet;
#[cfg(not(feature = "ahash"))]
use std::collections::HashSet;

#[cfg(tauri_store_tracing)]
use tracing::trace;

pub(crate) static RESOURCE_ID: OnceLock<ResourceId> = OnceLock::new();

#[cfg(not(feature = "unstable-async"))]
pub type OnLoadResult = Result<()>;
#[cfg(feature = "unstable-async")]
pub type OnLoadResult = BoxFuture<'static, Result<()>>;

pub type OnLoadFn<R> = dyn Fn(&Store<R>) -> OnLoadResult + Send + Sync;

pub struct StoreCollection<R: Runtime> {
  pub(crate) app: AppHandle<R>,
  pub(crate) path: PathBuf,
  pub(crate) stores: DashMap<String, ResourceId>,
  pub(crate) on_load: Option<Box<OnLoadFn<R>>>,
  pub(crate) autosave: Mutex<Autosave>,
  pub(crate) save_strategy: SaveStrategy,
  pub(crate) save_denylist: Option<HashSet<String>>,
  pub(crate) sync_denylist: Option<HashSet<String>>,
  pub(crate) pretty: bool,
}

impl<R: Runtime> StoreCollection<R> {
  pub fn builder() -> StoreCollectionBuilder<R> {
    StoreCollectionBuilder::new()
  }

  /// Directory where the stores are saved.
  pub fn path(&self) -> &Path {
    &self.path
  }

  /// Gets the resource id for a store.
  fn rid(&self, store_id: &str) -> Option<ResourceId> {
    self.stores.get(store_id).map(|it| *it.value())
  }

  /// Gets the resource ids for all the stores.
  fn rids(&self) -> Vec<ResourceId> {
    self.stores.iter().map(|it| *it.value()).collect()
  }

  /// Lists all the store ids.
  pub fn ids(&self) -> Vec<String> {
    self
      .stores
      .iter()
      .map(|it| it.key().clone())
      .collect()
  }

  /// Current save strategy for the stores.
  pub fn save_strategy(&self) -> SaveStrategy {
    self.save_strategy
  }

  /// Saves the stores periodically.
  pub fn set_autosave(&self, duration: Duration) {
    if let Ok(mut autosave) = self.autosave.lock() {
      autosave.set_duration(duration);
      autosave.start(&self.app);
    }
  }

  /// Stops the autosave.
  pub fn clear_autosave(&self) {
    if let Ok(mut autosave) = self.autosave.lock() {
      autosave.stop();
    }
  }
}

#[cfg(not(feature = "unstable-async"))]
impl<R: Runtime> StoreCollection<R> {
  pub(crate) fn get_resource(&self, id: impl AsRef<str>) -> Result<Arc<StoreResource<R>>> {
    let id = id.as_ref();
    let rid = if let Some(rid) = self.rid(id) {
      rid
    } else {
      let (rid, resource) = Store::load(&self.app, id)?;
      if let Some(on_load) = &self.on_load {
        let store = resource.inner.lock().unwrap();
        on_load(&store)?;
      }

      self.stores.insert(id.to_owned(), rid);
      rid
    };

    StoreResource::get(&self.app, rid)
  }

  /// Calls a closure with a mutable reference to the store with the given id.
  pub fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> T,
  {
    let resource = self.get_resource(id)?;
    let mut store = resource.inner.lock().unwrap();
    Ok(f(&mut *store))
  }

  /// Saves a store to the disk.
  pub fn save(&self, id: impl AsRef<str>) -> Result<()> {
    let resource = self.get_resource(id)?;
    let store = resource.inner.lock().unwrap();
    store.save()
  }

  /// Saves some stores to the disk.
  pub fn save_some(&self, ids: &[impl AsRef<str>]) -> Result<()> {
    for id in ids {
      self.save(id)?;
    }

    Ok(())
  }

  /// Saves all the stores to the disk.
  pub fn save_all(&self) -> Result<()> {
    for rid in self.rids() {
      StoreResource::get(&self.app, rid)?
        .inner
        .lock()
        .unwrap()
        .save()?;
    }

    Ok(())
  }

  /// Gets a clone of the store state.
  pub fn store_state(&self, store_id: impl AsRef<str>) -> Option<StoreState> {
    let resource = self.get_resource(store_id).ok()?;
    let store = resource.inner.lock().unwrap();
    Some(store.state().clone())
  }

  /// Gets the store state, then tries to parse it as an instance of type `T`.
  pub fn try_store_state<T>(&self, store_id: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let resource = self.get_resource(store_id)?;
    let store = resource.inner.lock().unwrap();
    store.try_state()
  }

  /// Gets a value from a store.
  pub fn get(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<Json> {
    let resource = self.get_resource(store_id).ok()?;
    let store = resource.inner.lock().unwrap();
    store.get(key).cloned()
  }

  /// Gets a value from a store and tries to parse it as an instance of type `T`.
  pub fn try_get<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let key = key.as_ref();
    let Some(value) = self.get(store_id, key) else {
      return io_err!(NotFound, "key not found: {key}");
    };

    Ok(serde_json::from_value(value)?)
  }

  /// Sets a key-value pair in a store.
  pub fn set(&self, store_id: impl AsRef<str>, key: impl AsRef<str>, value: Json) -> Result<()> {
    let resource = self.get_resource(store_id)?;
    let mut store = resource.inner.lock().unwrap();
    store.set(key, value)
  }

  /// Patches a store state.
  pub fn patch(&self, store_id: impl AsRef<str>, state: StoreState) -> Result<()> {
    let resource = self.get_resource(store_id)?;
    let mut store = resource.inner.lock().unwrap();
    store.patch(state)
  }

  /// Watches a store for changes.
  pub fn watch<F>(&self, store_id: impl AsRef<str>, f: F) -> Result<u32>
  where
    F: Fn(AppHandle<R>) -> Result<()> + Send + Sync + 'static,
  {
    let resource = self.get_resource(store_id)?;
    let mut store = resource.inner.lock().unwrap();
    Ok(store.watch(f))
  }

  /// Removes a listener from a store.
  pub fn unwatch(&self, store_id: impl AsRef<str>, listener_id: u32) -> Result<bool> {
    let resource = self.get_resource(store_id)?;
    let mut store = resource.inner.lock().unwrap();
    Ok(store.unwatch(listener_id))
  }

  pub fn unload_store(&self, id: &str) -> Result<()> {
    if let Some((_, rid)) = self.stores.remove(id) {
      // The store needs to be saved immediately here.
      // Otherwise, the plugin might try to load it again if `StoreCollection::get_resource` is called.
      // This scenario will happen whenever the save strategy is not `Immediate`.
      StoreResource::take(&self.app, rid)?
        .inner
        .lock()
        .unwrap()
        .save_now()?;

      emit_all(&self.app, STORE_UNLOADED_EVENT, id)?;
    }

    Ok(())
  }
}

impl<R: Runtime> Resource for StoreCollection<R> {}

impl<R: Runtime> fmt::Debug for StoreCollection<R> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("StoreCollection")
      .field("path", &self.path)
      .field("on_load", &self.on_load.is_some())
      .field("save_strategy", &self.save_strategy)
      .field(
        "save_denylist",
        &self
          .save_denylist
          .as_ref()
          .map(HashSet::len)
          .unwrap_or(0),
      )
      .field(
        "sync_denylist",
        &self
          .sync_denylist
          .as_ref()
          .map(HashSet::len)
          .unwrap_or(0),
      )
      .field("pretty", &self.pretty)
      .finish_non_exhaustive()
  }
}

impl<R: Runtime> Drop for StoreCollection<R> {
  fn drop(&mut self) {
    self.clear_autosave();
  }
}

pub struct StoreCollectionBuilder<R: Runtime> {
  path: Option<PathBuf>,
  save_strategy: SaveStrategy,
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
  pub fn save_strategy(mut self, strategy: SaveStrategy) -> Self {
    self.save_strategy = strategy;
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
      save_strategy: self.save_strategy,
      autosave: Mutex::new(autosave),
      on_load: self.on_load,
      pretty: self.pretty,
      save_denylist: self.save_denylist,
      sync_denylist: self.sync_denylist,
    });

    let rid = app
      .resources_table()
      .add_arc(Arc::clone(&collection));

    let _ = RESOURCE_ID.set(rid);

    collection.autosave.lock().unwrap().start(app);

    #[cfg(tauri_store_tracing)]
    trace!(?collection);

    collection
  }
}

impl<R: Runtime> Default for StoreCollectionBuilder<R> {
  fn default() -> Self {
    Self {
      path: None,
      save_strategy: SaveStrategy::Immediate,
      autosave: None,
      on_load: None,
      pretty: false,
      save_denylist: None,
      sync_denylist: None,
    }
  }
}
