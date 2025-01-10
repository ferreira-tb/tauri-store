use super::{Generator, OutputContext};
use crate::path::{assets_examples_dir, examples_shared_rust_impl_dir};
use anyhow::Result;
use colored::Colorize;
use std::path::Path;
use std::time::Instant;

pub(super) fn generate() -> Result<()> {
  let start = Instant::now();
  println!("{}", "generating example code".cyan());

  let assets = assets_examples_dir();
  generate_shared_commands(&assets)?;

  let duration = start.elapsed();
  println!("{}", format!("done in {duration:?}").green());

  Ok(())
}

fn generate_shared_commands(assets: &Path) -> Result<()> {
  let input = assets.join("commands.rs");
  let output = |ctx: OutputContext<'_>| {
    let dir = examples_shared_rust_impl_dir();
    dir.join(format!("{}.rs", ctx.name))
  };

  Generator::builder(&input, &output)
    .build()
    .generate()
}
