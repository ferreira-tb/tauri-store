use crate::error::Result;
use crate::event::{Payload, STORE_UPDATED_EVENT};
use crate::manager::ManagerExt;
use crate::state::{StoreState, StoreStateExt};
use crate::watch::{Watcher, WatcherResult};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value as Json;
use std::fmt;
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{async_runtime, AppHandle, Runtime};

#[cfg(feature = "unstable-async")]
use tauri::async_runtime::spawn_blocking;

#[cfg(feature = "ahash")]
use ahash::HashMap;
#[cfg(not(feature = "ahash"))]
use std::collections::HashMap;

pub struct Store<R: Runtime> {
  app: AppHandle<R>,
  pub(crate) id: String,
  pub(crate) state: StoreState,
  pub(crate) listeners: Arc<Mutex<HashMap<u32, Watcher>>>,
}

impl<R: Runtime> Store<R> {
  fn blocking_load(app: AppHandle<R>, id: String) -> Result<Self> {
    let path = store_path(&app, &id);
    let state = match std::fs::read(path) {
      Ok(bytes) => serde_json::from_slice(&bytes)?,
      Err(e) if e.kind() == NotFound => StoreState::default(),
      Err(e) => return Err(e.into()),
    };

    Ok(Self {
      app,
      id,
      state,
      listeners: Arc::new(Mutex::default()),
    })
  }

  #[cfg(not(feature = "unstable-async"))]
  pub(crate) fn load(app: AppHandle<R>, id: impl AsRef<str>) -> Result<Self> {
    let id = id.as_ref().to_owned();
    Self::blocking_load(app, id)
  }

  #[cfg(feature = "unstable-async")]
  pub(crate) async fn load(app: AppHandle<R>, id: impl AsRef<str>) -> Result<Self> {
    let id = id.as_ref().to_owned();
    spawn_blocking(move || Self::blocking_load(app, id)).await?
  }

  /// Save the store state to the disk.
  #[cfg(not(feature = "unstable-async"))]
  pub fn save(&self) -> Result<()> {
    use std::fs::{self, File};
    use std::io::Write;

    let collection = self.app.store_collection();
    if collection
      .save_denylist
      .as_ref()
      .is_some_and(|it| it.contains(&self.id))
    {
      return Ok(());
    }

    fs::create_dir_all(collection.path())?;

    let bytes = to_bytes(&self.state, collection.pretty)?;
    let mut file = File::create(self.path())?;
    file.write_all(&bytes)?;

    Ok(())
  }

  /// Save the store state to the disk.
  #[cfg(feature = "unstable-async")]
  pub async fn save(&self) -> Result<()> {
    use tokio::fs::{self, File};
    use tokio::io::AsyncWriteExt;

    let collection = self.app.store_collection();
    if collection
      .save_denylist
      .as_ref()
      .is_some_and(|it| it.contains(&self.id))
    {
      return Ok(());
    }

    fs::create_dir_all(collection.path()).await?;

    let bytes = to_bytes(&self.state, collection.pretty)?;
    let mut file = File::create(self.path()).await?;
    file.write_all(&bytes).await?;
    file.flush().await?;

    Ok(())
  }

  /// The id of the store.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Path to the store file.
  pub fn path(&self) -> PathBuf {
    store_path(&self.app, &self.id)
  }

  /// Gets a clone of the store state.
  ///
  /// **WARNING:** Changes to the returned state will not be reflected in the store.
  pub fn state(&self) -> StoreState {
    self.state.clone()
  }

  /// Gets a value from the store.
  pub fn get(&self, key: impl AsRef<str>) -> Option<&Json> {
    self.state.get(key.as_ref())
  }

  /// Gets a clone of the value from the store.
  pub fn get_owned(&self, key: impl AsRef<str>) -> Option<Json> {
    self.state.get_owned(key)
  }

  /// Gets a value from the store and tries to interpret it as an instance of type `T`.
  pub fn try_get<T>(&self, key: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    self.state.try_get(key)
  }

