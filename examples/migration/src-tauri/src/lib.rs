mod migration;

use anyhow::Result;
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tauri_store::BoxResult;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  setup_tracing().unwrap();

  tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_window_state::Builder::new().build())
    .plugin(
      tauri_store::Builder::new()
        .on_before_each_migration(|_| ())
        .migrations("my-store", migration::all())
        .build_plugin(),
    )
    .setup(|app| open_window(app.handle()))
    .run(tauri::generate_context!())
    .unwrap();
}

fn open_window(app: &AppHandle) -> BoxResult<()> {
  let url = WebviewUrl::App("index.html".into());
  WebviewWindowBuilder::new(app, "main", url)
    .title("Migration")
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
