use crate::error::Result;
use crate::store::StoreState;
#[cfg(feature = "async-pinia")]
use crate::FutureExt;
use crate::ManagerExt;
use tauri::{AppHandle, Manager, Runtime, WebviewWindow};

#[cfg(not(feature = "async-pinia"))]
#[tauri::command]
pub(crate) async fn load<R: Runtime>(app: AppHandle<R>, id: String) -> Result<StoreState> {
  app.with_store(id, |store| Ok(store.state.clone()))
}

#[cfg(feature = "async-pinia")]
#[tauri::command]
pub(crate) async fn load<R: Runtime>(app: AppHandle<R>, id: String) -> Result<StoreState> {
  app
    .with_store(id, |store| async { Ok(store.state.clone()) }.boxed())
    .await
}

#[cfg(not(feature = "async-pinia"))]
#[tauri::command]
pub(crate) async fn patch<R: Runtime>(
  window: WebviewWindow<R>,
  id: String,
  state: StoreState,
) -> Result<()> {
  let app = window.app_handle().clone();
  app.with_store(id, move |store| {
    store.patch_with_source(state, window.label())
  })
}

#[cfg(feature = "async-pinia")]
#[tauri::command]
pub(crate) async fn patch<R: Runtime>(
  window: WebviewWindow<R>,
  id: String,
  state: StoreState,
) -> Result<()> {
  let app = window.app_handle().clone();
  app
    .with_store(id, move |store| {
      async move { store.patch_with_source(state, window.label()) }.boxed()
    })
    .await
}

#[cfg(not(feature = "async-pinia"))]
#[tauri::command]
pub(crate) async fn save<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.save_store(id)
}

#[cfg(feature = "async-pinia")]
#[tauri::command]
pub(crate) async fn save<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
  app.save_store(id).await
}

#[cfg(not(feature = "async-pinia"))]
#[tauri::command]
pub(crate) async fn save_all<R: Runtime>(app: AppHandle<R>) {
  app.pinia().save_all();
}

#[cfg(feature = "async-pinia")]
#[tauri::command]
pub(crate) async fn save_all<R: Runtime>(app: AppHandle<R>) {
  app.pinia().save_all().await;
}

#[cfg(not(feature = "async-pinia"))]
#[tauri::command]
pub(crate) async fn unload<R: Runtime>(app: AppHandle<R>, id: String) {
  app.pinia().unload_store(&id);
}

#[cfg(feature = "async-pinia")]
#[tauri::command]
pub(crate) async fn unload<R: Runtime>(app: AppHandle<R>, id: String) {
  app.pinia().unload_store(&id).await;
}
