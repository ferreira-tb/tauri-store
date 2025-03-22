use crate::path::examples_dir;
use crate::process::command;
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct Playground;

impl Playground {
  #[allow(clippy::unused_self)]
  pub fn run(self) -> Result<()> {
    command("pnpm")
      .args(["run", "build:shared"])
      .call()?;

    command("pnpm")
      .args(["run", "-F", "'@tauri-store/pinia'", "build"])
      .call()?;

    command("cargo")
      .args(["tauri", "dev"])
      .dir(examples_dir().join("playground"))
      .call()
  }
}
