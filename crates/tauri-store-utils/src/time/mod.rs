mod debounce;

use crate::manager::ManagerExt;
pub use debounce::Debounce;
use std::future::Future;
use std::time::Duration;
use tauri::{AppHandle, Runtime};
use tokio::task::AbortHandle;
use tokio::time::{self, MissedTickBehavior};

/// Calls the given function at regular intervals.
pub fn set_interval<R, F, Fut>(app: &AppHandle<R>, duration: Duration, f: F) -> AbortHandle
where
  R: Runtime,
  F: Fn(AppHandle<R>) -> Fut + Send + 'static,
  Fut: Future<Output = ()> + Send + 'static,
{
  app.spawn(move |app| {
    async move {
      let mut interval = time::interval(duration);
      interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

      // The first tick completes immediately.
      interval.tick().await;

      loop {
        interval.tick().await;
        f(app.clone()).await;
      }
    }
  })
}
