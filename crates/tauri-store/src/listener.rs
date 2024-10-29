use crate::error::Result;
use crate::store::StoreStateArc;
use std::fmt;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

#[cfg(feature = "unstable-async")]
use crate::BoxFuture;

static ID: AtomicU32 = AtomicU32::new(0);

#[cfg(not(feature = "unstable-async"))]
type ListenerFn = dyn Fn(StoreStateArc) -> Result<()> + Send + Sync;
#[cfg(feature = "unstable-async")]
type ListenerFn = dyn Fn(StoreStateArc) -> BoxFuture<'static, Result<()>> + Send + Sync;

pub(crate) struct Listener {
  pub(crate) id: u32,
  inner: Arc<ListenerFn>,
}

impl Listener {
  #[cfg(not(feature = "unstable-async"))]
  pub(crate) fn new<F>(f: F) -> Self
  where
    F: Fn(StoreStateArc) -> Result<()> + Send + Sync + 'static,
  {
    Self {
      id: ID.fetch_add(1, Ordering::Relaxed),
      inner: Arc::new(f),
    }
  }

  #[cfg(feature = "unstable-async")]
  pub(crate) fn new<F>(f: F) -> Self
  where
    F: Fn(StoreStateArc) -> BoxFuture<'static, Result<()>> + Send + Sync + 'static,
  {
    Self {
      id: ID.fetch_add(1, Ordering::Relaxed),
      inner: Arc::new(f),
    }
  }

  #[cfg(not(feature = "unstable-async"))]
  pub(crate) fn call(&self, state: StoreStateArc) -> Result<()> {
    (self.inner)(state)
  }

  #[cfg(feature = "unstable-async")]
  pub(crate) async fn call(&self, state: StoreStateArc) -> Result<()> {
    (self.inner)(state).await
  }
}

impl Clone for Listener {
  fn clone(&self) -> Self {
    Self {
      id: self.id,
      inner: Arc::clone(&self.inner),
    }
  }
}

impl fmt::Debug for Listener {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Listener")
      .field("id", &self.id)
      .finish_non_exhaustive()
  }
}
