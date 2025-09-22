mod id;
mod options;
mod resource;
mod save;
mod state;
mod watch;

use crate::collection::CollectionMarker;
use crate::error::Result;
use crate::manager::ManagerExt;
use options::set_options;
use save::{debounce, throttle, SaveHandle};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, ResourceId, Runtime};
use tauri_store_utils::{read_file, write_file};
use watch::Watcher;

use crate::event::{
  emit, ConfigPayload, EventSource, StatePayload, STORE_CONFIG_CHANGE_EVENT,
  STORE_STATE_CHANGE_EVENT,
};

pub use id::StoreId;
pub use options::StoreOptions;
pub(crate) use resource::StoreResource;
pub use save::SaveStrategy;
pub use state::StoreState;
pub use watch::WatcherId;

#[cfg(debug_assertions)]
const FILE_EXTENSION: &str = "dev.json";
#[cfg(not(debug_assertions))]
const FILE_EXTENSION: &str = "json";

type ResourceTuple<R, C> = (ResourceId, Arc<StoreResource<R, C>>);

/// A key-value store that can persist its state to disk.
pub struct Store<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
  app: AppHandle<R>,
  pub(crate) id: StoreId,
  state: StoreState,
  pub(crate) save_on_exit: bool,
  save_on_change: bool,
  save_strategy: Option<SaveStrategy>,
  debounce_save_handle: OnceLock<SaveHandle<R>>,
  throttle_save_handle: OnceLock<SaveHandle<R>>,
  watchers: HashMap<WatcherId, Watcher<R>>,
  phantom: PhantomData<C>,
}

