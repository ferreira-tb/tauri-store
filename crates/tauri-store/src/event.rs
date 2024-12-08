use crate::error::Result;
use crate::store::{Store, StoreState};
use serde::Serialize;
use tauri::{AppHandle, Emitter as _, EventTarget, Runtime};

pub const STORE_UPDATED_EVENT: &str = "tauri-store://updated";
pub const STORE_UNLOADED_EVENT: &str = "tauri-store://unloaded";

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Payload<'a> {
  id: &'a str,
  state: &'a StoreState,
}

impl Payload<'_> {
  pub fn emit_all<R: Runtime>(&self, app: &AppHandle<R>, event: &str) -> Result<()> {
    emit_all(app, event, self)
  }

  pub fn emit_filter<R, F>(&self, app: &AppHandle<R>, event: &str, f: F) -> Result<()>
  where
    R: Runtime,
    F: Fn(&str) -> bool,
  {
    emit_filter(app, event, self, f)
  }
}

impl<'a, R: Runtime> From<&'a Store<R>> for Payload<'a> {
  fn from(store: &'a Store<R>) -> Self {
    Self { id: &store.id, state: &store.state }
  }
}

pub(crate) fn emit_all<R, T>(app: &AppHandle<R>, event: &str, payload: &T) -> Result<()>
where
  R: Runtime,
  T: Serialize + ?Sized,
{
  app.emit_filter(event, payload, |target| {
    matches!(target, EventTarget::WebviewWindow { .. })
  })?;

  Ok(())
}

pub(crate) fn emit_filter<R, T, F>(app: &AppHandle<R>, event: &str, payload: &T, f: F) -> Result<()>
where
  R: Runtime,
  T: Serialize + ?Sized,
  F: Fn(&str) -> bool,
{
  #[rustfmt::skip]
  app.emit_filter(event, payload, |target| {
    matches!(target, EventTarget::WebviewWindow { label } if f(label))
  })?;

  Ok(())
}
