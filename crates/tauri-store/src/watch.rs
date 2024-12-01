use crate::error::Result;
use std::fmt;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Runtime};

#[cfg(feature = "unstable-async")]
use futures::future::BoxFuture;

static WATCHER_ID: AtomicU32 = AtomicU32::new(0);

#[cfg(not(feature = "unstable-async"))]
pub type WatcherResult = Result<()>;
#[cfg(feature = "unstable-async")]
pub type WatcherResult = BoxFuture<'static, Result<()>>;

type WatcherFn<R> = dyn Fn(AppHandle<R>) -> WatcherResult + Send + Sync;

pub(crate) struct Watcher<R: Runtime> {
  pub(crate) id: u32,
  inner: Arc<WatcherFn<R>>,
}

impl<R: Runtime> Watcher<R> {
  pub(crate) fn new<F>(f: F) -> Self
  where
    F: Fn(AppHandle<R>) -> WatcherResult + Send + Sync + 'static,
  {
    Self {
      id: WATCHER_ID.fetch_add(1, Ordering::Relaxed),
      inner: Arc::new(f),
    }
  }

  #[cfg(not(feature = "unstable-async"))]
  pub(crate) fn call(&self, app: AppHandle<R>) -> Result<()> {
    (self.inner)(app)
  }

  #[cfg(feature = "unstable-async")]
  pub(crate) async fn call(&self, app: AppHandle<R>) -> Result<()> {
    (self.inner)(app).await
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
