use super::{Generator, OutputContext};
use crate::path::{assets_permissions_dir, crate_dir, permissions_dir};
use anyhow::Result;
use colored::Colorize;
use std::path::Path;
use std::time::Instant;

pub(super) fn generate() -> Result<()> {
  let start = Instant::now();
  println!("{}", "generating permissions".cyan());

  let assets = assets_permissions_dir();
  generate_build_rs(&assets)?;
  generate_default(&assets)?;

  let duration = start.elapsed();
  println!("{}", format!("done in {duration:?}").green());

  Ok(())
}

fn generate_build_rs(assets: &Path) -> Result<()> {
  let input = assets.join("build.rs");
  let output = |ctx: OutputContext<'_>| {
    let dir = crate_dir(ctx.target);
    dir.join("build.rs")
  };

  Generator::builder(&input, &output)
    .build()
    .generate()
}

fn generate_default(assets: &Path) -> Result<()> {
  let input = assets.join("default.toml");
  let output = |ctx: OutputContext<'_>| {
    let dir = permissions_dir(ctx.target);
    dir.join("default.toml")
  };

  Generator::builder(&input, &output)
    .transform(&[])
    .build()
    .generate()
}
