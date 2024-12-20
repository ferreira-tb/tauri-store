mod pinia;

use tauri::AppHandle;
use tauri_plugin_pinia::ManagerExt;
use tracing::{error, warn};

pub(crate) mod prelude {
  pub(crate) use super::pinia::*;
  pub(crate) use super::{on_error, on_warn, watch_counter};
}

#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub(crate) fn on_error(message: String) {
  error!(error = message);
}

#[tauri::command]
#[expect(clippy::needless_pass_by_value)]
pub(crate) fn on_warn(message: String) {
  warn!(warning = message);
}

macro_rules! watch_counter {
  ($kind:ident) => {
    #[cfg(not(feature = "unstable-async"))]
    pub(crate) fn watch_counter(app: &AppHandle) {
      let _ = app.$kind().watch("counter-store", |handle| {
        handle
          .$kind()
          .try_get::<i32>("counter-store", "counter")
          .inspect(|counter| println!("counter: {counter}"))
          .map(drop)
      });
    }

    #[cfg(feature = "unstable-async")]
    pub(crate) async fn watch_counter(app: &AppHandle) {
      let _ = app
        .$kind()
        .watch("counter-store", |handle| {
          Box::pin(async move {
            handle
              .$kind()
              .try_get::<i32>("counter-store", "counter")
              .await
              .inspect(|counter| println!("counter: {counter}"))
              .map(drop)
          })
        })
        .await;
    }
  };
}

#[cfg(feature = "pinia")]
watch_counter!(pinia);
