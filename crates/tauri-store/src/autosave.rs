use crate::manager::ManagerExt;
use std::time::Duration;
use tauri::async_runtime::{self, RuntimeHandle};
use tauri::{AppHandle, Runtime};
use tokio::task::AbortHandle;
use tokio::time::{self, MissedTickBehavior};

#[cfg(not(feature = "unstable-async"))]
use tauri::async_runtime::spawn_blocking;

#[cfg(feature = "tracing")]
use tracing::{debug, trace};

pub struct Autosave {
  duration: Option<Duration>,
  abort_handle: Option<AbortHandle>,
}

impl Autosave {
  pub(crate) fn new(duration: Option<Duration>) -> Self {
    Self { duration, abort_handle: None }
  }

  pub(crate) fn start<R: Runtime>(&mut self, app: &AppHandle<R>) {
    self.stop();

    if let Some(duration) = self.duration.clone() {
      let app = app.clone();
      let RuntimeHandle::Tokio(runtime) = async_runtime::handle();

      let task = runtime.spawn(async move {
        let mut interval = time::interval(duration);
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

        // The first tick completes immediately.
        interval.tick().await;

        loop {
          interval.tick().await;
          save_all(&app).await;

          #[cfg(feature = "tracing")]
          trace!("autosave ticked");
        }
      });

      self.abort_handle = Some(task.abort_handle());

      #[cfg(feature = "tracing")]
      debug!("autosave started");
    };
  }

  pub(crate) fn stop(&mut self) {
    if let Some(handle) = self.abort_handle.take() {
      handle.abort();

      #[cfg(feature = "tracing")]
      debug!("autosave aborted");
    }
  }

  pub(crate) fn set_duration(&mut self, duration: Duration) {
    self.duration = Some(duration);
  }
}

#[cfg(feature = "unstable-async")]
async fn save_all<R: Runtime>(app: &AppHandle<R>) {
  let _ = app.store_collection().save_all().await;
}

#[cfg(not(feature = "unstable-async"))]
async fn save_all<R: Runtime>(app: &AppHandle<R>) {
  let app = app.clone();
  let _ = spawn_blocking(move || app.store_collection().save_all()).await;
}

impl Drop for Autosave {
  fn drop(&mut self) {
    self.stop();
  }
}
