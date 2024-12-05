use crate::manager::ManagerExt;
use std::time::Duration;
use tauri::async_runtime::{self, RuntimeHandle};
use tauri::{AppHandle, Runtime};
use tokio::task::AbortHandle;
use tokio::time::{self, MissedTickBehavior};

#[cfg(not(feature = "unstable-async"))]
use tauri::async_runtime::spawn_blocking;

#[cfg(feature = "tracing")]
use {
  std::sync::atomic::{AtomicU32, Ordering},
  tracing::debug,
};

#[cfg(feature = "tracing")]
static AUTO_SAVE_ID: AtomicU32 = AtomicU32::new(0);

pub struct Autosave {
  #[cfg(feature = "tracing")]
  id: u32,
  abort_handle: AbortHandle,
}

impl Autosave {
  pub(crate) fn new<R>(app: &AppHandle<R>, duration: Duration) -> Self
  where
    R: Runtime,
  {
    let autosave = Self {
      #[cfg(feature = "tracing")]
      id: AUTO_SAVE_ID.fetch_add(1, Ordering::SeqCst),
      abort_handle: spawn_task(app, duration),
    };

    #[cfg(feature = "tracing")]
    debug!("autosave {} started", autosave.id);

    autosave
  }

  pub(crate) fn abort(self) {
    self.abort_handle.abort();

    #[cfg(feature = "tracing")]
    debug!("autosave {} aborted", self.id);
  }
}

fn spawn_task<R>(app: &AppHandle<R>, duration: Duration) -> AbortHandle
where
  R: Runtime,
{
  let app = app.clone();
  let RuntimeHandle::Tokio(runtime) = async_runtime::handle();
  let task = runtime.spawn(async move {
    let mut interval = time::interval(duration);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    loop {
      interval.tick().await;

      #[cfg(feature = "unstable-async")]
      let _ = app.store_collection().save_all().await;
      #[cfg(not(feature = "unstable-async"))]
      {
        let app = app.clone();
        let _ = spawn_blocking(move || app.store_collection().save_all()).await;
      }
    }
  });

  task.abort_handle()
}
