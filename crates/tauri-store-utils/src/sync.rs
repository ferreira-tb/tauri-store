use std::sync::Mutex;

/// Naive `Mutex<Option<T>>` wrapper.
pub struct MutexOption<T> {
  inner: Mutex<Option<T>>,
}

impl<T> MutexOption<T> {
  pub fn new(value: T) -> Self {
    Self { inner: Mutex::new(Some(value)) }
  }

  pub fn map<U>(&self, f: impl FnOnce(&T) -> U) -> Option<U> {
    self.inner.lock().unwrap().as_ref().map(f)
  }

  pub fn replace(&self, value: T) -> Option<T> {
    self.inner.lock().unwrap().replace(value)
  }

  pub fn take(&self) -> Option<T> {
    self.inner.lock().unwrap().take()
  }
}

impl<T> Default for MutexOption<T> {
  fn default() -> Self {
    Self { inner: Mutex::new(None) }
  }
}
