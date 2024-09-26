use crate::error::Result;
use crate::event::{emit_all, STORE_UNLOADED_EVENT};
use crate::io_err;
use crate::store::{Store, StoreState};
use serde::de::DeserializeOwned;
use serde_json::json;
use serde_json::Value as Json;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex as StdMutex, OnceLock};
use tauri::{AppHandle, Manager, Resource, ResourceId, Runtime};

#[cfg(feature = "unstable-async")]
use {
  crate::manager::ManagerExt,
  crate::{BoxFuture, FutureExt},
  std::time::Duration,
  tokio::sync::Mutex as TokioMutex,
  tokio::task::AbortHandle,
};

#[cfg(feature = "ahash")]
use ahash::{HashMap, HashMapExt, HashSet};
#[cfg(not(feature = "ahash"))]
use std::collections::{HashMap, HashSet};

pub(crate) static RESOURCE_ID: OnceLock<ResourceId> = OnceLock::new();

pub struct StoreCollection<R: Runtime> {
  pub(crate) app: AppHandle<R>,
  pub(crate) path: PathBuf,

  #[cfg(not(feature = "unstable-async"))]
  pub(crate) stores: StdMutex<HashMap<String, Store<R>>>,
  #[cfg(feature = "unstable-async")]
  pub(crate) stores: TokioMutex<HashMap<String, Store<R>>>,

  pub(crate) pretty: bool,
  pub(crate) sync_denylist: StdMutex<HashSet<String>>,

  #[cfg(feature = "unstable-async")]
  pub(crate) autosave: StdMutex<Option<AbortHandle>>,
}

macro_rules! get_store {
  ($stores:expr, $id:expr) => {{
    let id = $id.as_ref();
    match $stores.get(id) {
      Some(store) => Ok(store),
      None => $crate::io_err!(NotFound, "store not found: {id}"),
    }
  }};
}

impl<R: Runtime> StoreCollection<R> {
  pub fn builder() -> StoreCollectionBuilder {
    StoreCollectionBuilder::new()
  }

  /// Directory where the stores are saved.
  pub fn path(&self) -> &Path {
    &self.path
  }

  /// Calls a closure with a mutable reference to the store with the given id.
  #[cfg(not(feature = "unstable-async"))]
  pub fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> Result<T>,
  {
    let id = id.as_ref();
    let mut stores = self.stores.lock().unwrap();
    if !stores.contains_key(id) {
      let app = self.app.clone();
      let store = Store::load(app, id)?;
      stores.insert(id.to_owned(), store);
    }

    f(stores.get_mut(id).expect("store should exist"))
  }

  /// Calls a closure with a mutable reference to the store with the given id.
  #[cfg(feature = "unstable-async")]
  pub fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> BoxFuture<Result<T>>
  where
    F: FnOnce(&mut Store<R>) -> BoxFuture<Result<T>> + Send + 'static,
    T: Send + 'static,
  {
    let id = id.as_ref().to_owned();
    let app = self.app.clone();
    Box::pin(async move {
      let mut stores = self.stores.lock().await;
      if !stores.contains_key(&id) {
        let store = Store::load(app, &id).unwrap();
        stores.insert(id.clone(), store);
      }

      f(stores.get_mut(&id).expect("store should exist")).await
    })
  }

  /// Saves a store to the disk.
  #[cfg(not(feature = "unstable-async"))]
  pub fn save(&self, id: impl AsRef<str>) -> Result<()> {
    let stores = self.stores.lock().unwrap();
    get_store!(stores, id)?.save()
  }

  /// Saves a store to the disk.
  #[cfg(feature = "unstable-async")]
  pub async fn save(&self, id: impl AsRef<str>) -> Result<()> {
    let stores = self.stores.lock().await;
    get_store!(stores, id)?.save().await
  }

  /// Saves some stores to the disk.
  #[cfg(not(feature = "unstable-async"))]
  pub fn save_some(&self, ids: &[impl AsRef<str>]) -> Result<()> {
    let stores = self.stores.lock().unwrap();
    for id in ids {
      get_store!(stores, id)?.save()?;
    }

    Ok(())
  }

  /// Saves some stores to the disk.
  #[cfg(feature = "unstable-async")]
  pub async fn save_some(&self, ids: &[impl AsRef<str>]) -> Result<()> {
    let stores = self.stores.lock().await;
    for id in ids {
      get_store!(stores, id)?.save().await?;
    }

    Ok(())
  }

