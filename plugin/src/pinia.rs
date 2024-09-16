use crate::error::Result;
use crate::io_err;
use crate::store::{Store, StoreState};
use serde::de::DeserializeOwned;
use serde_json::json;
use serde_json::Value as Json;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Runtime};

#[cfg(feature = "async-pinia")]
use {
  crate::{BoxFuture, FutureExt},
  std::time::Duration,
  tokio::task::AbortHandle,
};

#[cfg(feature = "ahash")]
use ahash::{HashMap, HashSet};
#[cfg(not(feature = "ahash"))]
use std::collections::{HashMap, HashSet};

pub struct Pinia<R: Runtime> {
  pub(crate) app: AppHandle<R>,
  pub(crate) path: PathBuf,
  pub(crate) sync_denylist: HashSet<String>,

  #[cfg(not(feature = "async-pinia"))]
  pub(crate) stores: std::sync::Mutex<HashMap<String, Store<R>>>,
  #[cfg(feature = "async-pinia")]
  pub(crate) stores: tokio::sync::Mutex<HashMap<String, Store<R>>>,

  #[cfg(feature = "async-pinia")]
  pub(crate) autosave: std::sync::Mutex<Option<AbortHandle>>,
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

impl<R: Runtime> Pinia<R> {
  /// Directory where the stores are saved.
  pub fn path(&self) -> &Path {
    &self.path
  }

  /// Calls a closure with a mutable reference to the store with the given id.
  #[cfg(not(feature = "async-pinia"))]
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
  #[cfg(feature = "async-pinia")]
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
  #[cfg(not(feature = "async-pinia"))]
  pub fn save(&self, id: impl AsRef<str>) -> Result<()> {
    let stores = self.stores.lock().unwrap();
    get_store!(stores, id)?.save()
  }

  /// Saves a store to the disk.
  #[cfg(feature = "async-pinia")]
  pub async fn save(&self, id: impl AsRef<str>) -> Result<()> {
    let stores = self.stores.lock().await;
    get_store!(stores, id)?.save().await
  }

  /// Saves some stores to the disk.
  #[cfg(not(feature = "async-pinia"))]
  pub fn save_some(&self, ids: &[impl AsRef<str>]) -> Result<()> {
    let stores = self.stores.lock().unwrap();
    for id in ids {
      get_store!(stores, id)?.save()?;
    }

    Ok(())
  }

  /// Saves some stores to the disk.
  #[cfg(feature = "async-pinia")]
  pub async fn save_some(&self, ids: &[impl AsRef<str>]) -> Result<()> {
    let stores = self.stores.lock().await;
    for id in ids {
      get_store!(stores, id)?.save().await?;
    }

    Ok(())
  }

  /// Saves all the stores to the disk.
  #[cfg(not(feature = "async-pinia"))]
  pub fn save_all(&self) -> Result<()> {
    let stores = self.stores.lock().unwrap();
    stores.values().try_for_each(Store::save)
  }

  /// Saves all the stores to the disk.
  #[cfg(feature = "async-pinia")]
  pub async fn save_all(&self) -> Result<()> {
    let stores = self.stores.lock().await;
    for store in stores.values() {
      store.save().await?;
    }

    Ok(())
  }

  /// Lists all the store ids.
  #[cfg(not(feature = "async-pinia"))]
  pub fn ids(&self) -> Vec<String> {
    let stores = self.stores.lock().unwrap();
    stores.keys().cloned().collect()
  }

  /// Lists all the store ids.
  #[cfg(feature = "async-pinia")]
  pub async fn ids(&self) -> Vec<String> {
    let stores = self.stores.lock().await;
    stores.keys().cloned().collect()
  }

  /// Gets a clone of the store state if it exists.
  ///
  /// **WARNING:** Changes to the returned state will not be reflected in the store.
  #[cfg(not(feature = "async-pinia"))]
  pub fn store_state(&self, store_id: impl AsRef<str>) -> Option<StoreState> {
    let stores = self.stores.lock().unwrap();
    stores.get(store_id.as_ref()).map(Store::state)
  }

  /// Gets a clone of the store state if it exists.
  ///
  /// **WARNING:** Changes to the returned state will not be reflected in the store.
  #[cfg(feature = "async-pinia")]
  pub async fn store_state(&self, store_id: impl AsRef<str>) -> Option<StoreState> {
    let stores = self.stores.lock().await;
    stores.get(store_id.as_ref()).map(Store::state)
  }

  /// Gets the store state if it exists, then tries to deserialize it as an instance of type `T`.
  #[cfg(not(feature = "async-pinia"))]
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
  #[cfg(feature = "async-pinia")]
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
  #[cfg(not(feature = "async-pinia"))]
  pub fn get(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<Json> {
    let stores = self.stores.lock().unwrap();
    stores
      .get(store_id.as_ref())
      .and_then(|store| store.get_owned(key))
  }

  /// Gets a value from a store.
  #[cfg(feature = "async-pinia")]
  pub async fn get(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<Json> {
    let stores = self.stores.lock().await;
    stores
      .get(store_id.as_ref())
      .and_then(|store| store.get_owned(key))
  }

  #[cfg(not(feature = "async-pinia"))]
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

  #[cfg(feature = "async-pinia")]
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
  #[cfg(not(feature = "async-pinia"))]
  pub fn set(&self, store_id: impl AsRef<str>, key: impl AsRef<str>, value: Json) -> Result<()> {
    self.with_store(store_id, |store| store.set(key, value))
  }

  /// Sets a key-value pair in a store.
  #[cfg(feature = "async-pinia")]
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
  #[cfg(not(feature = "async-pinia"))]
  pub fn patch(&self, store_id: impl AsRef<str>, state: StoreState) -> Result<()> {
    self.with_store(store_id, |store| store.patch(state))
  }

  /// Patches a store state.
  #[cfg(feature = "async-pinia")]
  pub async fn patch(&self, store_id: impl AsRef<str>, state: StoreState) -> Result<()> {
    self
      .with_store(store_id, |store| async { store.patch(state) }.boxed())
      .await
  }

  /// Saves the stores periodically.
  #[cfg(feature = "async-pinia")]
  #[cfg_attr(docsrs, doc(cfg(feature = "async-pinia")))]
  pub fn set_autosave(&self, duration: Duration) {
    use crate::ManagerExt;
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
        let _ = app.pinia().save_all().await;
      }
    });

    let mut guard = self.autosave.lock().unwrap();
    *guard = Some(task.abort_handle());
  }

  /// Stops the autosave.
  #[cfg(feature = "async-pinia")]
  #[cfg_attr(docsrs, doc(cfg(feature = "async-pinia")))]
  pub fn clear_autosave(&self) {
    let mut guard = self.autosave.lock().unwrap();
    if let Some(autosave) = guard.take() {
      drop(guard);
      autosave.abort();
    }
  }

  #[cfg(not(feature = "async-pinia"))]
  pub(crate) fn unload_store(&self, id: &str) -> Result<()> {
    let mut stores = self.stores.lock().unwrap();
    if let Some(store) = stores.remove(id) {
      drop(stores);
      store.save()?;
    }

    Ok(())
  }

  #[cfg(feature = "async-pinia")]
  pub(crate) async fn unload_store(&self, id: &str) -> Result<()> {
    let mut stores = self.stores.lock().await;
    if let Some(store) = stores.remove(id) {
      drop(stores);
      store.save().await?;
    }

    Ok(())
  }
}
