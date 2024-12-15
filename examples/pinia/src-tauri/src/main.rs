#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use example_shared::{build, setup_tracing};

fn main() -> Result<()> {
  setup_tracing("example_pinia")?;
  build().run(tauri::generate_context!())?;
  Ok(())
}
