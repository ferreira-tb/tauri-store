use super::{StoreCollection, StoreResource};
use crate::error::Result;
use crate::event::{emit_all, STORE_UNLOADED_EVENT};
use crate::io_err;
use crate::store::{Store, StoreState, WatcherResult};
use serde::de::DeserializeOwned;
use serde_json::Value as Json;
use std::future::Future;
use std::sync::Arc;
use tauri::{AppHandle, Runtime};

#[cfg(feature = "unstable-async")]
impl<R: Runtime> StoreCollection<R> {
  pub(crate) async fn get_resource<Id>(&self, id: Id) -> Result<Arc<StoreResource<R>>>
  where
    Id: AsRef<str>,
  {
    let id = id.as_ref();
    let rid = if let Some(rid) = self.rid(id) {
      rid
    } else {
      let app = self.app.clone();
      let (rid, resource) = Store::load(app, id).await?;
      if let Some(on_load) = &self.on_load {
        on_load(&*resource.inner.lock().await).await?;
      }

      self.stores.insert(id.to_owned(), rid);
      rid
    };

    StoreResource::get(&self.app, rid)
  }

  /// Calls a closure with a mutable reference to the store with the given id.
  pub async fn with_store<F, Fut, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> Fut + Send,
    Fut: Future<Output = T> + Send,
    T: Send + 'static,
  {
    let resource = self.get_resource(id).await?;
    let mut store = resource.inner.lock().await;
    Ok(f(&mut *store).await)
  }

  /// Saves a store to the disk.
  pub async fn save(&self, id: impl AsRef<str>) -> Result<()> {
    let resource = self.get_resource(id).await?;
    let store = resource.inner.lock().await;
    store.save().await
  }

  /// Saves a store to the disk immediately, ignoring the save strategy.
  pub async fn save_now(&self, id: impl AsRef<str>) -> Result<()> {
    let resource = self.get_resource(id).await?;
    let store = resource.inner.lock().await;
    store.abort_pending_save();
    store.save_now().await
  }

  /// Saves some stores to the disk.
  pub async fn save_some(&self, ids: &[impl AsRef<str>]) -> Result<()> {
    for id in ids {
      self.save(id).await?;
    }

    Ok(())
  }

  /// Saves some stores to the disk immediately, ignoring the save strategy.
  pub async fn save_some_now(&self, ids: &[impl AsRef<str>]) -> Result<()> {
    for id in ids {
      self.save_now(id).await?;
    }

    Ok(())
  }

  /// Saves all the stores to the disk.
  pub async fn save_all(&self) -> Result<()> {
    for rid in self.rids() {
      StoreResource::save(&self.app, rid).await?;
    }

    Ok(())
  }

  /// Saves all the stores to the disk immediately, ignoring the save strategy.
  pub async fn save_all_now(&self) -> Result<()> {
    for rid in self.rids() {
      StoreResource::save_now(&self.app, rid).await?;
    }

    Ok(())
  }

  /// Gets a clone of the store state.
  pub async fn store_state(&self, store_id: impl AsRef<str>) -> Option<StoreState> {
    let resource = self.get_resource(store_id).await.ok()?;
    let store = resource.inner.lock().await;
    Some(store.state().clone())
  }

  /// Gets the store state, then tries to parse it as an instance of type `T`.
  pub async fn try_store_state<T>(&self, store_id: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let resource = self.get_resource(store_id).await?;
    let store = resource.inner.lock().await;
    store.try_state()
  }

  /// Gets a value from a store.
  pub async fn get(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Option<Json> {
    let resource = self.get_resource(store_id).await.ok()?;
    let store = resource.inner.lock().await;
    store.get(key).cloned()
  }

  /// Gets a value from a store and tries to parse it as an instance of type `T`.
  pub async fn try_get<T>(&self, store_id: impl AsRef<str>, key: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let key = key.as_ref();
    let Some(value) = self.get(store_id, key).await else {
      return io_err!(NotFound, "key not found: {key}");
    };

    Ok(serde_json::from_value(value)?)
  }

  /// Sets a key-value pair in a store.
  pub async fn set<Id, Key>(&self, store_id: Id, key: Key, value: Json) -> Result<()>
  where
    Id: AsRef<str>,
    Key: AsRef<str>,
  {
    let resource = self.get_resource(store_id).await?;
    let mut store = resource.inner.lock().await;
    store.set(key, value)
  }

  /// Patches a store state.
  pub async fn patch(&self, store_id: impl AsRef<str>, state: StoreState) -> Result<()> {
    let resource = self.get_resource(store_id).await?;
    let mut store = resource.inner.lock().await;
    store.patch(state)
  }

  /// Watches a store for changes.
  pub async fn watch<F>(&self, store_id: impl AsRef<str>, f: F) -> Result<u32>
  where
    F: Fn(AppHandle<R>) -> WatcherResult + Send + Sync + 'static,
  {
    let resource = self.get_resource(store_id).await?;
    let mut store = resource.inner.lock().await;
    Ok(store.watch(f))
  }

  /// Removes a watcher from a store.
  pub async fn unwatch(&self, store_id: impl AsRef<str>, listener_id: u32) -> Result<bool> {
    let resource = self.get_resource(store_id).await?;
    let mut store = resource.inner.lock().await;
    Ok(store.unwatch(listener_id))
  }

  pub async fn unload_store(&self, id: &str) -> Result<()> {
    if let Some((_, rid)) = self.stores.remove(id) {
      // See the comment in the sync version of this method for why we use `save_now` here.
      StoreResource::take(&self.app, rid)?
        .inner
        .lock()
        .await
        .save_now()
        .await?;

      emit_all(&self.app, STORE_UNLOADED_EVENT, id)?;
    }

    Ok(())
  }
}
