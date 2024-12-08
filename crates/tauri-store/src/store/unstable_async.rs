use super::{to_bytes, ResourceTuple, SaveStrategy, Store};
use crate::error::Result;
use crate::manager::ManagerExt;
use crate::store::save::debounce;
use std::sync::Arc;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Runtime};

#[cfg(tauri_store_tracing)]
use tracing::debug;

#[cfg(feature = "unstable-async")]
impl<R: Runtime> Store<R> {
  pub(crate) async fn load(app: AppHandle<R>, id: impl AsRef<str>) -> Result<ResourceTuple<R>> {
    let id = id.as_ref().to_owned();
    spawn_blocking(move || Self::blocking_load(&app, id)).await?
  }

  /// Save the store state to the disk.
  pub async fn save(&self) -> Result<()> {
    match self.app.store_collection().save_strategy {
      SaveStrategy::Debounce(duration) => {
        self
          .save_handle
          .get_or_init(|| debounce(duration, Arc::from(self.id.as_str())))
          .call(&self.app);
      }
      SaveStrategy::Throttle(_) => unimplemented!(),
      SaveStrategy::Immediate => self.save_now().await?,
    };

    Ok(())
  }

  /// Save the store immediately, ignoring the save strategy.
  pub async fn save_now(&self) -> Result<()> {
    use tokio::fs::{self, File};
    use tokio::io::AsyncWriteExt;

    let collection = self.app.store_collection();
    if collection
      .save_denylist
      .as_ref()
      .is_some_and(|it| it.contains(&self.id))
    {
      return Ok(());
    }

    fs::create_dir_all(collection.path()).await?;

    let bytes = to_bytes(&self.state, collection.pretty)?;
    let mut file = File::create(self.path()).await?;
    file.write_all(&bytes).await?;
    file.flush().await?;

    #[cfg(tauri_store_tracing)]
    debug!("store saved: {}", self.id);

    Ok(())
  }
}
