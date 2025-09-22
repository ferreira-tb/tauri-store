use tauri::{AppHandle, Runtime};

/// Plugin handle.
pub struct Handle<R: Runtime>(AppHandle<R>);

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
impl<R: Runtime> Handle<R> {
  pub fn new(handle: AppHandle<R>) -> Self {
    Self(handle)
  }

  pub fn app(&self) -> &AppHandle<R> {
    &self.0
  }
}