  /// Sets a key-value pair in the store.
  pub fn set(&mut self, key: impl AsRef<str>, value: Json) -> Result<()> {
    self.state.insert(key.as_ref().to_owned(), value);
    self.emit(None)?;
    self.call_listeners();

    Ok(())
  }

  /// Patches the store state, optionally having a window as the source.
  pub fn patch_with_source<'a, S>(&mut self, state: StoreState, source: S) -> Result<()>
  where
    S: Into<Option<&'a str>>,
  {
    self.state.extend(state);
    self.emit(source)?;
    self.call_listeners();

    Ok(())
  }

  /// Patches the store state.
  pub fn patch(&mut self, state: StoreState) -> Result<()> {
    self.patch_with_source(state, None)
  }

  /// Whether the store has a key.
  pub fn has(&self, key: impl AsRef<str>) -> bool {
    self.state.contains_key(key.as_ref())
  }

  /// Creates an iterator over the store keys.
  pub fn keys(&self) -> impl Iterator<Item = &String> {
    self.state.keys()
  }

  /// Creates an iterator over the store values.
  pub fn values(&self) -> impl Iterator<Item = &Json> {
    self.state.values()
  }

  /// Creates an iterator over the store entries.
  pub fn entries(&self) -> impl Iterator<Item = (&String, &Json)> {
    self.state.iter()
  }

  /// Returns the amount of items in the store.
  pub fn len(&self) -> usize {
    self.state.len()
  }

  /// Whether the store is empty.
  pub fn is_empty(&self) -> bool {
    self.state.is_empty()
  }

  /// Watches the store for changes.
  pub fn watch<F>(&self, f: F) -> u32
  where
    F: Fn(Arc<StoreState>) -> WatcherResult + Send + Sync + 'static,
  {
    let listener = Watcher::new(f);
    let id = listener.id;
    self
      .listeners
      .lock()
      .expect("listeners mutex is poisoned")
      .insert(id, listener);

    id
  }

  /// Removes a listener from this store.
  pub fn unwatch(&self, id: u32) -> bool {
    self
      .listeners
      .lock()
      .expect("listeners mutex is poisoned")
      .remove(&id)
      .is_some()
  }

  fn emit<'a, S>(&self, source: S) -> Result<()>
  where
    S: Into<Option<&'a str>>,
  {
    let source: Option<&str> = source.into();
    let collection = self.app.store_collection();

    // If we also skip the store when the source is the backend,
    // the window where the store resides would never know about the change.
    if source.is_some()
      && collection
        .sync_denylist
        .as_ref()
        .is_some_and(|it| it.contains(&self.id))
    {
      return Ok(());
    }

    let payload = Payload::from(self);
    if let Some(source) = source {
      payload.emit_filter(&self.app, STORE_UPDATED_EVENT, |it| it != source)
    } else {
      payload.emit_all(&self.app, STORE_UPDATED_EVENT)
    }
  }

  fn call_listeners(&self) {
    let listeners = self
      .listeners
      .lock()
      .expect("listeners mutex is poisoned")
      .clone();

    let state = Arc::new(self.state());
    for listener in listeners.into_values() {
      let state = Arc::clone(&state);

      #[cfg(not(feature = "unstable-async"))]
      async_runtime::spawn_blocking(move || {
        let _ = listener.call(state);
      });

      #[cfg(feature = "unstable-async")]
      async_runtime::spawn(async move {
        let _ = listener.call(state).await;
      });
    }
  }
}

impl<R: Runtime> fmt::Debug for Store<R> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Store")
      .field("id", &self.id)
      .field("state", &self.state)
      .finish_non_exhaustive()
  }
}

fn store_path<R: Runtime>(app: &AppHandle<R>, id: &str) -> PathBuf {
  app
    .store_collection()
    .path()
    .join(format!("{id}.json"))
}

fn to_bytes<T>(value: &T, pretty: bool) -> Result<Vec<u8>>
where
  T: ?Sized + Serialize,
{
  if pretty {
    serde_json::to_vec_pretty(value).map_err(Into::into)
  } else {
    serde_json::to_vec(value).map_err(Into::into)
  }
}
