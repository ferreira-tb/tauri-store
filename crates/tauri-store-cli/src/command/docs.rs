use crate::path::docs_data_dir;
use crate::plugin::Plugin;
use anyhow::Result;
use bon::Builder;
use clap::Args;
use colored::Colorize;
use convert_case::{Case, Casing};
use semver::Version;
use serde::Serialize;
use serde_json::to_vec_pretty;
use std::cmp::Ordering;
use std::fs;
use std::time::Instant;
use strum::VariantArray;

#[derive(Debug, Args)]
pub struct Docs;

impl Docs {
  #[allow(clippy::unused_self)]
  pub fn run(&self) -> Result<()> {
    let start = Instant::now();
    println!("{}", "generating docs metadata".cyan());

    generate_metadata()?;

    let duration = start.elapsed();
    println!("{}", format!("done in {duration:?}").green());

    Ok(())
  }
}

fn generate_metadata() -> Result<()> {
  let mut plugins = Vec::with_capacity(Plugin::VARIANTS.len());
  for plugin in Plugin::VARIANTS {
    let manifest = plugin.manifest()?;
    let crate_name = plugin.crate_name();
    let metadata = Metadata::builder()
      .crate_name(&crate_name)
      .package_name(plugin.package_name())
      .title(plugin.title())
      .version(manifest.version())
      .docs(
        DocsUrl::builder()
          .js(docs_js(&crate_name))
          .rust(docs_rs(&crate_name))
          .build(),
      )
      .build();

    plugins.push(metadata);
  }

  plugins.sort_unstable();

  let path = docs_data_dir().join("metadata.json");
  fs::write(path, to_vec_pretty(&plugins)?)?;

  Ok(())
}

#[derive(Builder, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
struct Metadata {
  crate_name: String,
  package_name: String,
  title: String,
  version: Version,
  docs: DocsUrl,
}

impl PartialEq for Metadata {
  fn eq(&self, other: &Self) -> bool {
    self.crate_name == other.crate_name
  }
}

impl Eq for Metadata {}

impl PartialOrd for Metadata {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Metadata {
  fn cmp(&self, other: &Self) -> Ordering {
    self.crate_name.cmp(&other.crate_name)
  }
}

#[derive(Builder, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
struct DocsUrl {
  js: String,
  rust: String,
}

fn docs_js(crate_name: &str) -> String {
  format!("https://tb.dev.br/tauri-store/js-docs/{crate_name}")
}

fn docs_rs(crate_name: &str) -> String {
  let name = crate_name.to_case(Case::Snake);
  format!("https://tb.dev.br/tauri-store/rust-docs/{name}")
}
