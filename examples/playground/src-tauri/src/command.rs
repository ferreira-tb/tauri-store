use tracing::error;

#[tauri::command]
pub async fn on_error(message: String) {
  error!(error = message);
}
