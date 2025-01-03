#[cfg(feature = "pinia")]
mod pinia;
#[cfg(feature = "svelte")]
mod svelte;

use tauri::AppHandle;
use tracing::{error, warn};

pub(crate) mod prelude {
  #[cfg(feature = "pinia")]
  pub(crate) use super::pinia::*;
  #[cfg(feature = "svelte")]
  pub(crate) use super::svelte::*;

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
  ($plugin:ident, $kind:ident) => {
    pub(crate) fn watch_counter(app: &AppHandle) {
      use $plugin::ManagerExt;
      let _ = app.$kind().watch("counter-store", |handle| {
        handle
          .$kind()
          .try_get::<i32>("counter-store", "counter")
          .inspect(|counter| println!("counter: {counter}"))
          .map(drop)
      });
    }
  };
}

#[cfg(feature = "pinia")]
watch_counter!(tauri_plugin_pinia, pinia);
#[cfg(feature = "svelte")]
watch_counter!(tauri_plugin_svelte, svelte);
