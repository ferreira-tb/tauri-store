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
    let plugin_name = plugin.name();

    let name = manifest.name();
    let docs_url = DocsUrl::builder()
      .javascript(docs_js(plugin_name))
      .rust(docs_rs(name))
      .changelog(changelog(name))
      .build();

    let metadata = Metadata::builder()
      .name(name)
      .version(manifest.version().clone())
      .title(plugin_name.to_case(Case::Title))
      .docs(docs_url)
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
  name: String,
  version: Version,
  title: Option<String>,
  docs: DocsUrl,
}

impl PartialEq for Metadata {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

impl Eq for Metadata {}

impl PartialOrd for Metadata {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Metadata {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.name.cmp(&other.name)
  }
}

#[derive(Builder, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
struct DocsUrl {
  javascript: Option<String>,
  rust: String,
  changelog: String,
}

fn docs_js(name: &str) -> String {
  format!("https://tb.dev.br/tauri-store/reference/{name}")
}

fn docs_rs(name: &str) -> String {
  let snake = name.to_case(Case::Snake);
  format!("https://tb.dev.br/tauri-store/rust-docs/{snake}")
}

fn changelog(name: &str) -> String {
  format!("/tauri-store/changelog/{name}")
}
