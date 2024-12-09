use super::Store;
use crate::error::Result;
use crate::manager::ManagerExt;
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as Json;
use std::fmt;
use std::result::Result as StdResult;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Runtime};
use tauri_store_utils::Debounce;
use tauri_store_utils::RemoteCallable;

#[cfg(not(feature = "unstable-async"))]
use {crate::error::Error, futures::future::FutureExt, tauri::async_runtime::spawn_blocking};

#[cfg(tauri_store_tracing)]
use tracing::debug;

type SaveHandleFn<R> = Box<dyn RemoteCallable<AppHandle<R>> + Send + Sync>;

pub(super) struct SaveHandle<R: Runtime>(SaveHandleFn<R>);

impl<R: Runtime> SaveHandle<R> {
  #[inline]
  pub fn call(&self, app: &AppHandle<R>) {
    self.0.call(app);
  }

  #[inline]
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

  #[cfg(tauri_store_tracing)]
  debug!("store saved: {}", store.id);

  Ok(())
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

/// The strategy to use when saving a store.
#[derive(Clone, Copy, Debug, Default)]
pub enum SaveStrategy {
  #[default]
  Immediate,
  Debounce(Duration),
  Throttle(Duration),
}

impl SaveStrategy {
  /// Returns [`Debounce`](SaveStrategy::Debounce) with the given duration, in milliseconds.
  pub fn debounce_millis(millis: u64) -> Self {
    Self::Debounce(Duration::from_millis(millis))
  }

  /// Returns [`Debounce`](SaveStrategy::Debounce) with the given duration, in seconds.
  pub fn debounce_secs(secs: u64) -> Self {
    Self::Debounce(Duration::from_secs(secs))
  }

  /// Returns [`Throttle`](SaveStrategy::Throttle) with the given duration, in milliseconds.
  pub fn throttle_millis(millis: u64) -> Self {
    Self::Throttle(Duration::from_millis(millis))
  }

  /// Returns [`Throttle`](SaveStrategy::Throttle) with the given duration, in seconds.
  pub fn throttle_secs(secs: u64) -> Self {
    Self::Throttle(Duration::from_secs(secs))
  }

  /// Whether the strategy is [`Debounce`](SaveStrategy::Debounce).
  pub fn is_debounce(&self) -> bool {
    matches!(self, Self::Debounce(_))
  }

  /// Whether the strategy is [`Throttle`](SaveStrategy::Throttle).
  pub fn is_throttle(&self) -> bool {
    matches!(self, Self::Throttle(_))
  }
}

impl fmt::Display for SaveStrategy {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Immediate => write!(f, "immediate"),
      Self::Debounce(_) => write!(f, "debounce"),
      Self::Throttle(_) => write!(f, "throttle"),
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
        .as_u64()
        .map(Duration::from_millis)
        .ok_or_else(err)?;

      if duration.is_zero() {
        return Ok(Self::Immediate);
      }

      match strategy.as_str() {
        "debounce" => Ok(Self::Debounce(duration)),
        "throttle" => Ok(Self::Throttle(duration)),
        "immediate" => Ok(Self::Immediate),
        _ => Err(err()),
      }
    } else {
      Err(err())
    }
  }
}

#[cfg(not(feature = "unstable-async"))]
pub(super) fn debounce<R: Runtime>(duration: Duration, id: Arc<str>) -> SaveHandle<R> {
  let debounce = Debounce::new(duration, move |app| {
    let id = Arc::clone(&id);
    let task = spawn_blocking(move || {
      let resource = app.store_collection().get_resource(&id)?;
      if let Ok(store) = resource.inner.lock() {
        store.save_now()?;
      }

      Ok::<_, Error>(())
    });

    task.map(drop)
  });

  SaveHandle(Box::new(debounce))
}

#[cfg(feature = "unstable-async")]
pub(super) fn debounce<R: Runtime>(duration: Duration, id: Arc<str>) -> SaveHandle<R> {
  let debounce = Debounce::new(duration, move |app| {
    let id = Arc::clone(&id);
    async move {
      let resource = app.store_collection().get_resource(&id).await?;
      let store = resource.inner.lock().await;
      store.save_now().await
    }
  });

  SaveHandle(Box::new(debounce))
}