  /// Saves all the stores to the disk.
  #[cfg(not(feature = "unstable-async"))]
  pub fn save_all(&self) -> Result<()> {
    let stores = self.stores.lock().unwrap();
    stores.values().try_for_each(Store::save)
  }

  /// Saves all the stores to the disk.
  #[cfg(feature = "unstable-async")]
  pub async fn save_all(&self) -> Result<()> {
    let stores = self.stores.lock().await;
    for store in stores.values() {
      store.save().await?;
    }

    Ok(())
  }

  /// Lists all the store ids.
  #[cfg(not(feature = "unstable-async"))]
  pub fn ids(&self) -> Vec<String> {
    let stores = self.stores.lock().unwrap();
    stores.keys().cloned().collect()
  }

  /// Lists all the store ids.
  #[cfg(feature = "unstable-async")]
  pub async fn ids(&self) -> Vec<String> {
    let stores = self.stores.lock().await;
    stores.keys().cloned().collect()
  }

  /// Gets a clone of the store state if it exists.
  ///
  /// **WARNING:** Changes to the returned state will not be reflected in the store.
  #[cfg(not(feature = "unstable-async"))]
  pub fn store_state(&self, store_id: impl AsRef<str>) -> Option<StoreState> {
    let stores = self.stores.lock().unwrap();
    stores.get(store_id.as_ref()).map(Store::state)
  }

  /// Gets a clone of the store state if it exists.
  ///
  /// **WARNING:** Changes to the returned state will not be reflected in the store.
  #[cfg(feature = "unstable-async")]
  pub async fn store_state(&self, store_id: impl AsRef<str>) -> Option<StoreState> {
    let stores = self.stores.lock().await;
    stores.get(store_id.as_ref()).map(Store::state)
  }

  /// Gets the store state if it exists, then tries to deserialize it as an instance of type `T`.
  #[cfg(not(feature = "unstable-async"))]
  pub fn try_store_state<T>(&self, store_id: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let stores = self.stores.lock().unwrap();
    let store = get_store!(stores, store_id)?;

    let state = json!(store.state());
    serde_json::from_value(state).map_err(Into::into)
  }

  /// Gets the store state if it exists, then tries to deserialize it as an instance of type `T`.
  #[cfg(feature = "unstable-async")]
  pub async fn try_store_state<T>(&self, store_id: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let stores = self.stores.lock().await;
    let store = get_store!(stores, store_id)?;

    let state = json!(store.state());
    serde_json::from_value(state).map_err(Into::into)
  }

  /// Gets a value from a store.
  #[cfg(not(feature = "unstable-async"))]
  pub fn get(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<Json> {
    let stores = self.stores.lock().unwrap();
    stores
      .get(store_id.as_ref())
      .and_then(|store| store.get_owned(key))
  }

  /// Gets a value from a store.
  #[cfg(feature = "unstable-async")]
  pub async fn get(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<Json> {
    let stores = self.stores.lock().await;
    stores
      .get(store_id.as_ref())
      .and_then(|store| store.get_owned(key))
  }

  #[cfg(not(feature = "unstable-async"))]
  /// Gets a value from a store and tries to interpret it as an instance of type `T`.
  pub fn try_get<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let key = key.as_ref();
    let Some(value) = self.get(store_id, key) else {
      return io_err!(NotFound, "key not found: {key}");
    };

    serde_json::from_value(value).map_err(Into::into)
  }

  #[cfg(feature = "unstable-async")]
  /// Gets a value from a store and tries to interpret it as an instance of type `T`.
  pub async fn try_get<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let key = key.as_ref();
    let Some(value) = self.get(store_id, key).await else {
      return io_err!(NotFound, "key not found: {key}");
    };

    serde_json::from_value(value).map_err(Into::into)
  }

  /// Sets a key-value pair in a store.
  #[cfg(not(feature = "unstable-async"))]
  pub fn set(&self, store_id: impl AsRef<str>, key: impl AsRef<str>, value: Json) -> Result<()> {
    self.with_store(store_id, |store| store.set(key, value))
  }

  /// Sets a key-value pair in a store.
  #[cfg(feature = "unstable-async")]
  pub async fn set(
    &self,
    store_id: impl AsRef<str>,
    key: impl AsRef<str>,
    value: Json,
  ) -> Result<()> {
    let key = key.as_ref().to_owned();
    self
      .with_store(store_id, |store| async { store.set(key, value) }.boxed())
      .await
  }

  /// Patches a store state.
  #[cfg(not(feature = "unstable-async"))]
  pub fn patch(&self, store_id: impl AsRef<str>, state: StoreState) -> Result<()> {
    self.with_store(store_id, |store| store.patch(state))
  }

