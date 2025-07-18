mod autosave;
mod builder;
mod marker;
mod path;

use crate::error::Result;
use crate::event::{emit, STORE_UNLOAD_EVENT};
use crate::meta::Meta;
use crate::store::{SaveStrategy, Store, StoreId, StoreResource, StoreState, WatcherId};
use autosave::Autosave;
use dashmap::{DashMap, DashSet};
use serde::de::DeserializeOwned;
use serde_json::Value as Json;
use std::fmt;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Resource, ResourceId, Runtime};

pub use builder::StoreCollectionBuilder;
pub use marker::{CollectionMarker, DefaultMarker};

#[cfg(feature = "unstable-migration")]
use crate::migration::Migrator;

/// Closure to be called when a store is loaded.
pub type OnLoadFn<R, C> = dyn Fn(&Store<R, C>) -> Result<()> + Send + Sync;

/// A collection of stores.
/// This is the core component for store plugins.
pub struct StoreCollection<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
  pub(crate) app: AppHandle<R>,
  pub(crate) name: Box<str>,
  pub(crate) path: Mutex<PathBuf>,
  pub(crate) stores: DashMap<StoreId, ResourceId>,
  pub(crate) on_load: Option<Box<OnLoadFn<R, C>>>,
  pub(crate) autosave: Mutex<Autosave>,
  pub(crate) default_save_strategy: SaveStrategy,
  pub(crate) save_denylist: DashSet<StoreId>,
  pub(crate) sync_denylist: DashSet<StoreId>,
  pub(crate) pretty: bool,
  phantom: PhantomData<C>,

  #[cfg(feature = "unstable-migration")]
  pub(crate) migrator: Mutex<Migrator>,
}

impl<R, C> StoreCollection<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
  /// Builds a new store collection.
  pub fn builder() -> StoreCollectionBuilder<R, C> {
    StoreCollectionBuilder::<R, C>::new()
  }

  pub(crate) fn get_resource(&self, id: impl AsRef<str>) -> Result<Arc<StoreResource<R, C>>> {
    let id = StoreId::from(id.as_ref());
    let rid = match self.rid(&id) {
      Some(rid) => rid,
      None => self.load_store(id)?,
    };

    StoreResource::get(&self.app, rid)
  }

  fn load_store(&self, id: StoreId) -> Result<ResourceId> {
    let (rid, resource) = Store::load(&self.app, &id)?;
    if let Some(on_load) = &self.on_load {
      resource.locked(|store| on_load(store))?;
    }

    self.stores.insert(id, rid);
    Ok(rid)
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

  /// Calls a closure with a mutable reference to the store with the given id.
  pub fn with_store<F, T>(&self, store_id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R, C>) -> T,
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

  /// Gets the store state, then tries to parse it as an instance of type `T`.
  ///
  /// If it cannot be parsed, returns the provided default value.
  pub fn try_state_or<T>(&self, store_id: impl AsRef<str>, default: T) -> Result<T>
  where
    T: DeserializeOwned,
  {
    self
      .get_resource(store_id)?
      .locked(move |store| Ok(store.try_state_or(default)))
  }

  /// Gets the store state, then tries to parse it as an instance of type `T`.
  ///
  /// If it cannot be parsed, returns the default value of `T`.
  pub fn try_state_or_default<T>(&self, store_id: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned + Default,
  {
    self
      .get_resource(store_id)?
      .locked(|store| Ok(store.try_state_or_default()))
  }

  /// Gets the store state, then tries to parse it as an instance of type `T`.
  ///
  /// If it cannot be parsed, returns the result of the provided closure.
  pub fn try_state_or_else<T>(&self, store_id: impl AsRef<str>, f: impl FnOnce() -> T) -> Result<T>
  where
    T: DeserializeOwned,
  {
    self
      .get_resource(store_id)?
      .locked(|store| Ok(store.try_state_or_else(f)))
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
      .try_for_each(|rid| StoreResource::<R, C>::save(&self.app, rid))
  }

  /// Saves all the stores to the disk immediately, ignoring the save strategy.
  pub fn save_all_now(&self) -> Result<()> {
    self
      .rids()
      .into_iter()
      .try_for_each(|rid| StoreResource::<R, C>::save_now(&self.app, rid))
  }

  /// Default save strategy for the stores.
  /// This can be overridden on a per-store basis.
  #[inline]
  pub fn default_save_strategy(&self) -> SaveStrategy {
    self.default_save_strategy
  }

  /// Saves the stores periodically.
  pub fn set_autosave(&self, duration: Duration) {
    if let Ok(mut autosave) = self.autosave.lock() {
      autosave.set_duration(duration);
      autosave.start::<R, C>(&self.app);
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

  /// Removes a store from the save denylist.
  pub fn allow_save(&self, id: impl AsRef<str>) {
    let id = StoreId::from(id.as_ref());
    self.save_denylist.remove(&id);
  }

  /// Adds a store to the save denylist.
  pub fn deny_save(&self, id: impl AsRef<str>) {
    let id = StoreId::from(id.as_ref());
    self.save_denylist.insert(id);
  }

  /// Removes a store from the sync denylist.
  pub fn allow_sync(&self, id: impl AsRef<str>) {
    let id = StoreId::from(id.as_ref());
    self.sync_denylist.remove(&id);
  }

  /// Adds a store to the deny denylist.
  pub fn deny_sync(&self, id: impl AsRef<str>) {
    let id = StoreId::from(id.as_ref());
    self.sync_denylist.insert(id);
  }

  /// Removes the store from the collection.
  #[doc(hidden)]
  pub fn unload_store(&self, id: &StoreId) -> Result<()> {
    if let Some((_, rid)) = self.stores.remove(id) {
      // The store needs to be saved immediately here.
      // Otherwise, the plugin might try to load it again if `StoreCollection::get_resource` is called.
      // This scenario will happen whenever the save strategy is not `Immediate`.
      let resource = StoreResource::<R, C>::take(&self.app, rid)?;
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
      if let Ok(resource) = StoreResource::<R, C>::take(&self.app, rid) {
        resource.locked(|store| {
          store.abort_pending_save();
          if store.save_on_exit {
            let _ = store.save_now();
          }
        });
      }
    }

    Meta::write(self)?;

    Ok(())
  }
}

impl<R, C> Resource for StoreCollection<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
}

impl<R, C> fmt::Debug for StoreCollection<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("StoreCollection")
      .field("default_save_strategy", &self.default_save_strategy)
      .field("pretty", &self.pretty)
      .finish_non_exhaustive()
  }
}
