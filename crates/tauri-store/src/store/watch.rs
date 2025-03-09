use crate::error::Result;
use std::fmt;
use std::ops::Deref;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use tauri::{AppHandle, Runtime};

static CURRENT_ID: AtomicU32 = AtomicU32::new(0);

type WatcherFn<R> = dyn Fn(AppHandle<R>) -> Result<()> + Send + Sync;

pub(crate) struct Watcher<R: Runtime> {
  pub(crate) id: WatcherId,
  inner: Arc<WatcherFn<R>>,
}

impl<R: Runtime> Watcher<R> {
  pub fn new<F>(f: F) -> Self
  where
    F: Fn(AppHandle<R>) -> Result<()> + Send + Sync + 'static,
  {
    Self {
      id: WatcherId(CURRENT_ID.fetch_add(1, Relaxed)),
      inner: Arc::new(f),
    }
  }

  pub fn call(&self, app: AppHandle<R>) {
    let _ = (self.inner)(app);
  }
}

impl<R: Runtime> Clone for Watcher<R> {
  fn clone(&self) -> Self {
    Self {
      id: self.id,
      inner: Arc::clone(&self.inner),
    }
  }
}

impl<R: Runtime> fmt::Debug for Watcher<R> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Listener")
      .field("id", &self.id)
      .finish_non_exhaustive()
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WatcherId(u32);

impl Deref for WatcherId {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<u32> for WatcherId {
  fn from(id: u32) -> Self {
    Self(id)
  }
}
