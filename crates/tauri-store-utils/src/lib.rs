mod manager;
mod sync;
mod time;

pub use manager::ManagerExt;
pub use sync::AtomicOption;
pub use time::{set_interval, Debounce};

/// A dyn compatible trait intended to be used with types like [Debounce](crate::Debounce).
pub trait RemoteCallable<T> {
  /// Call the function with the provided context.
  fn call(&self, ctx: &T);

  /// Abort any pending calls.
  fn abort(&self);
}
