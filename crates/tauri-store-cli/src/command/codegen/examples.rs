use super::util::consts::{IMPORT_SOURCE, STORE_COLLECTION};
use super::util::replace::store_collection;
use super::{Context, Generator, OutputDir};
use crate::path::{assets_dir, examples_dir};
use crate::plugin::Plugin;
use anyhow::Result;
use colored::Colorize;
use convert_case::{Case, Casing};
use std::path::{Path, PathBuf};
use std::time::Instant;

pub(super) fn generate() -> Result<()> {
  let start = Instant::now();
  println!("{}", "generating examples".cyan());

  let assets = assets_examples_dir();
  generate_lib(&assets)?;

  let duration = start.elapsed();
  println!("{}", format!("done in {duration:?}").green());

  Ok(())
}

fn generate_lib(assets: &Path) -> Result<()> {
  let input = assets.join("lib.rs");
  let output = |ctx: Context<'_>| match ctx.plugin {
    Plugin::Pinia => {
      let pinia = example_lib("pinia");
      let nuxt = example_lib("pinia-nuxt");
      OutputDir::from([pinia, nuxt])
    }
    Plugin::Svelte => {
      let svelte = example_lib("svelte");
      let runes = example_lib("svelte-runes");
      OutputDir::from([svelte, runes])
    }
    Plugin::Valtio => {
      let valtio = example_lib("valtio");
      OutputDir::from(valtio)
    }
    Plugin::Store => unreachable!(),
  };

  Generator::builder(&input, &output)
    .skip(&[Plugin::Store])
    .replace(&[
      (IMPORT_SOURCE, &|it| it.as_ref().to_case(Case::Snake)),
      (STORE_COLLECTION, &|it| store_collection(it, Case::Snake)),
    ])
    .generate()
}

fn assets_examples_dir() -> PathBuf {
  assets_dir().join("examples")
}

fn example_lib(example: &str) -> PathBuf {
  examples_dir()
    .join(example)
    .join("src-tauri/src/lib.rs")
}
