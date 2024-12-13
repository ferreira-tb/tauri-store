mod resource;
mod save;
mod state;
mod watch;

#[cfg(feature = "unstable-async")]
mod unstable_async;

use crate::error::Result;
use crate::event::{
  emit, ConfigPayload, EventSource, StatePayload, STORE_CONFIG_CHANGE_EVENT,
  STORE_STATE_CHANGE_EVENT,
};
use crate::manager::ManagerExt;
use save::SaveHandle;
pub use save::SaveStrategy;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use std::collections::HashMap;
use std::fmt;
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use tauri::{AppHandle, ResourceId, Runtime};
use watch::Watcher;

pub(crate) use resource::StoreResource;
pub use state::{StoreState, StoreStateExt};
pub use watch::WatcherResult;

#[cfg(not(feature = "unstable-async"))]
use {
  save::{debounce, save_now, throttle},
  tauri::async_runtime::spawn_blocking,
};

#[cfg(feature = "unstable-async")]
use tauri::async_runtime::spawn;

#[cfg(tauri_store_tracing)]
use tracing::{debug, warn};

type ResourceTuple<R> = (ResourceId, Arc<StoreResource<R>>);

pub struct Store<R: Runtime> {
  app: AppHandle<R>,
  pub(crate) id: String,
  pub(crate) state: StoreState,
  pub(crate) watchers: HashMap<u32, Watcher<R>>,
  save_on_change: bool,
  save_strategy: Option<SaveStrategy>,
  debounce_save_handle: OnceLock<SaveHandle<R>>,
  throttle_save_handle: OnceLock<SaveHandle<R>>,
}

impl<R: Runtime> Store<R> {
  fn blocking_load(app: &AppHandle<R>, id: String) -> Result<ResourceTuple<R>> {
    let path = store_path(app, &id);
    let state = match std::fs::read(path) {
      Ok(bytes) => serde_json::from_slice(&bytes)?,
      Err(e) if e.kind() == NotFound => {
        #[cfg(tauri_store_tracing)]
        warn!("store not found: {id}, using default state");

        StoreState::default()
      }
      Err(e) => return Err(e.into()),
    };

    #[cfg(tauri_store_tracing)]
    debug!("store loaded: {id}");

    let store = Self {
      app: app.clone(),
      id,
      state,
      watchers: HashMap::new(),
      save_on_change: false,
      save_strategy: None,
      debounce_save_handle: OnceLock::new(),
      throttle_save_handle: OnceLock::new(),
    };

    Ok(StoreResource::create(app, store))
  }

  /// The id of the store.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Path to the store file.
  pub fn path(&self) -> PathBuf {
    store_path(&self.app, &self.id)
  }

  /// Gets a handle to the application instance.
  pub fn app_handle(&self) -> &AppHandle<R> {
    &self.app
  }

  /// Gets a reference to the store state.
  pub fn state(&self) -> &StoreState {
    &self.state
  }

  /// Tries to parse the store state as an instance of type `T`.
  pub fn try_state<T: DeserializeOwned>(&self) -> Result<T> {
    self.state.parse()
  }

  /// Gets a value from the store.
  pub fn get(&self, key: impl AsRef<str>) -> Option<&Json> {
    self.state.get(key.as_ref())
  }

