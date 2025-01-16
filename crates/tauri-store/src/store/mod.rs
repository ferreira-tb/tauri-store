mod options;
mod resource;
mod save;
mod state;
mod watch;

use crate::error::Result;
use crate::event::{
  emit, ConfigPayload, EventSource, StatePayload, STORE_CONFIG_CHANGE_EVENT,
  STORE_STATE_CHANGE_EVENT,
};
use crate::io_err;
use crate::manager::ManagerExt;
use options::set_options;
use save::{debounce, throttle, to_bytes, SaveHandle};
use serde::de::DeserializeOwned;
use serde_json::{json, Value as Json};
use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File};
use std::io::ErrorKind::NotFound;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, ResourceId, Runtime};
use watch::Watcher;

pub use options::StoreOptions;
pub(crate) use resource::StoreResource;
pub use save::SaveStrategy;
pub use state::StoreState;

#[cfg(tauri_store_tracing)]
use tracing::{debug, warn};

type ResourceTuple<R> = (ResourceId, Arc<StoreResource<R>>);

/// A key-value store that can persist its state to disk.
pub struct Store<R: Runtime> {
  app: AppHandle<R>,
  pub(crate) id: String,
  pub(crate) state: StoreState,
  pub(crate) watchers: HashMap<u32, Watcher<R>>,
  pub(crate) save_on_exit: bool,
  save_on_change: bool,
  save_strategy: Option<SaveStrategy>,
  debounce_save_handle: OnceLock<SaveHandle<R>>,
  throttle_save_handle: OnceLock<SaveHandle<R>>,
}

impl<R: Runtime> Store<R> {
  pub(crate) fn load(app: &AppHandle<R>, id: impl AsRef<str>) -> Result<ResourceTuple<R>> {
    let id = id.as_ref();
    let path = store_path(app, id);
    let state = match fs::read(path) {
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
      id: id.to_owned(),
      state,
      watchers: HashMap::new(),
      save_on_change: false,
      save_on_exit: true,
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
    Ok(serde_json::from_value(json!(self.state))?)
  }

  /// Gets a value from the store.
  pub fn get(&self, key: impl AsRef<str>) -> Option<&Json> {
    self.state.0.get(key.as_ref())
  }

  /// Gets a value from the store and tries to parse it as an instance of type `T`.
  pub fn try_get<T: DeserializeOwned>(&self, key: impl AsRef<str>) -> Result<T> {
    let key = key.as_ref();
    let Some(value) = self.state.0.get(key).cloned() else {
      return io_err!(NotFound, "key not found: {key}");
    };

    Ok(serde_json::from_value(value)?)
  }

  /// Gets a value from the store and tries to parse it as an instance of type `T`.
  ///
  /// If the key does not exist, returns the provided default value.
  pub fn try_get_or<T>(&self, key: impl AsRef<str>, default: T) -> T
  where
    T: DeserializeOwned,
  {
    self.try_get(key).unwrap_or(default)
  }

  /// Gets a value from the store and tries to parse it as an instance of type `T`.
  ///
  /// If the key does not exist, returns the default value of `T`.
  pub fn try_get_or_default<T>(&self, key: impl AsRef<str>) -> T
  where
    T: DeserializeOwned + Default,
  {
    self.try_get(key).unwrap_or_default()
  }

  /// Gets a value from the store and tries to parse it as an instance of type `T`.
  ///
  /// If the key does not exist, returns the result of the provided closure.
  pub fn try_get_or_else<T, F>(&self, key: impl AsRef<str>, f: F) -> T
  where
    T: DeserializeOwned,
    F: FnOnce() -> T,
  {
    self.try_get(key).unwrap_or_else(|_| f())
  }

  /// Sets a key-value pair in the store.
  pub fn set(&mut self, key: impl AsRef<str>, value: impl Into<Json>) -> Result<()> {
    self
      .state
      .0
      .insert(key.as_ref().to_owned(), value.into());

    self.on_state_change(None)
  }

  /// Patches the store state, optionally having a window as the source.
  pub fn patch_with_source<S, E>(&mut self, state: S, source: E) -> Result<()>
  where
    S: Into<StoreState>,
    E: Into<EventSource>,
  {
    self.state.0.extend(state.into().0);
    self.on_state_change(source)
  }

  /// Patches the store state.
  pub fn patch<S: Into<StoreState>>(&mut self, state: S) -> Result<()> {
    self.patch_with_source(state, None)
  }

  /// Whether the store has a key.
  pub fn has(&self, key: impl AsRef<str>) -> bool {
    self.state.0.contains_key(key.as_ref())
  }

  /// Creates an iterator over the store keys.
  pub fn keys(&self) -> impl Iterator<Item = &String> {
    self.state.0.keys()
  }

  /// Creates an iterator over the store values.
  pub fn values(&self) -> impl Iterator<Item = &Json> {
    self.state.0.values()
  }

  /// Creates an iterator over the store entries.
  pub fn entries(&self) -> impl Iterator<Item = (&String, &Json)> {
    self.state.0.iter()
  }

  /// Returns the amount of items in the store.
  pub fn len(&self) -> usize {
    self.state.0.len()
  }

  /// Whether the store is empty.
  pub fn is_empty(&self) -> bool {
    self.state.0.is_empty()
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

    #[cfg(feature = "file-sync-all")]
    file.sync_all()?;

    #[cfg(tauri_store_tracing)]
    debug!("store saved: {}", self.id);

    Ok(())
  }

  /// Whether to save the store on exit.
  /// This is enabled by default.
  pub fn save_on_exit(&mut self, enabled: bool) {
    self.save_on_exit = enabled;
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

  /// Watches the store for changes.
  pub fn watch<F>(&mut self, f: F) -> u32
  where
    F: Fn(AppHandle<R>) -> Result<()> + Send + Sync + 'static,
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

  /// Sets the store options, optionally having a window as the source.
  pub fn set_options_with_source<E>(&mut self, options: StoreOptions, source: E) -> Result<()>
  where
    E: Into<EventSource>,
  {
    set_options(self, options);
    self.on_config_change(source)
  }

  /// Sets the store options.
  pub fn set_options(&mut self, options: StoreOptions) -> Result<()> {
    self.set_options_with_source(options, None::<&str>)
  }

  fn on_state_change(&self, source: impl Into<EventSource>) -> Result<()> {
    self.emit_state_change(source)?;
    self.call_watchers();

    if self.save_on_change {
      self.save()?;
    }

    Ok(())
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

  /// Calls all watchers currently attached to the store.
  fn call_watchers(&self) {
    if self.watchers.is_empty() {
      return;
    }

    let watchers = self
      .watchers
      .values()
      .cloned()
      .collect::<Vec<_>>();

    for watcher in watchers {
      let app = self.app.clone();
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

impl<R: Runtime> fmt::Debug for Store<R> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Store")
      .field("id", &self.id)
      .field("state", &self.state)
      .field("watchers", &self.watchers.len())
      .field("save_on_exit", &self.save_on_exit)
      .field("save_on_change", &self.save_on_change)
      .field("save_strategy", &self.save_strategy)
      .finish_non_exhaustive()
  }
}

fn store_path<R: Runtime>(app: &AppHandle<R>, id: &str) -> PathBuf {
  app
    .store_collection()
    .path()
    .join(format!("{id}.json"))
}
