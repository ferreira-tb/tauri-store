use crate::error::Result;
use crate::event::{Payload, STORE_UPDATED_EVENT};
use crate::io_err;
use crate::manager::ManagerExt;
use serde::de::DeserializeOwned;
use serde_json::Value as Json;
use std::path::PathBuf;
use std::{fmt, io};
use tauri::{AppHandle, Runtime};

#[cfg(feature = "ahash")]
use ahash::HashMap;
#[cfg(not(feature = "ahash"))]
use std::collections::HashMap;

pub type StoreState = HashMap<String, Json>;

pub struct Store<R: Runtime> {
  pub(crate) id: String,
  pub(crate) state: StoreState,
  app: AppHandle<R>,
}

impl<R: Runtime> Store<R> {
  pub(crate) fn load(app: AppHandle<R>, id: impl AsRef<str>) -> Result<Self> {
    let id = id.as_ref().to_owned();
    let path = store_path(&app, &id);

    let state = match std::fs::read(path) {
      Ok(bytes) => serde_json::from_slice(&bytes)?,
      Err(e) if e.kind() == io::ErrorKind::NotFound => StoreState::default(),
      Err(e) => return Err(e.into()),
    };

    Ok(Self { id, state, app })
  }

  /// Save the store state to the disk.
  #[cfg(not(feature = "unstable-async"))]
  pub fn save(&self) -> Result<()> {
    use std::fs::{self, File};
    use std::io::Write;

    let collection = self.app.store_collection();
    fs::create_dir_all(collection.path())?;

    let bytes = serde_json::to_vec(&self.state)?;
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
    fs::create_dir_all(collection.path()).await?;

    let path = store_path(&self.app, &self.id);
    let bytes = serde_json::to_vec(&self.state)?;
    let mut file = File::create(path).await?;
    file.write_all(&bytes).await?;

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
    self.get(key).cloned()
  }

  /// Gets a value from the store and tries to interpret it as an instance of type `T`.
  pub fn try_get<T>(&self, key: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let Some(value) = self.get_owned(key.as_ref()) else {
      return io_err!(NotFound, "key not found: {}", key.as_ref());
    };

    serde_json::from_value(value).map_err(Into::into)
  }

  /// Sets a key-value pair in the store.
  pub fn set(&mut self, key: impl AsRef<str>, value: Json) -> Result<()> {
    self.state.insert(key.as_ref().to_owned(), value);
    self.emit(None)
  }

  /// Patches the store state, optionally having a window as the source.
  pub fn patch_with_source<'a, S>(&mut self, state: StoreState, source: S) -> Result<()>
  where
    S: Into<Option<&'a str>>,
  {
    self.state.extend(state);
    self.emit(source)
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

  fn emit<'a, S>(&self, source: S) -> Result<()>
  where
    S: Into<Option<&'a str>>,
  {
    let source: Option<&str> = source.into();
    let collection = self.app.store_collection();

    let sync_denylist = collection
      .sync_denylist
      .lock()
      .expect("sync denylist mutex is poisoned");

    // If we also skip the store when the source is the backend,
    // the window where the store resides would never know about the change.
    if source.is_some() && sync_denylist.contains(&self.id) {
      return Ok(());
    }

    drop(sync_denylist);

    let payload = Payload::from(self);
    if let Some(source) = source {
      payload.emit_filter(&self.app, STORE_UPDATED_EVENT, |it| it != source)
    } else {
      payload.emit_all(&self.app, STORE_UPDATED_EVENT)
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
