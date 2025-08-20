#[cfg(any(target_os = "android", target_os = "ios"))]
use crate::Result;
#[cfg(any(target_os = "android", target_os = "ios"))]
use serde::Deserialize;
#[cfg(any(target_os = "android", target_os = "ios"))]
use std::path::PathBuf;
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

  pub fn get_sandboxed_path(&self) -> Result<PathBuf> {
      println!("Store collection path: just before the sandboxed path retrieval");
    self
      .0
      .run_mobile_plugin::<GetSandboxedPathResponse>("getAppSandboxPath", ())
      .map(|r| r.path)
      .map_err(Into::into)
  }
}
#[cfg(any(target_os = "android", target_os = "ios"))]
#[derive(Deserialize)]
struct GetSandboxedPathResponse {
  path: PathBuf,
}