  /// Patches a store state.
  #[cfg(feature = "unstable-async")]
  pub async fn patch(&self, store_id: impl AsRef<str>, state: StoreState) -> Result<()> {
    self
      .with_store(store_id, |store| async { store.patch(state) }.boxed())
      .await
  }

  /// Remove a store from the sync denylist.
  pub fn enable_sync(&self, store_id: impl AsRef<str>) {
    self
      .sync_denylist
      .lock()
      .expect("sync denylist mutex is poisoned")
      .remove(store_id.as_ref());
  }

  /// Add a store to the sync denylist.
  pub fn disable_sync(&self, store_id: impl AsRef<str>) {
    self
      .sync_denylist
      .lock()
      .expect("sync denylist mutex is poisoned")
      .insert(store_id.as_ref().to_owned());
  }

  /// Saves the stores periodically.
  #[cfg(feature = "unstable-async")]
  #[cfg_attr(docsrs, doc(cfg(feature = "unstable-async")))]
  pub fn set_autosave(&self, duration: Duration) {
    use tauri::async_runtime::{self, RuntimeHandle};
    use tokio::time::{self, MissedTickBehavior};

    self.clear_autosave();

    let app = self.app.clone();
    let RuntimeHandle::Tokio(runtime) = async_runtime::handle();
    let task = runtime.spawn(async move {
      let mut interval = time::interval(duration);
      interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
      loop {
        interval.tick().await;
        let _ = app.store_collection().save_all().await;
      }
    });

    let mut guard = self.autosave.lock().unwrap();
    *guard = Some(task.abort_handle());
  }

  /// Stops the autosave.
  #[cfg(feature = "unstable-async")]
  #[cfg_attr(docsrs, doc(cfg(feature = "unstable-async")))]
  pub fn clear_autosave(&self) {
    let mut guard = self.autosave.lock().unwrap();
    if let Some(autosave) = guard.take() {
      drop(guard);
      autosave.abort();
    }
  }

  #[cfg(not(feature = "unstable-async"))]
  pub fn unload_store(&self, id: &str) -> Result<()> {
    let mut stores = self.stores.lock().unwrap();
    if let Some(store) = stores.remove(id) {
      drop(stores);
      store.save()?;
      emit_all(&self.app, STORE_UNLOADED_EVENT, id)?;
    }

    Ok(())
  }

  #[cfg(feature = "unstable-async")]
  pub async fn unload_store(&self, id: &str) -> Result<()> {
    let mut stores = self.stores.lock().await;
    if let Some(store) = stores.remove(id) {
      drop(stores);
      store.save().await?;
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
      .finish_non_exhaustive()
  }
}

#[cfg(feature = "unstable-async")]
impl<R: Runtime> Drop for StoreCollection<R> {
  fn drop(&mut self) {
    self.clear_autosave();
  }
}

#[derive(Debug, Default)]
pub struct StoreCollectionBuilder {
  path: Option<PathBuf>,
  pretty: bool,
  sync_denylist: Option<HashSet<String>>,
}

impl StoreCollectionBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  #[must_use]
  pub fn path(mut self, path: impl AsRef<Path>) -> Self {
    self.path = Some(path.as_ref().to_path_buf());
    self
  }

  #[must_use]
  pub fn pretty(mut self, yes: bool) -> Self {
    self.pretty = yes;
    self
  }

  #[must_use]
  pub fn sync_denylist(mut self, sync_denylist: HashSet<String>) -> Self {
    self.sync_denylist = Some(sync_denylist);
    self
  }

  pub fn build<R: Runtime>(mut self, app: &AppHandle<R>) -> Arc<StoreCollection<R>> {
    let path = self.path.take().unwrap_or_else(|| {
      app
        .path()
        .app_data_dir()
        .expect("failed to resolve app data dir")
        .join("tauri-store")
    });

    let sync_denylist = self.sync_denylist.take().unwrap_or_default();

    let collection = Arc::new(StoreCollection::<R> {
      app: app.clone(),
      path,
      pretty: self.pretty,
      sync_denylist: StdMutex::new(sync_denylist),

      #[cfg(not(feature = "unstable-async"))]
      stores: StdMutex::new(HashMap::new()),
      #[cfg(feature = "unstable-async")]
      stores: TokioMutex::new(HashMap::new()),

      #[cfg(feature = "unstable-async")]
      autosave: StdMutex::new(None),
    });

    let rid = app
      .resources_table()
      .add_arc(Arc::clone(&collection));

    let _ = RESOURCE_ID.set(rid);

    collection
  }
}
