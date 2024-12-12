use crate::sync::AtomicOption;
use std::ops::Deref;
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
pub(crate) struct OptionalAbortHandle(AtomicOption<AbortHandle>);

impl OptionalAbortHandle {
  pub(crate) fn abort(&self) {
    if let Some(handle) = self.take() {
      handle.abort();
    }
  }
}

impl Deref for OptionalAbortHandle {
  type Target = AtomicOption<AbortHandle>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
