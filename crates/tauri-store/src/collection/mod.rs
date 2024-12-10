mod autosave;
mod builder;

#[cfg(feature = "unstable-async")]
mod unstable_async;

use crate::error::Result;
use crate::store::{SaveStrategy, Store, StoreResource};
use autosave::Autosave;
use dashmap::DashMap;
use std::collections::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use tauri::{AppHandle, Resource, ResourceId, Runtime};

pub use builder::StoreCollectionBuilder;

#[cfg(not(feature = "unstable-async"))]
use {
  crate::event::{emit_all, STORE_UNLOADED_EVENT},
  crate::io_err,
  crate::store::StoreState,
  serde::de::DeserializeOwned,
  serde_json::Value as Json,
  std::sync::Arc,
};

#[cfg(feature = "unstable-async")]
use futures::future::BoxFuture;

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
  pub(crate) default_save_strategy: SaveStrategy,
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

  /// Default save strategy for the stores.
  /// This can be overridden on a per-store basis.
  pub fn default_save_strategy(&self) -> SaveStrategy {
    self.default_save_strategy
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

  /// Saves a store to the disk immediately, ignoring the save strategy.
  pub fn save_now(&self, id: impl AsRef<str>) -> Result<()> {
    let resource = self.get_resource(id)?;
    let store = resource.inner.lock().unwrap();
    store.abort_pending_save();
    store.save_now()
  }

  /// Saves some stores to the disk.
  pub fn save_some(&self, ids: &[impl AsRef<str>]) -> Result<()> {
    ids.iter().try_for_each(|id| self.save(id))
  }

  /// Saves some stores to the disk immediately, ignoring the save strategy.
  pub fn save_some_now(&self, ids: &[impl AsRef<str>]) -> Result<()> {
    ids.iter().try_for_each(|id| self.save_now(id))
  }

  /// Saves all the stores to the disk.
  pub fn save_all(&self) -> Result<()> {
    // I suppose going through the rids should be better than through the store ids.
    // This way, we don't need to hold references into the dashmap nor clone its keys.
    // The downside (?) is that we need to use the StoreResource directly.
    self
      .rids()
      .into_iter()
      .try_for_each(|rid| StoreResource::save(&self.app, rid))
  }

  /// Saves all the stores to the disk immediately, ignoring the save strategy.
  pub fn save_all_now(&self) -> Result<()> {
    self
      .rids()
      .into_iter()
      .try_for_each(|rid| StoreResource::save_now(&self.app, rid))
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

  /// Removes a watcher from a store.
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

impl<R: Runtime> Drop for StoreCollection<R> {
  fn drop(&mut self) {
    self.clear_autosave();
  }
}

impl<R: Runtime> Resource for StoreCollection<R> {}

impl<R: Runtime> fmt::Debug for StoreCollection<R> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("StoreCollection")
      .field("path", &self.path)
      .field("on_load", &self.on_load.is_some())
      .field("default_save_strategy", &self.default_save_strategy)
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
