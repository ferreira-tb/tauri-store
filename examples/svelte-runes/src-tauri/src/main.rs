#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use anyhow::Result;
use example_shared::{build, setup_tracing};

fn main() -> Result<()> {
  setup_tracing("example_svelte_runes")?;
  build().run(tauri::generate_context!())?;
  Ok(())
}