  /// Gets a value from the store and tries to parse it as an instance of type `T`.
  pub fn try_get<T: DeserializeOwned>(&self, key: impl AsRef<str>) -> Result<T> {
    self.state.try_get(key)
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

  /// Whether to save the store on state change.
  pub fn save_on_change(&mut self, enabled: bool) {
    self.save_on_change = enabled;
  }

  /// Current save strategy used by this store.
  pub fn save_strategy(&self) -> SaveStrategy {
    self
      .save_strategy
      .unwrap_or_else(|| self.app.store_collection().default_save_strategy)
  }

  /// Sets the save strategy for this store.
  /// Calling this will abort any pending save operation.
  pub fn set_save_strategy(&mut self, strategy: SaveStrategy) {
    if strategy.is_debounce() {
      self
        .debounce_save_handle
        .take()
        .inspect(SaveHandle::abort);
    } else if strategy.is_throttle() {
      self
        .throttle_save_handle
        .take()
        .inspect(SaveHandle::abort);
    }

    self.save_strategy = Some(strategy);
  }

  /// Sets the store options, optionally having a window as the source.
  #[expect(
    clippy::needless_pass_by_value,
    reason = "We are just anticipating the need for it."
  )]
  pub fn set_options_with_source<S>(&mut self, options: StoreOptions, source: S) -> Result<()>
  where
    S: Into<EventSource>,
  {
    if let Some(strategy) = options.save_strategy {
      self.set_save_strategy(strategy);
    }

    if let Some(enabled) = options.save_on_change {
      self.save_on_change = enabled;
    }

    self.on_config_change(source)
  }

  /// Sets the store options.
  pub fn set_options(&mut self, options: StoreOptions) -> Result<()> {
    self.set_options_with_source(options, None::<&str>)
  }

  /// Watches the store for changes.
  pub fn watch<F>(&mut self, f: F) -> u32
  where
    F: Fn(AppHandle<R>) -> WatcherResult + Send + Sync + 'static,
  {
    let listener = Watcher::new(f);
    let id = listener.id;
    self.watchers.insert(id, listener);
    id
  }

  /// Removes a listener from this store.
  pub fn unwatch(&mut self, id: u32) -> bool {
    self.watchers.remove(&id).is_some()
  }

  fn on_config_change(&self, source: impl Into<EventSource>) -> Result<()> {
    self.emit_config_change(source)
  }

  fn emit_config_change(&self, source: impl Into<EventSource>) -> Result<()> {
    emit(
      &self.app,
      STORE_CONFIG_CHANGE_EVENT,
      &ConfigPayload::from(self),
      source,
    )
  }

  fn emit_state_change(&self, source: impl Into<EventSource>) -> Result<()> {
    let source: EventSource = source.into();

    // If we also skip the store when the source is the backend,
    // the window where the store resides would never know about the change.
    if !source.is_backend()
      && self
        .app
        .store_collection()
        .sync_denylist
        .as_ref()
        .is_some_and(|it| it.contains(&self.id))
    {
      return Ok(());
    }

    emit(
      &self.app,
      STORE_STATE_CHANGE_EVENT,
      &StatePayload::from(self),
      source,
    )
  }

  /// Calls all watchers currently attached to the store.
  fn call_watchers(&self) {
    let watchers = self
      .watchers
      .values()
      .cloned()
      .collect::<Vec<_>>();

    for watcher in watchers {
      let app = self.app.clone();

      #[cfg(feature = "unstable-async")]
      spawn(async move { watcher.call(app).await });
      #[cfg(not(feature = "unstable-async"))]
      spawn_blocking(move || watcher.call(app));
    }
  }

  pub(crate) fn abort_pending_save(&self) {
    if let Some(debounce_save_handle) = self.debounce_save_handle.get() {
      debounce_save_handle.abort();
    }

    if let Some(throttle_save_handle) = self.throttle_save_handle.get() {
      throttle_save_handle.abort();
    }
  }
}

#[cfg(not(feature = "unstable-async"))]
impl<R: Runtime> Store<R> {
  pub(crate) fn load(app: &AppHandle<R>, id: impl AsRef<str>) -> Result<ResourceTuple<R>> {
    Self::blocking_load(app, id.as_ref().to_owned())
  }

  /// Sets a key-value pair in the store.
  pub fn set(&mut self, key: impl AsRef<str>, value: Json) -> Result<()> {
    self.state.insert(key.as_ref().to_owned(), value);
    self.on_state_change(None)
  }

  /// Patches the store state, optionally having a window as the source.
  pub fn patch_with_source<S>(&mut self, state: StoreState, source: S) -> Result<()>
  where
    S: Into<EventSource>,
  {
    self.state.extend(state);
    self.on_state_change(source)
  }

  /// Patches the store state.
  pub fn patch(&mut self, state: StoreState) -> Result<()> {
    self.patch_with_source(state, None)
  }

  fn on_state_change(&self, source: impl Into<EventSource>) -> Result<()> {
    self.emit_state_change(source)?;
    self.call_watchers();

    if self.save_on_change {
      self.save()?;
    }

    Ok(())
  }

  /// Save the store state to the disk.
  pub fn save(&self) -> Result<()> {
    match self.save_strategy() {
      SaveStrategy::Immediate => self.save_now()?,
      SaveStrategy::Debounce(duration) => {
        self
          .debounce_save_handle
          .get_or_init(|| debounce(duration, Arc::from(self.id.as_str())))
          .call(&self.app);
      }
      SaveStrategy::Throttle(duration) => {
        self
          .throttle_save_handle
          .get_or_init(|| throttle(duration, Arc::from(self.id.as_str())))
          .call(&self.app);
      }
    }

    Ok(())
  }

  /// Save the store immediately, ignoring the save strategy.
  pub fn save_now(&self) -> Result<()> {
    save_now(self)
  }
}

impl<R: Runtime> fmt::Debug for Store<R> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Store")
      .field("id", &self.id)
      .field("state", &self.state)
      .field("save_on_change", &self.save_on_change)
      .field("save_strategy", &self.save_strategy)
      .finish_non_exhaustive()
  }
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreOptions {
  pub save_strategy: Option<SaveStrategy>,
  pub save_on_change: Option<bool>,
}

impl<R: Runtime> From<&Store<R>> for StoreOptions {
  fn from(store: &Store<R>) -> Self {
    Self {
      save_strategy: store.save_strategy,
      save_on_change: Some(store.save_on_change),
    }
  }
}

fn store_path<R: Runtime>(app: &AppHandle<R>, id: &str) -> PathBuf {
  app
    .store_collection()
    .path()
    .join(format!("{id}.json"))
}
