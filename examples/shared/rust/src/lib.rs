mod r#impl;

use anyhow::Result;
use r#impl::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::Wry;
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};
use tauri_store::SaveStrategy;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CounterStore {
  counter: i32,
}

pub fn build() -> tauri::Builder<Wry> {
  let mut builder = tauri::Builder::default();

  macro_rules! plugin {
    ($name:ident) => {
      builder = builder.plugin(
        $name::Builder::new()
          .default_save_strategy(SaveStrategy::throttle_secs(3))
          .autosave(Duration::from_secs(60))
          .pretty(true)
          .build(),
      );
    };
  }

  #[cfg(feature = "pinia")]
  plugin!(tauri_plugin_pinia);
  #[cfg(feature = "svelte")]
  plugin!(tauri_plugin_svelte);
  #[cfg(feature = "valtio")]
  plugin!(tauri_plugin_valtio);

  builder
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_window_state::Builder::new().build())
    .setup(|app| {
      let handle = app.handle();
      (1..=3).for_each(|id| open_window(handle, id));

      watch_counter(handle);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      get_counter,
      on_error,
      on_warn,
      print_counter,
      try_get_counter,
      try_store_state
    ])
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

pub fn setup_tracing(krate: &str) -> Result<()> {
  use tracing::subscriber::set_global_default;
  use tracing_subscriber::fmt::time::ChronoLocal;
  use tracing_subscriber::fmt::Layer;
  use tracing_subscriber::layer::SubscriberExt;
  use tracing_subscriber::{EnvFilter, Registry};

  const TIMESTAMP: &str = "%F %T%.3f %:z";

  #[cfg(feature = "pinia")]
  let directive = "tauri_plugin_pinia=trace";
  #[cfg(feature = "svelte")]
  let directive = "tauri_plugin_svelte=trace";
  #[cfg(feature = "valtio")]
  let directive = "tauri_plugin_valtio=trace";

  let filter = EnvFilter::builder()
    .from_env()?
    .add_directive(format!("{krate}=trace").parse()?)
    .add_directive("tauri_store=trace".parse()?)
    .add_directive("tauri_store_utils=trace".parse()?)
    .add_directive(directive.parse()?);

  let stderr = Layer::default()
    .with_ansi(true)
    .with_timer(ChronoLocal::new(TIMESTAMP.into()))
    .with_writer(std::io::stderr)
    .pretty();

  set_global_default(Registry::default().with(stderr).with(filter))?;

  Ok(())
}
