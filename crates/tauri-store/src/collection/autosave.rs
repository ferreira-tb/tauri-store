use crate::manager::ManagerExt;
use futures::future::BoxFuture;
use std::sync::Arc;
use std::time::Duration;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Runtime};
use tauri_store_utils::set_interval;
use tokio::sync::Semaphore;
use tokio::task::AbortHandle;

type AutosaveFn<R> = Box<dyn Fn(AppHandle<R>) -> BoxFuture<'static, ()> + Send + 'static>;

pub(crate) struct Autosave {
  duration: Option<Duration>,
  abort_handle: Option<AbortHandle>,
  semaphore: Arc<Semaphore>,
}

impl Autosave {
  pub fn new(duration: Option<Duration>) -> Self {
    Self {
      duration,
      abort_handle: None,
      semaphore: Arc::new(Semaphore::new(1)),
    }
  }

  pub fn start<R: Runtime>(&mut self, app: &AppHandle<R>) {
    self.stop();
    if let Some(duration) = self.duration {
      let semaphore = Arc::clone(&self.semaphore);
      let abort_handle = set_interval(app, duration, save(semaphore));
      self.abort_handle = Some(abort_handle);
    }
  }

  pub fn stop(&mut self) {
    if let Some(handle) = self.abort_handle.take() {
      handle.abort();
    }
  }

  pub fn set_duration(&mut self, duration: Duration) {
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

      let _ = spawn_blocking(move || app.store_collection().save_all()).await;
    })
  })
}

impl Drop for Autosave {
  fn drop(&mut self) {
    self.stop();
  }
}
