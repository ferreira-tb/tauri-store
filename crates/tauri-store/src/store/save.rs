use crate::manager::ManagerExt;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Runtime};
use tauri_store_utils::Debounce;
use tauri_store_utils::RemoteCallable;

#[cfg(not(feature = "unstable-async"))]
use {crate::error::Error, futures::future::FutureExt, tauri::async_runtime::spawn_blocking};

pub(super) type SaveHandle<R> = Box<dyn RemoteCallable<AppHandle<R>> + Send + Sync>;

#[derive(Clone, Copy, Debug, Default)]
pub enum SaveStrategy {
  #[default]
  Immediate,
  Debounce(Duration),
  Throttle(Duration),
}

impl SaveStrategy {
  #[inline]
  pub fn debounce_millis(millis: u64) -> Self {
    Self::Debounce(Duration::from_millis(millis))
  }

  #[inline]
  pub fn debounce_secs(secs: u64) -> Self {
    Self::Debounce(Duration::from_secs(secs))
  }

  #[inline]
  pub fn throttle_millis(millis: u64) -> Self {
    Self::Throttle(Duration::from_millis(millis))
  }

  #[inline]
  pub fn throttle_secs(secs: u64) -> Self {
    Self::Throttle(Duration::from_secs(secs))
  }
}

#[cfg(not(feature = "unstable-async"))]
pub(super) fn debounce<R: Runtime>(duration: Duration, id: Arc<str>) -> SaveHandle<R> {
  let debounce = Debounce::new(duration, move |app| {
    let id = Arc::clone(&id);
    let task = spawn_blocking(move || {
      let resource = app.store_collection().get_resource(&id)?;
      if let Ok(store) = resource.inner.lock() {
        store.save_now()?;
      }

      Ok::<_, Error>(())
    });

    task.map(drop)
  });

  Box::new(debounce)
}

#[cfg(feature = "unstable-async")]
pub(super) fn debounce<R: Runtime>(duration: Duration, id: Arc<str>) -> SaveHandle<R> {
  let debounce = Debounce::new(duration, move |app| {
    let id = Arc::clone(&id);
    async move {
      let resource = app.store_collection().get_resource(&id).await?;
      let store = resource.inner.lock().await;
      store.save_now().await
    }
  });

  Box::new(debounce)
}
