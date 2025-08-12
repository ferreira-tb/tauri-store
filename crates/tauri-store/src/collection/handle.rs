use tauri::{AppHandle, Runtime};

#[cfg(any(target_os = "android", target_os = "ios"))]
use tauri::plugin::PluginHandle;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
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

#[cfg(any(target_os = "android", target_os = "ios"))]
pub struct Handle<R: Runtime>(PluginHandle<R>);

#[cfg(any(target_os = "android", target_os = "ios"))]
impl<R: Runtime> Handle<R> {
  pub fn new(handle: PluginHandle<R>) -> Self {
    Self(handle)
  }

  pub fn app(&self) -> &AppHandle<R> {
    self.0.app()
  }
}
