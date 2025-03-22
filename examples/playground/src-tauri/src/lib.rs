mod command;
mod migration;

use anyhow::Result;
use std::time::Duration;
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_pinia::{BoxResult, SaveStrategy};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  setup_tracing().unwrap();

  tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_window_state::Builder::new().build())
    .plugin(
      tauri_plugin_pinia::Builder::new()
        .autosave(Duration::from_secs(60))
        .default_save_strategy(SaveStrategy::throttle_secs(3))
        .pretty(true)
        .save_denylist(["dont-save-1", "dont-save-2"])
        .sync_denylist(["dont-sync-1", "dont-sync-2"])
        .on_before_each_migration(|_| ())
        .migrations("playground", migration::all())
        .build(),
    )
    .setup(|app| open_window(app.handle()))
    .invoke_handler(tauri::generate_handler![command::on_error])
    .run(tauri::generate_context!())
    .unwrap();
}

fn open_window(app: &AppHandle) -> BoxResult<()> {
  let url = WebviewUrl::App("index.html".into());
  WebviewWindowBuilder::new(app, "main", url)
    .title("Playground")
    .inner_size(300.0, 500.0)
    .resizable(true)
    .maximizable(true)
    .minimizable(true)
    .visible(true)
    .always_on_top(true)
    .build()?;

  Ok(())
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
