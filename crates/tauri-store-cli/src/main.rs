mod command;
mod fs;
mod path;
mod plugin;

use anyhow::Result;
use clap::Parser;
use command::prelude::*;

#[derive(Debug, Parser)]
#[command(name = "tauri-store-cli")]
enum Cli {
  Codegen(Codegen),
}

fn main() -> Result<()> {
  match Cli::parse() {
    Cli::Codegen(cmd) => cmd.run(),
  }
}
