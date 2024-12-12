use super::{ResourceTuple, SaveStrategy, Store, StoreState};
use crate::error::Result;
use crate::event::EventSource;
use crate::manager::ManagerExt;
use crate::store::save::{debounce, save_now, throttle};
use serde_json::Value as Json;
use std::sync::Arc;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Runtime};

#[cfg(feature = "unstable-async")]
impl<R: Runtime> Store<R> {
  pub(crate) async fn load(app: AppHandle<R>, id: impl AsRef<str>) -> Result<ResourceTuple<R>> {
    let id = id.as_ref().to_owned();
    spawn_blocking(move || Self::blocking_load(&app, id)).await?
  }

  /// Sets a key-value pair in the store.
  pub async fn set(&mut self, key: impl AsRef<str>, value: Json) -> Result<()> {
    self.state.insert(key.as_ref().to_owned(), value);
    self.on_state_change(None).await
  }

  /// Patches the store state, optionally having a window as the source.
  pub async fn patch_with_source<S>(&mut self, state: StoreState, source: S) -> Result<()>
  where
    S: Into<EventSource>,
  {
    self.state.extend(state);
    self.on_state_change(source).await
  }

  /// Patches the store state.
  pub async fn patch(&mut self, state: StoreState) -> Result<()> {
    self.patch_with_source(state, None).await
  }

  async fn on_state_change(&self, source: impl Into<EventSource>) -> Result<()> {
    self.emit_state_change(source)?;
    self.call_watchers();

    if self.save_on_change {
      self.save().await?;
    }

    Ok(())
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
