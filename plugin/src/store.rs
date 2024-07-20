use crate::error::Result;
use crate::PiniaExt;
use serde::Serialize;
use serde_json::Value as Json;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Write};
use tauri::{AppHandle, Emitter, EventTarget, Runtime};
use tracing::{info, trace, warn};

#[cfg(feature = "ahash")]
use ahash::HashMap;
#[cfg(not(feature = "ahash"))]
use std::collections::HashMap;

pub type State = HashMap<String, Json>;

const CHANGE_EVENT: &str = "pinia://change";

pub struct Store<R: Runtime> {
  pub(crate) id: String,
  pub(crate) state: State,
  app: AppHandle<R>,
}

impl<R: Runtime> Store<R> {
  pub(crate) fn load(app: AppHandle<R>, id: impl AsRef<str>) -> Result<Self> {
    let id = id.as_ref().to_owned();
    let path = app.pinia().path().join(format!("{id}.json"));

    let state = match fs::read(path) {
      Ok(bytes) => serde_json::from_slice(&bytes)?,
      Err(e) if e.kind() == io::ErrorKind::NotFound => {
        warn!("pinia store not found: {id}");
        State::default()
      }
      Err(e) => return Err(e.into()),
    };

    let store = Self { app, id, state };
    info!("pinia store loaded: {}", store.id);

    Ok(store)
  }

  /// Saves the store state to the disk.
  pub fn save(&self) -> Result<()> {
    let pinia = self.app.pinia();
    fs::create_dir_all(pinia.path())?;

    let path = pinia.path().join(format!("{}.json", self.id));

    let bytes = serde_json::to_vec(&self.state)?;
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;

    Ok(())
  }

  /// Patches the store state.
  pub fn patch<'a, S>(&mut self, state: State, source: S) -> Result<()>
  where
    S: Into<Option<&'a str>>,
  {
    self.state.extend(state);
    self.emit(source)
  }

  fn emit<'a, S>(&self, source: S) -> Result<()>
  where
    S: Into<Option<&'a str>>,
  {
    let pinia = self.app.pinia();
    if pinia.sync_denylist.contains(&self.id) {
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
      .finish()
  }
}

#[derive(Clone, Debug, Serialize)]
struct Payload<'a> {
  id: &'a str,
  state: &'a State,
}

impl<'a> Payload<'a> {
  fn emit_all<R: Runtime>(&self, app: &AppHandle<R>) -> Result<()> {
    trace!(event = CHANGE_EVENT, payload = ?self);
    app.emit_filter(CHANGE_EVENT, self, |target| {
      matches!(target, EventTarget::WebviewWindow { .. })
    })?;

    Ok(())
  }

  fn emit_filter<R: Runtime>(&self, app: &AppHandle<R>, source: &str) -> Result<()> {
    trace!(event = CHANGE_EVENT, source, payload = ?self);
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
