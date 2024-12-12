use crate::manager::ManagerExt;
use futures::future::BoxFuture;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Runtime};
use tauri_store_utils::set_interval;
use tokio::sync::Semaphore;
use tokio::task::AbortHandle;

#[cfg(not(feature = "unstable-async"))]
use tauri::async_runtime::spawn_blocking;

#[cfg(tauri_store_tracing)]
use tracing::{debug, trace};

type AutosaveFn<R> = Box<dyn Fn(AppHandle<R>) -> BoxFuture<'static, ()> + Send + 'static>;

pub struct Autosave {
  duration: Option<Duration>,
  abort_handle: Option<AbortHandle>,
  semaphore: Arc<Semaphore>,
}

impl Autosave {
  pub(crate) fn new(duration: Option<Duration>) -> Self {
    Self {
      duration,
      abort_handle: None,
      semaphore: Arc::new(Semaphore::new(1)),
    }
  }

  pub(crate) fn start<R: Runtime>(&mut self, app: &AppHandle<R>) {
    self.stop();
    if let Some(duration) = self.duration {
      let semaphore = Arc::clone(&self.semaphore);
      let abort_handle = set_interval(app, duration, save(semaphore));
      self.abort_handle = Some(abort_handle);

      #[cfg(tauri_store_tracing)]
      debug!("autosave started");
    };
  }

  pub(crate) fn stop(&mut self) {
    if let Some(handle) = self.abort_handle.take() {
      handle.abort();

      #[cfg(tauri_store_tracing)]
      debug!("autosave aborted");
    }
  }

  pub(crate) fn set_duration(&mut self, duration: Duration) {
    self.duration.replace(duration);
  }
}

fn save<R: Runtime>(semaphore: Arc<Semaphore>) -> AutosaveFn<R> {
  Box::new(move |app| {
    let semaphore = Arc::clone(&semaphore);
    Box::pin(async move {
      let _permit = semaphore
        .acquire()
        .await
        .expect("semaphore will not be closed");

      #[cfg(feature = "unstable-async")]
      let _ = app.store_collection().save_all().await;

      #[cfg(not(feature = "unstable-async"))]
      let _ = spawn_blocking(move || app.store_collection().save_all()).await;

      #[cfg(tauri_store_tracing)]
      trace!("autosave ticked");
    })
  })
}

impl Drop for Autosave {
  fn drop(&mut self) {
    self.stop();
  }
}
