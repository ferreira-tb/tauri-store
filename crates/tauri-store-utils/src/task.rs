use parking_lot::Mutex;
use tokio::task::AbortHandle;

/// A dyn compatible trait intended to be used with types
/// like [`Debounce`](crate::Debounce) and [`Throttle`](crate::Throttle).
pub trait RemoteCallable<T> {
  /// Call the function with the provided context.
  fn call(&self, ctx: &T);
  /// Abort any pending calls.
  fn abort(&self);
}

#[derive(Default)]
pub(crate) struct OptionalAbortHandle {
  inner: Mutex<Option<AbortHandle>>,
}

impl OptionalAbortHandle {
  pub(crate) fn abort(&self) {
    let mut lock = self.inner.lock();
    if let Some(handle) = lock.take() {
      drop(lock);
      handle.abort();
    }
  }

  pub(crate) fn replace(&self, handle: AbortHandle) {
    let mut lock = self.inner.lock();
    if let Some(old) = lock.replace(handle) {
      old.abort();
    }
  }
}
