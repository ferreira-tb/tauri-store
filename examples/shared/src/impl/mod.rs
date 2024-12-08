mod pinia;

use tauri::AppHandle;
use tauri_plugin_pinia::ManagerExt;

pub(crate) mod prelude {
  pub(crate) use super::pinia::*;
  pub(crate) use super::watch_counter;
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
