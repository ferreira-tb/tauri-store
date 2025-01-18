mod command;
mod fs;
mod manifest;
mod path;
mod target;
mod transform;

use anyhow::Result;
use clap::Parser;
use command::prelude::*;

#[derive(Debug, Parser)]
#[command(name = "tauri-store-cli")]
enum Cli {
  Changelog(Changelog),
  Codegen(Codegen),
  Docs(Docs),
}

fn main() -> Result<()> {
  match Cli::parse() {
    Cli::Changelog(cmd) => cmd.run(),
    Cli::Codegen(cmd) => cmd.run(),
    Cli::Docs(cmd) => cmd.run(),
  }
}
