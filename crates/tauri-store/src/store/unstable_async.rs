use super::{ResourceTuple, SaveStrategy, Store};
use crate::error::Result;
use crate::manager::ManagerExt;
use crate::store::save::{debounce, save_now, throttle};
use std::sync::Arc;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Runtime};

#[cfg(feature = "unstable-async")]
impl<R: Runtime> Store<R> {
  pub(crate) async fn load(app: AppHandle<R>, id: impl AsRef<str>) -> Result<ResourceTuple<R>> {
    let id = id.as_ref().to_owned();
    spawn_blocking(move || Self::blocking_load(&app, id)).await?
  }

  /// Save the store state to the disk.
  pub async fn save(&self) -> Result<()> {
    match self.app.store_collection().default_save_strategy {
      SaveStrategy::Immediate => self.save_now().await?,
      SaveStrategy::Debounce(duration) => {
        self
          .debounce_save_handle
          .get_or_init(|| debounce(duration, Arc::from(self.id.as_str())))
          .call(&self.app);
      }
      SaveStrategy::Throttle(duration) => {
        self
          .throttle_save_handle
          .get_or_init(|| throttle(duration, Arc::from(self.id.as_str())))
          .call(&self.app);
      }
    };

    Ok(())
  }

  /// Save the store immediately, ignoring the save strategy.
  pub async fn save_now(&self) -> Result<()> {
    save_now(self).await
  }
}
