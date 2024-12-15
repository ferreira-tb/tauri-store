use crate::error::Result;
use crate::store::{Store, StoreOptions, StoreState};
use serde::Serialize;
use tauri::{AppHandle, Emitter as _, EventTarget, Runtime, WebviewWindow, Window};

pub const STORE_CONFIG_CHANGE_EVENT: &str = "tauri-store://config-change";
pub const STORE_STATE_CHANGE_EVENT: &str = "tauri-store://state-change";
pub const STORE_UNLOAD_EVENT: &str = "tauri-store://unload";

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StatePayload<'a> {
  id: &'a str,
  state: &'a StoreState,
}

impl<'a, R: Runtime> From<&'a Store<R>> for StatePayload<'a> {
  fn from(store: &'a Store<R>) -> Self {
    Self { id: &store.id, state: &store.state }
  }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConfigPayload<'a> {
  id: &'a str,
  config: StoreOptions,
}

impl<'a, R: Runtime> From<&'a Store<R>> for ConfigPayload<'a> {
  fn from(store: &'a Store<R>) -> Self {
    Self { id: &store.id, config: store.into() }
  }
}

pub(crate) fn emit<R, T, S>(app: &AppHandle<R>, event: &str, payload: &T, source: S) -> Result<()>
where
  R: Runtime,
  T: Serialize + ?Sized,
  S: Into<EventSource>,
{
  let source: EventSource = source.into();
  if let Some(source) = source.0 {
    emit_filter(app, event, payload, |it| it != source)
  } else {
    emit_all(app, event, payload)
  }
}

fn emit_all<R, T>(app: &AppHandle<R>, event: &str, payload: &T) -> Result<()>
where
  R: Runtime,
  T: Serialize + ?Sized,
{
  app.emit_filter(event, payload, |target| {
    matches!(target, EventTarget::WebviewWindow { .. })
  })?;

  Ok(())
}

fn emit_filter<R, T, F>(app: &AppHandle<R>, event: &str, payload: &T, f: F) -> Result<()>
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

pub struct EventSource(Option<String>);

impl EventSource {
  pub const fn is_backend(&self) -> bool {
    self.0.is_none()
  }
}

impl From<&str> for EventSource {
  fn from(source: &str) -> Self {
    Self(Some(String::from(source)))
  }
}

impl From<Option<&str>> for EventSource {
  fn from(source: Option<&str>) -> Self {
    Self(source.map(String::from))
  }
}

impl From<String> for EventSource {
  fn from(source: String) -> Self {
    Self(Some(source))
  }
}

impl From<&WebviewWindow> for EventSource {
  fn from(window: &WebviewWindow) -> Self {
    Self(Some(window.label().to_owned()))
  }
}

impl From<&Window> for EventSource {
  fn from(window: &Window) -> Self {
    Self(Some(window.label().to_owned()))
  }
}
