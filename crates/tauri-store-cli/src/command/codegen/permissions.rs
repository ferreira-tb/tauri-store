use super::util::consts::PLUGIN_NAME;
use super::{Codegen, Context, Generator};
use crate::path::{assets_dir, crate_dir};
use crate::plugin::Plugin;
use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use strum::VariantArray;

pub(super) fn generate(codegen: &Codegen) -> Result<()> {
  let start = Instant::now();
  println!("{}", "generating permissions".cyan());

  if codegen.cleanup {
    cleanup_permissions()?;
  }

  let assets = assets_permissions_dir();
  generate_build_rs(&assets)?;
  generate_default(&assets)?;

  let duration = start.elapsed();
  println!("{}", format!("done in {duration:?}").green());

  Ok(())
}

fn cleanup_permissions() -> Result<()> {
  for plugin in Plugin::VARIANTS {
    let dir = permissions_dir(*plugin);
    let autogenerated = dir.join("autogenerated");
    let schemas = dir.join("schemas");

    fs::remove_dir_all(autogenerated)?;
    fs::remove_dir_all(schemas)?;
  }

  Ok(())
}

fn generate_build_rs(assets: &Path) -> Result<()> {
  let input = assets.join("build.rs");
  let output = |ctx: Context<'_>| {
    let dir = crate_dir(ctx.plugin);
    dir.join("build.rs").into()
  };

  Generator::builder(&input, &output)
    .skip(&[Plugin::Store])
    .replace(&[])
    .generate()
}

fn generate_default(assets: &Path) -> Result<()> {
  let input = assets.join("default.toml");
  let output = |ctx: Context<'_>| {
    let dir = permissions_dir(ctx.plugin);
    dir.join("default.toml").into()
  };

  Generator::builder(&input, &output)
    .transform(&[])
    .replace(&[(PLUGIN_NAME, &|it| it.as_ref().to_owned())])
    .generate()
}

fn assets_permissions_dir() -> PathBuf {
  assets_dir().join("permissions")
}

fn permissions_dir(plugin: Plugin) -> PathBuf {
  crate_dir(plugin).join("permissions")
}
