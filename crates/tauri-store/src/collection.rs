mod autosave;
mod builder;
mod path;

use crate::error::Result;
use crate::event::{emit, STORE_UNLOAD_EVENT};
use crate::store::{SaveStrategy, Store, StoreId, StoreResource, StoreState, WatcherId};
use autosave::Autosave;
use dashmap::DashMap;
use path::set_path;
use serde::de::DeserializeOwned;
use serde_json::Value as Json;
use std::collections::HashSet;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use tauri::{AppHandle, Resource, ResourceId, Runtime};

pub use builder::StoreCollectionBuilder;

pub(crate) static RESOURCE_ID: OnceLock<ResourceId> = OnceLock::new();

/// Closure to be called when a store is loaded.
pub type OnLoadFn<R> = dyn Fn(&Store<R>) -> Result<()> + Send + Sync;

/// A collection of stores.
/// This is the core component for store plugins.
pub struct StoreCollection<R: Runtime> {
  pub(crate) app: AppHandle<R>,
  pub(crate) path: Mutex<PathBuf>,
  pub(crate) stores: DashMap<StoreId, ResourceId>,
  pub(crate) on_load: Option<Box<OnLoadFn<R>>>,
  pub(crate) autosave: Mutex<Autosave>,
  pub(crate) default_save_strategy: SaveStrategy,
  pub(crate) save_denylist: Option<HashSet<StoreId>>,
  pub(crate) sync_denylist: Option<HashSet<StoreId>>,
  pub(crate) pretty: bool,
}

impl<R: Runtime> StoreCollection<R> {
  /// Builds a new store collection.
  pub fn builder() -> StoreCollectionBuilder<R> {
    StoreCollectionBuilder::new()
  }

  pub(crate) fn get_resource(&self, id: impl AsRef<str>) -> Result<Arc<StoreResource<R>>> {
    let id = StoreId::from(id.as_ref());
    let rid = match self.rid(&id) {
      Some(rid) => rid,
      None => {
        let (rid, resource) = Store::load(&self.app, &id)?;
        if let Some(on_load) = &self.on_load {
          resource.locked(|store| on_load(store))?;
        }

        self.stores.insert(id, rid);
        rid
      }
    };

    StoreResource::get(&self.app, rid)
  }

  /// Gets the resource id for a store.
  fn rid(&self, store_id: &StoreId) -> Option<ResourceId> {
    self.stores.get(store_id).map(|it| *it.value())
  }

  /// Gets the resource ids for all the stores.
  fn rids(&self) -> Vec<ResourceId> {
    self.stores.iter().map(|it| *it.value()).collect()
  }

  /// Lists all the store ids.
  pub fn ids(&self) -> Vec<StoreId> {
    self
      .stores
      .iter()
      .map(|it| it.key().clone())
      .collect()
  }

  /// Directory where the stores are saved.
  pub fn path(&self) -> PathBuf {
    self.path.lock().unwrap().clone()
  }

  /// Sets the directory where the stores are saved.
  /// This will move all *currently active* stores to the new directory.
  pub fn set_path(&self, path: impl AsRef<Path>) -> Result<()> {
    set_path(self, path)
  }

  /// Calls a closure with a mutable reference to the store with the given id.
  pub fn with_store<F, T>(&self, store_id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> T,
  {
    Ok(self.get_resource(store_id)?.locked(f))
  }

  /// Gets a clone of the store state.
  pub fn state(&self, store_id: impl AsRef<str>) -> Result<StoreState> {
    self
      .get_resource(store_id)?
      .locked(|store| Ok(store.state().clone()))
  }

  /// Gets the store state, then tries to parse it as an instance of type `T`.
  pub fn try_state<T>(&self, store_id: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    self
      .get_resource(store_id)?
      .locked(|store| store.try_state())
  }

  /// Gets a value from a store.
  pub fn get(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<Json> {
    self
      .get_resource(store_id)
      .ok()?
      .locked(|store| store.get(key).cloned())
  }

  /// Gets a value from a store and tries to parse it as an instance of type `T`.
  pub fn try_get<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    self
      .get_resource(store_id)?
      .locked(|store| store.try_get(key))
  }

  /// Gets a value from a store and tries to parse it as an instance of type `T`.
  ///
  /// If the key does not exist, returns the provided default value.
  pub fn try_get_or<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>, default: T) -> T
  where
    T: DeserializeOwned,
  {
    self.try_get(store_id, key).unwrap_or(default)
  }

