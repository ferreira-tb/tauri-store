use crate::Result;
use serde::Deserialize;
use std::path::PathBuf;
use tauri::plugin::PluginHandle;
use tauri::{AppHandle, Runtime};

pub struct Handle<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Handle<R> {
  pub fn new(handle: PluginHandle<R>) -> Self {
    Self(handle)
  }

  pub fn app(&self) -> &AppHandle<R> {
    self.0.app()
  }

  pub fn get_sandboxed_path(&self) -> Result<PathBuf> {
    self
      .0
      .run_mobile_plugin::<GetSandboxedPathResponse>("getAppSandboxPath", ())
      .map(|response| response.path)
      .map_err(Into::into)
  }
}

#[derive(Deserialize)]
struct GetSandboxedPathResponse {
  path: PathBuf,
}
