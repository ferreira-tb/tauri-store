use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tracing::{error, warn};
use __IMPORT_SOURCE__::{ManagerExt, SaveStrategy};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CounterStore {
  counter: i32,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  setup_tracing().unwrap();

  tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_window_state::Builder::new().build())
    .plugin(
      __IMPORT_SOURCE__::Builder::new()
        .default_save_strategy(SaveStrategy::throttle_secs(3))
        .autosave(Duration::from_secs(60))
        .pretty(true)
        .build(),
    )
    .setup(|app| {
      let handle = app.handle();
      (1..=3).for_each(|id| open_window(handle, id));
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      get_counter,
      on_error,
      on_warn,
      print_store,
      try_get_counter,
      try_store_state
    ])
    .run(tauri::generate_context!())
    .unwrap();
}

fn open_window(app: &AppHandle, id: u8) {
  let label = format!("window-{id}");
  let url = WebviewUrl::App("index.html".into());
  WebviewWindowBuilder::new(app, &label, url)
    .title(label)
    .inner_size(300.0, 500.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(true)
    .always_on_top(true)
    .build()
    .unwrap();
}

fn setup_tracing() -> Result<()> {
  use tracing::subscriber::set_global_default;
  use tracing_subscriber::fmt::time::ChronoLocal;
  use tracing_subscriber::fmt::Layer;
  use tracing_subscriber::layer::SubscriberExt;
  use tracing_subscriber::{EnvFilter, Registry};

  const TIMESTAMP: &str = "%F %T%.3f %:z";

  let filter = EnvFilter::builder()
    .from_env()?
    .add_directive("tauri_store=trace".parse()?)
    .add_directive("tauri_store_utils=trace".parse()?);

  let stderr = Layer::default()
    .with_ansi(true)
    .with_timer(ChronoLocal::new(TIMESTAMP.into()))
    .with_writer(std::io::stderr)
    .pretty();

  set_global_default(Registry::default().with(stderr).with(filter))?;

  Ok(())
}

#[tauri::command]
async fn on_error(message: String) {
  error!(error = message);
}

#[tauri::command]
async fn on_warn(message: String) {
  warn!(warning = message);
}

#[tauri::command]
async fn get_counter(app: AppHandle) -> Option<i32> {
  app
    .__STORE_COLLECTION__()
    .get("counter-store", "counter")
    .and_then(|counter| serde_json::from_value(counter).ok())
}

#[tauri::command]
async fn print_store(app: AppHandle) {
  let state = app
    .__STORE_COLLECTION__()
    .state("counter-store")
    .unwrap();

  println!("{state:?}");
}

#[tauri::command]
async fn try_get_counter(app: AppHandle) -> i32 {
  app
    .__STORE_COLLECTION__()
    .try_get::<i32>("counter-store", "counter")
    .unwrap()
}

#[tauri::command]
async fn try_store_state(app: AppHandle) -> CounterStore {
  app
    .__STORE_COLLECTION__()
    .try_state::<CounterStore>("counter-store")
    .unwrap()
}
