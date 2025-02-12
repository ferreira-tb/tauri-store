use super::codegen::util::transform::remove_nocheck;
use crate::fs::{read_file, write_file};
use crate::path::{assets_dir, docs_changelog_dir};
use crate::plugin::Plugin;
use anyhow::Result;
use clap::Args;
use semver::Version;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct Changelog {
  #[arg(short = 'p', long)]
  plugin: String,
  #[arg(short = 'v', long)]
  version: Version,
}

impl Changelog {
  pub fn run(&self) -> Result<()> {
    let input = assets_changelog_dir().join("version.svelte");
    let mut contents = read_file(&input)?;
    remove_nocheck(&mut contents);

    contents = contents.replace("VERSION", &self.version.to_string());

    let plugin = Plugin::try_from(self.plugin.as_str())?;
    let output = changelog_dir(plugin)
      .join(format!("v{}", self.version.major))
      .join(format!("{}.svelte", self.version));

    write_file(&output, &contents)
  }
}

fn assets_changelog_dir() -> PathBuf {
  assets_dir().join("changelog")
}

fn changelog_dir(plugin: Plugin) -> PathBuf {
  docs_changelog_dir().join(plugin.as_ref())
}
