use super::Store;
use crate::error::Result;
use crate::manager::ManagerExt;
use futures::future::BoxFuture;
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as Json;
use std::fmt;
use std::result::Result as StdResult;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Runtime};
use tauri_store_utils::{Debounce, RemoteCallable, Throttle};

#[cfg(not(feature = "unstable-async"))]
use {futures::future::FutureExt, tauri::async_runtime::spawn_blocking};

#[cfg(tauri_store_tracing)]
use tracing::debug;

type RemoteSaveHandle<R> = Box<dyn RemoteCallable<AppHandle<R>> + Send + Sync>;
type SaveHandleFn<R> = Box<dyn Fn(AppHandle<R>) -> BoxFuture<'static, ()> + Send + Sync + 'static>;

pub(super) struct SaveHandle<R: Runtime>(RemoteSaveHandle<R>);

impl<R: Runtime> SaveHandle<R> {
  pub fn call(&self, app: &AppHandle<R>) {
    self.0.call(app);
  }

  pub fn abort(&self) {
    self.0.abort();
  }
}

#[cfg(not(feature = "unstable-async"))]
pub(super) fn save_now<R: Runtime>(store: &Store<R>) -> Result<()> {
  use std::fs::{self, File};
  use std::io::Write;

  let collection = store.app.store_collection();
  if collection
    .save_denylist
    .as_ref()
    .is_some_and(|it| it.contains(&store.id))
  {
    return Ok(());
  }

  fs::create_dir_all(collection.path())?;

  let bytes = to_bytes(&store.state, collection.pretty)?;
  let mut file = File::create(store.path())?;
  file.write_all(&bytes)?;

  #[cfg(feature = "file-sync-all")]
  file.sync_all()?;

  #[cfg(tauri_store_tracing)]
  debug!("store saved: {}", store.id);

  Ok(())
}

#[cfg(feature = "unstable-async")]
pub async fn save_now<R: Runtime>(store: &Store<R>) -> Result<()> {
  use tokio::fs::{self, File};
  use tokio::io::AsyncWriteExt;

  let collection = store.app.store_collection();
  if collection
    .save_denylist
    .as_ref()
    .is_some_and(|it| it.contains(&store.id))
  {
    return Ok(());
  }

  fs::create_dir_all(collection.path()).await?;

  let bytes = to_bytes(&store.state, collection.pretty)?;
  let mut file = File::create(store.path()).await?;
  file.write_all(&bytes).await?;
  file.flush().await?;

  #[cfg(feature = "file-sync-all")]
  file.sync_all().await?;

  #[cfg(tauri_store_tracing)]
  debug!("store saved: {}", store.id);

  Ok(())
}

fn to_bytes<T>(value: &T, pretty: bool) -> Result<Vec<u8>>
where
  T: ?Sized + Serialize,
{
  if pretty {
    Ok(serde_json::to_vec_pretty(value)?)
  } else {
    Ok(serde_json::to_vec(value)?)
  }
}

/// The strategy to use when saving a store.
///
/// For a detailed explanation of the differences between debouncing and throttling,
/// take a look at [this article](https://kettanaito.com/blog/debounce-vs-throttle).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default)]
pub enum SaveStrategy {
  #[default]
  Immediate,
  Debounce(Duration),
  Throttle(Duration),
}

impl SaveStrategy {
  const IMMEDIATE: &'static str = "immediate";
  const DEBOUNCE: &'static str = "debounce";
  const THROTTLE: &'static str = "throttle";

  /// Returns [`Debounce`](SaveStrategy::Debounce) with the given duration, in milliseconds.
  pub const fn debounce_millis(millis: u64) -> Self {
    Self::Debounce(Duration::from_millis(millis))
  }

  /// Returns [`Debounce`](SaveStrategy::Debounce) with the given duration, in seconds.
  pub const fn debounce_secs(secs: u64) -> Self {
    Self::Debounce(Duration::from_secs(secs))
  }

  /// Returns [`Throttle`](SaveStrategy::Throttle) with the given duration, in milliseconds.
  pub const fn throttle_millis(millis: u64) -> Self {
    Self::Throttle(Duration::from_millis(millis))
  }

  /// Returns [`Throttle`](SaveStrategy::Throttle) with the given duration, in seconds.
  pub const fn throttle_secs(secs: u64) -> Self {
    Self::Throttle(Duration::from_secs(secs))
  }

  /// Whether the strategy is [`Debounce`](SaveStrategy::Debounce).
  pub const fn is_debounce(&self) -> bool {
    matches!(self, Self::Debounce(_))
  }

  /// Whether the strategy is [`Throttle`](SaveStrategy::Throttle).
  pub const fn is_throttle(&self) -> bool {
    matches!(self, Self::Throttle(_))
  }
}

impl fmt::Display for SaveStrategy {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Immediate => write!(f, "{}", Self::IMMEDIATE),
      Self::Debounce(_) => write!(f, "{}", Self::DEBOUNCE),
      Self::Throttle(_) => write!(f, "{}", Self::THROTTLE),
    }
  }
}

impl Serialize for SaveStrategy {
  fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let interval = match self {
      Self::Debounce(duration) | Self::Throttle(duration) => duration.as_millis(),
      Self::Immediate => 0,
    };

    let mut tuple = serializer.serialize_tuple(2)?;
    tuple.serialize_element(&self.to_string())?;
    tuple.serialize_element(&interval.to_string())?;
    tuple.end()
  }
}

impl<'de> Deserialize<'de> for SaveStrategy {
  fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let err = || {
      use serde::de::Error;
      D::Error::custom("invalid save strategy")
    };

    let value = Json::deserialize(deserializer)?;
    if let Json::Array(mut array) = value {
      if array.len() != 2 {
        return Err(err());
      }

      let strategy = array
        .remove(0)
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(err)?;

      let duration = array
        .remove(0)
        .as_str()
        .map(str::parse)
        .ok_or_else(err)?
        .map(Duration::from_millis)
        .map_err(|_| err())?;

      if duration.is_zero() {
        return Ok(Self::Immediate);
      }

      match strategy.as_str() {
        Self::DEBOUNCE => Ok(Self::Debounce(duration)),
        Self::THROTTLE => Ok(Self::Throttle(duration)),
        Self::IMMEDIATE => Ok(Self::Immediate),
        _ => Err(err()),
      }
    } else {
      Err(err())
    }
  }
}

pub(super) fn debounce<R: Runtime>(duration: Duration, id: Arc<str>) -> SaveHandle<R> {
  SaveHandle(Box::new(Debounce::new(duration, save_handle(id))))
}

pub(super) fn throttle<R: Runtime>(duration: Duration, id: Arc<str>) -> SaveHandle<R> {
  SaveHandle(Box::new(Throttle::new(duration, save_handle(id))))
}

#[cfg(not(feature = "unstable-async"))]
fn save_handle<R: Runtime>(id: Arc<str>) -> SaveHandleFn<R> {
  Box::new(move |app| {
    let id = Arc::clone(&id);
    Box::pin(async move {
      let task = spawn_blocking(move || {
        app
          .store_collection()
          .get_resource(&id)?
          .locked(|store| store.save_now())
      });

      task.map(drop).await;
    })
  })
}

#[cfg(feature = "unstable-async")]
fn save_handle<R: Runtime>(id: Arc<str>) -> SaveHandleFn<R> {
  Box::new(move |app| {
    let id = Arc::clone(&id);
    Box::pin(async move {
      if let Ok(resource) = app.store_collection().get_resource(&id).await {
        let store = resource.inner.lock().await;
        let _ = store.save_now().await;
      }
    })
  })
}