  /// Gets a value from a store and tries to parse it as an instance of type `T`.
  ///
  /// If the key does not exist, returns the default value of `T`.
  pub fn try_get_or_default<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> T
  where
    T: Default + DeserializeOwned,
  {
    self.try_get(store_id, key).unwrap_or_default()
  }

  /// Gets a value from a store and tries to parse it as an instance of type `T`.
  ///
  /// If the key does not exist, returns the result of the provided closure.
  pub fn try_get_or_else<T>(
    &self,
    store_id: impl AsRef<str>,
    key: impl AsRef<str>,
    f: impl FnOnce() -> T,
  ) -> T
  where
    T: DeserializeOwned,
  {
    self
      .try_get(store_id, key)
      .unwrap_or_else(|_| f())
  }

  /// Sets a key-value pair in a store.
  pub fn set<K, V>(&self, store_id: impl AsRef<str>, key: K, value: V) -> Result<()>
  where
    K: AsRef<str>,
    V: Into<Json>,
  {
    self
      .get_resource(store_id)?
      .locked(|store| store.set(key, value))
  }

  /// Patches a store state.
  pub fn patch<S>(&self, store_id: impl AsRef<str>, state: S) -> Result<()>
  where
    S: Into<StoreState>,
  {
    self
      .get_resource(store_id)?
      .locked(|store| store.patch(state))
  }

  /// Saves a store to the disk.
  pub fn save(&self, store_id: impl AsRef<str>) -> Result<()> {
    self
      .get_resource(store_id)?
      .locked(|store| store.save())
  }

  /// Saves a store to the disk immediately, ignoring the save strategy.
  pub fn save_now(&self, store_id: impl AsRef<str>) -> Result<()> {
    self.get_resource(store_id)?.locked(|store| {
      store.abort_pending_save();
      store.save_now()
    })
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
    // I suppose going through the rids is better than through the store ids.
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

  /// Watches a store for changes.
  pub fn watch<F>(&self, store_id: impl AsRef<str>, f: F) -> Result<WatcherId>
  where
    F: Fn(AppHandle<R>) -> Result<()> + Send + Sync + 'static,
  {
    self
      .get_resource(store_id)?
      .locked(|store| Ok(store.watch(f)))
  }

  /// Removes a watcher from a store.
  pub fn unwatch(
    &self,
    store_id: impl AsRef<str>,
    watcher_id: impl Into<WatcherId>,
  ) -> Result<bool> {
    self
      .get_resource(store_id)?
      .locked(|store| Ok(store.unwatch(watcher_id)))
  }

  /// Removes the store from the collection.
  #[doc(hidden)]
  pub fn unload_store(&self, id: &StoreId) -> Result<()> {
    if let Some((_, rid)) = self.stores.remove(id) {
      // The store needs to be saved immediately here.
      // Otherwise, the plugin might try to load it again if `StoreCollection::get_resource` is called.
      // This scenario will happen whenever the save strategy is not `Immediate`.
      let resource = StoreResource::take(&self.app, rid)?;
      resource.locked(|store| store.save_now())?;

      emit(&self.app, STORE_UNLOAD_EVENT, id, None::<&str>)?;
    }

    Ok(())
  }

  /// Runs any necessary tasks before the application exits.
  #[doc(hidden)]
  pub fn on_exit(&self) -> Result<()> {
    self.clear_autosave();

    for rid in self.rids() {
      if let Ok(resource) = StoreResource::take(&self.app, rid) {
        resource.locked(|store| {
          if store.save_on_exit {
            let _ = store.save_now();
          }
        });
      }
    }

    Ok(())
  }
}

impl<R: Runtime> Resource for StoreCollection<R> {}

impl<R: Runtime> fmt::Debug for StoreCollection<R> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("StoreCollection")
      .field("default_save_strategy", &self.default_save_strategy)
      .field("on_load", &self.on_load.is_some())
      .field("pretty", &self.pretty)
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
      .finish_non_exhaustive()
  }
}
