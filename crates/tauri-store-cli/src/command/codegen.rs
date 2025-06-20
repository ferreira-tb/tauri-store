mod crates;
mod examples;
mod packages;
mod permissions;
pub mod util;

use crate::fs::{read_file, write_file};
use crate::plugin::Plugin;
use crate::process::command;
use anyhow::Result;
use bon::Builder;
use clap::Args;
use derive_more::Deref;
use std::path::{Path, PathBuf};
use strum::VariantArray;
use util::transform::prepend_autogenerated;

#[derive(Debug, Args)]
pub struct Codegen {
  #[arg(long)]
  cleanup: bool,
  #[arg(long)]
  format: bool,
}

impl Codegen {
  pub fn run(self) -> Result<()> {
    crates::generate()?;
    packages::generate()?;
    examples::generate()?;
    permissions::generate(&self)?;

    if self.format {
      command("pnpm").args(["run", "format"]).call()?;
    }

    Ok(())
  }
}

#[derive(Builder)]
#[builder(finish_fn = replace)]
struct Generator<'a> {
  #[builder(start_fn)]
  input: &'a Path,
  #[builder(start_fn)]
  output: &'a dyn Fn(Context) -> OutputDir,
  #[builder(finish_fn)]
  replacements: &'a [(&'a str, &'a dyn Fn(Plugin) -> String)],
  #[builder(default = &[&prepend_autogenerated])]
  transform: &'a [&'a dyn Fn(&mut String)],
  #[builder(default)]
  skip: &'a [Plugin],
}

impl Generator<'_> {
  pub fn generate(self) -> Result<()> {
    for plugin in Plugin::VARIANTS {
      if self.skip.contains(plugin) {
        continue;
      }

      let mut contents = read_file(self.input)?;
      self
        .transform
        .iter()
        .for_each(|t| t(&mut contents));

      for (key, value) in self.replacements {
        contents = contents.replace(key, &value(*plugin).to_string());
      }

      let title = plugin.title();
      let ctx = Context::new(*plugin, &title);
      (self.output)(ctx)
        .iter()
        .try_for_each(|path| write_file(path, &contents))?;
    }

    Ok(())
  }
}

#[derive(Deref)]
struct OutputDir(Vec<PathBuf>);

impl From<PathBuf> for OutputDir {
  fn from(path: PathBuf) -> Self {
    Self(vec![path])
  }
}

impl<const N: usize> From<[PathBuf; N]> for OutputDir {
  fn from(paths: [PathBuf; N]) -> Self {
    Self(paths.to_vec())
  }
}

struct Context<'a> {
  plugin: Plugin,
  name: &'a str,
}

impl<'a> Context<'a> {
  fn new(plugin: Plugin, name: &'a str) -> Self {
    Self { plugin, name }
  }
}
