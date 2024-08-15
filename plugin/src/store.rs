use crate::error::Result;
use crate::ManagerExt;
use serde::Serialize;
use serde_json::Value as Json;
use std::path::PathBuf;
use std::{fmt, io};
use tauri::{AppHandle, Emitter, EventTarget, Runtime};

#[cfg(feature = "ahash")]
use ahash::HashMap;
#[cfg(not(feature = "ahash"))]
use std::collections::HashMap;

pub type StoreState = HashMap<String, Json>;

const CHANGE_EVENT: &str = "pinia://change";

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
      Err(e) if e.kind() == io::ErrorKind::NotFound => {
        #[cfg(feature = "tracing")]
        tracing::warn!("pinia store not found: {id}, using default state");
        StoreState::default()
      }
      Err(e) => return Err(e.into()),
    };

    #[cfg(feature = "tracing")]
    tracing::trace!("pinia store loaded: {id}");

    Ok(Self { id, state, app })
  }

  /// Save the store state to the disk.
  #[cfg(not(feature = "async-pinia"))]
  pub fn save(&self) -> Result<()> {
    use std::fs::{self, File};
    use std::io::Write;

    let pinia = self.app.pinia();
    fs::create_dir_all(pinia.path())?;

    let bytes = serde_json::to_vec(&self.state)?;
    let mut file = File::create(self.path())?;
    file.write_all(&bytes)?;

    Ok(())
  }

  /// Save the store state to the disk.
  #[cfg(feature = "async-pinia")]
  pub async fn save(&self) -> Result<()> {
    use tokio::fs::{self, File};
    use tokio::io::AsyncWriteExt;

    let pinia = self.app.pinia();
    fs::create_dir_all(pinia.path()).await?;

    let path = pinia.path().join(format!("{}.json", self.id));

    let bytes = serde_json::to_vec(&self.state)?;
    let mut file = File::create(path).await?;
    file.write_all(&bytes).await?;

    Ok(())
  }

  pub fn id(&self) -> &str {
    &self.id
  }

  /// Path to the store file.
  pub fn path(&self) -> PathBuf {
    store_path(&self.app, &self.id)
  }

  /// Patches the store state, optionally having a window as the source.
  pub(crate) fn patch_with_source<'a, S>(&mut self, state: StoreState, source: S) -> Result<()>
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

  /// Sets a key-value pair in the store.
  pub fn set(&mut self, key: impl AsRef<str>, value: Json) -> Result<()> {
    self.state.insert(key.as_ref().to_owned(), value);
    self.emit(None)
  }

  /// Gets a value from the store.
  pub fn get(&self, key: impl AsRef<str>) -> Option<&Json> {
    self.state.get(key.as_ref())
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
    let pinia = self.app.pinia();
    if pinia.sync_denylist.contains(&self.id) {
      #[cfg(feature = "tracing")]
      tracing::trace!("store {} is in the denylist, skipping emit", self.id);

      return Ok(());
    }

    let payload = Payload::from(self);
    let source: Option<&str> = source.into();
    if let Some(source) = source {
      payload.emit_filter(&self.app, source)
    } else {
      payload.emit_all(&self.app)
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

#[derive(Clone, Debug, Serialize)]
struct Payload<'a> {
  id: &'a str,
  state: &'a StoreState,
}

impl<'a> Payload<'a> {
  fn emit_all<R: Runtime>(&self, app: &AppHandle<R>) -> Result<()> {
    #[cfg(feature = "tracing")]
    tracing::trace!(event = CHANGE_EVENT, payload = ?self);

    app.emit_filter(CHANGE_EVENT, self, |target| {
      matches!(target, EventTarget::WebviewWindow { .. })
    })?;

    Ok(())
  }

  fn emit_filter<R: Runtime>(&self, app: &AppHandle<R>, source: &str) -> Result<()> {
    #[cfg(feature = "tracing")]
    tracing::trace!(event = CHANGE_EVENT, source, payload = ?self);

    app.emit_filter(CHANGE_EVENT, self, |target| match target {
      EventTarget::WebviewWindow { label } => label != source,
      _ => false,
    })?;

    Ok(())
  }
}

impl<'a, R: Runtime> From<&'a Store<R>> for Payload<'a> {
  fn from(store: &'a Store<R>) -> Self {
    Self { id: &store.id, state: &store.state }
  }
}

fn store_path<R: Runtime>(app: &AppHandle<R>, id: &str) -> PathBuf {
  app.pinia().path().join(format!("{id}.json"))
}
