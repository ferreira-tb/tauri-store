use super::StoreId;
use crate::manager::ManagerExt;
use crate::CollectionMarker;
use futures::future::BoxFuture;
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt;
use std::result::Result as StdResult;
use std::time::Duration;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Runtime};
use tauri_store_utils::{Debounce, RemoteCallable, Throttle};

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

impl<R: Runtime> Drop for SaveHandle<R> {
  fn drop(&mut self) {
    self.abort();
  }
}

pub(super) fn debounce<R, C>(id: StoreId, duration: Duration) -> SaveHandle<R>
where
  R: Runtime,
  C: CollectionMarker,
{
  SaveHandle(Box::new(Debounce::new(duration, save_handle::<R, C>(id))))
}

pub(super) fn throttle<R, C>(id: StoreId, duration: Duration) -> SaveHandle<R>
where
  R: Runtime,
  C: CollectionMarker,
{
  SaveHandle(Box::new(Throttle::new(duration, save_handle::<R, C>(id))))
}

fn save_handle<R, C>(id: StoreId) -> SaveHandleFn<R>
where
  R: Runtime,
  C: CollectionMarker,
{
  Box::new(move |app| {
    let id = id.clone();
    Box::pin(async move {
      let task = spawn_blocking(move || {
        app
          .store_collection_with_marker::<C>()
          .get_resource(&id)?
          .locked(|store| store.save_now())
      });

      let _ = task.await;
    })
  })
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
  #[inline]
  pub const fn debounce_millis(millis: u64) -> Self {
    Self::Debounce(Duration::from_millis(millis))
  }

  /// Returns [`Debounce`](SaveStrategy::Debounce) with the given duration, in seconds.
  #[inline]
  pub const fn debounce_secs(secs: u64) -> Self {
    Self::Debounce(Duration::from_secs(secs))
  }

  /// Returns [`Throttle`](SaveStrategy::Throttle) with the given duration, in milliseconds.
  #[inline]
  pub const fn throttle_millis(millis: u64) -> Self {
    Self::Throttle(Duration::from_millis(millis))
  }

  /// Returns [`Throttle`](SaveStrategy::Throttle) with the given duration, in seconds.
  #[inline]
  pub const fn throttle_secs(secs: u64) -> Self {
    Self::Throttle(Duration::from_secs(secs))
  }

  /// Whether the strategy is [`Debounce`](SaveStrategy::Debounce).
  #[inline]
  pub const fn is_debounce(&self) -> bool {
    matches!(self, Self::Debounce(_))
  }

  /// Whether the strategy is [`Throttle`](SaveStrategy::Throttle).
  #[inline]
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
      Self::Immediate => 0,
      Self::Debounce(duration) | Self::Throttle(duration) => duration.as_millis(),
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

    let value = Value::deserialize(deserializer)?;
    if let Value::Array(mut array) = value {
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