impl<R, C> Store<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
  pub(crate) fn load(app: &AppHandle<R>, id: impl AsRef<str>) -> Result<ResourceTuple<R, C>> {
    let id = StoreId::from(id.as_ref());
    let path = store_path::<R, C>(app, &id);
    let state = read_file(&path).call()?;

    #[allow(unused_mut)]
    let mut store = Self {
      app: app.clone(),
      id,
      state,
      save_on_change: false,
      save_on_exit: true,
      save_strategy: None,
      debounce_save_handle: OnceLock::new(),
      throttle_save_handle: OnceLock::new(),
      watchers: HashMap::new(),
      phantom: PhantomData,
    };

    #[cfg(feature = "unstable-migration")]
    store.run_pending_migrations(app)?;

    Ok(StoreResource::create(app, store))
  }

  #[cfg(feature = "unstable-migration")]
  fn run_pending_migrations(&mut self, app: &AppHandle<R>) -> Result<()> {
    use crate::meta::Meta;

    let collection = app.store_collection_with_marker::<C>();
    let result = collection
      .migrator
      .lock()
      .expect("migrator is poisoned")
      .migrate(&self.id, &mut self.state);

    if result.done > 0 {
      Meta::write(&collection)?;
    }

    if let Some(err) = result.error {
      Err(err)
    } else {
      Ok(())
    }
  }

  /// The id of the store.
  #[inline]
  pub fn id(&self) -> StoreId {
    self.id.clone()
  }

  /// Path to the store file.
  pub fn path(&self) -> PathBuf {
    store_path::<R, C>(&self.app, &self.id)
  }

  /// Gets a handle to the application instance.
  pub fn app_handle(&self) -> &AppHandle<R> {
    &self.app
  }

  /// Gets a reference to the raw store state.
  #[inline]
  pub fn raw_state(&self) -> &StoreState {
    &self.state
  }

  /// Tries to parse the store state as an instance of type `T`.
  pub fn state<T>(&self) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let value = Value::from(&self.state);
    Ok(serde_json::from_value(value)?)
  }

  /// Tries to parse the store state as an instance of type `T`.
  ///
  /// If it cannot be parsed, returns the provided default value.
  pub fn state_or<T>(&self, default: T) -> T
  where
    T: DeserializeOwned,
  {
    self.state().unwrap_or(default)
  }

  /// Tries to parse the store state as an instance of type `T`.
  ///
  /// If it cannot be parsed, returns the default value of `T`.
  pub fn state_or_default<T>(&self) -> T
  where
    T: DeserializeOwned + Default,
  {
    self.state().unwrap_or_default()
  }

  /// Tries to parse the store state as an instance of type `T`.
  ///
  /// If it cannot be parsed, returns the result of the provided closure.
  pub fn state_or_else<T>(&self, f: impl FnOnce() -> T) -> T
  where
    T: DeserializeOwned,
  {
    self.state().unwrap_or_else(|_| f())
  }

  /// Gets a raw value from the store.
  pub fn get_raw(&self, key: impl AsRef<str>) -> Option<&Value> {
    self.state.get_raw(key)
  }

  /// Gets a value from the store and tries to parse it as an instance of type `T`.
  pub fn get<T>(&self, key: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    self.state.get(key)
  }

  /// Gets a value from the store and tries to parse it as an instance of type `T`.
  ///
  /// If the key does not exist, returns the provided default value.
  pub fn get_or<T>(&self, key: impl AsRef<str>, default: T) -> T
  where
    T: DeserializeOwned,
  {
    self.state.get_or(key, default)
  }

  /// Gets a value from the store and tries to parse it as an instance of type `T`.
  ///
  /// If the key does not exist, returns the default value of `T`.
  pub fn get_or_default<T>(&self, key: impl AsRef<str>) -> T
  where
    T: DeserializeOwned + Default,
  {
    self.state.get_or_default(key)
  }

  /// Gets a value from the store and tries to parse it as an instance of type `T`.
  ///
  /// If the key does not exist, returns the result of the provided closure.
  pub fn get_or_else<T>(&self, key: impl AsRef<str>, f: impl FnOnce() -> T) -> T
  where
    T: DeserializeOwned,
  {
    self.state.get_or_else(key, f)
  }

  /// Sets a key-value pair in the store.
  pub fn set(&mut self, key: impl AsRef<str>, value: impl Into<Value>) -> Result<()> {
    self.state.set(key, value);
    self.on_state_change(None::<&str>)
  }

  /// Patches the store state, optionally having a window as the source.
  #[doc(hidden)]
  pub fn patch_with_source<S, E>(&mut self, state: S, source: E) -> Result<()>
  where
    S: Into<StoreState>,
    E: Into<EventSource>,
  {
    self.state.patch(state);
    self.on_state_change(source)
  }

  /// Patches the store state.
  pub fn patch<S>(&mut self, state: S) -> Result<()>
  where
    S: Into<StoreState>,
  {
    self.patch_with_source(state, None::<&str>)
  }

  /// Whether the store has a key.
  pub fn has(&self, key: impl AsRef<str>) -> bool {
    self.state.has(key)
  }

  /// Creates an iterator over the store keys.
  pub fn keys(&self) -> impl Iterator<Item = &String> {
    self.state.keys()
  }

  /// Creates an iterator over the store values.
  pub fn values(&self) -> impl Iterator<Item = &Value> {
    self.state.values()
  }

  /// Creates an iterator over the store entries.
  pub fn entries(&self) -> impl Iterator<Item = (&String, &Value)> {
    self.state.entries()
  }

  /// Returns the amount of items in the store.
  #[inline]
  pub fn len(&self) -> usize {
    self.state.len()
  }

  /// Whether the store is empty.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.state.is_empty()
  }

  /// Save the store state to the disk.
  pub fn save(&self) -> Result<()> {
    match self.save_strategy() {
      SaveStrategy::Immediate => self.save_now()?,
      SaveStrategy::Debounce(duration) => {
        self
          .debounce_save_handle
          .get_or_init(|| debounce::<R, C>(self.id.clone(), duration))
          .call(&self.app);
      }
      SaveStrategy::Throttle(duration) => {
        self
          .throttle_save_handle
          .get_or_init(|| throttle::<R, C>(self.id.clone(), duration))
          .call(&self.app);
      }
    }

    Ok(())
  }

  /// Save the store immediately, ignoring the save strategy.
  pub fn save_now(&self) -> Result<()> {
    let collection = self.app.store_collection_with_marker::<C>();
    if collection.save_denylist.contains(&self.id) {
      return Ok(());
    }

    write_file(self.path(), &self.state)
      .sync(cfg!(feature = "file-sync-all"))
      .call()?;

    Ok(())
  }

  /// Whether to save the store on exit.
  /// This is enabled by default.
  #[inline]
  pub fn save_on_exit(&mut self, enabled: bool) {
    self.save_on_exit = enabled;
  }

  /// Whether to save the store on state change.
  #[inline]
  pub fn save_on_change(&mut self, enabled: bool) {
    self.save_on_change = enabled;
  }

  /// Current save strategy used by this store.
  pub fn save_strategy(&self) -> SaveStrategy {
    self.save_strategy.unwrap_or_else(|| {
      self
        .app
        .store_collection_with_marker::<C>()
        .default_save_strategy
    })
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
  pub fn watch<F>(&mut self, f: F) -> WatcherId
  where
    F: Fn(AppHandle<R>) -> Result<()> + Send + Sync + 'static,
  {
    let (id, listener) = Watcher::new(f);
    self.watchers.insert(id, listener);
    id
  }

  /// Removes a listener from this store.
  pub fn unwatch(&mut self, id: impl Into<WatcherId>) -> bool {
    self.watchers.remove(&id.into()).is_some()
  }

  /// Sets the store options, optionally having a window as the source.
  #[doc(hidden)]
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
        .store_collection_with_marker::<C>()
        .sync_denylist
        .contains(&self.id)
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

    for watcher in self.watchers.values() {
      let app = self.app.clone();
      let watcher = watcher.clone();
      spawn_blocking(move || watcher.call(app));
    }
  }

  pub(crate) fn abort_pending_save(&self) {
    self
      .debounce_save_handle
      .get()
      .map(SaveHandle::abort);

    self
      .throttle_save_handle
      .get()
      .map(SaveHandle::abort);
  }
}

impl<R, C> fmt::Debug for Store<R, C>
where
  R: Runtime,
  C: CollectionMarker,
{
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

fn store_path<R, C>(app: &AppHandle<R>, id: &StoreId) -> PathBuf
where
  R: Runtime,
  C: CollectionMarker,
{
  append_filename(&app.store_collection_with_marker::<C>().path(), id)
}

/// Appends the store filename to the given directory path.
pub(crate) fn append_filename(path: &Path, id: &StoreId) -> PathBuf {
  path.join(format!("{id}.{FILE_EXTENSION}"))
}
