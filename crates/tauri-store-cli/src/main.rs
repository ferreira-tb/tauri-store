mod command;
mod fs;
mod path;
mod plugin;
mod process;

use anyhow::Result;
use clap::Parser;
use command::prelude::*;

#[derive(Debug, Parser)]
#[command(name = "tauri-store-cli")]
enum Cli {
  Codegen(Codegen),
  Example(Example),
  Playground(Playground),
}

fn main() -> Result<()> {
  match Cli::parse() {
    Cli::Codegen(cmd) => cmd.run(),
    Cli::Example(cmd) => cmd.run(),
    Cli::Playground(cmd) => cmd.run(),
  }
}
