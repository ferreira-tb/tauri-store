use crate::path::docs_data_dir;
use crate::target::Target;
use anyhow::Result;
use bon::Builder;
use clap::Args;
use convert_case::{Case, Casing};
use semver::Version;
use serde::Serialize;
use serde_json::to_vec_pretty;
use std::cmp::Ordering;
use std::fs;
use strum::VariantArray;

#[derive(Debug, Args)]
pub struct Docs;

impl Docs {
  #[expect(clippy::unused_self)]
  pub fn run(&self) -> Result<()> {
    generate_metadata()
  }
}

fn generate_metadata() -> Result<()> {
  let mut targets = Vec::with_capacity(Target::VARIANTS.len());
  for target in Target::VARIANTS {
    let manifest = target.manifest()?;
    let plugin_name = target.plugin_name();

    let name = manifest.name();
    let docs_url = DocsUrl::builder()
      .maybe_javascript(plugin_name.map(|_| docs_js(name)))
      .rust(docs_rs(name))
      .changelog(changelog(name))
      .build();

    let metadata = Metadata::builder()
      .name(name)
      .version(manifest.version().clone())
      .maybe_title(plugin_name.map(|it| it.to_case(Case::Title)))
      .is_plugin(plugin_name.is_some())
      .docs(docs_url)
      .build();

    targets.push(metadata);
  }

  targets.sort_unstable();

  let path = docs_data_dir().join("metadata.json");
  fs::write(path, to_vec_pretty(&targets)?)?;

  Ok(())
}

#[derive(Builder, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
struct Metadata {
  name: String,
  version: Version,
  title: Option<String>,
  is_plugin: bool,
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
