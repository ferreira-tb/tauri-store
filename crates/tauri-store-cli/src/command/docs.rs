use crate::path::docs_data_dir;
use crate::target::Target;
use anyhow::Result;
use clap::Args;
use serde_json::{to_vec_pretty, Value as Json};
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
  let mut manifests = Vec::with_capacity(Target::VARIANTS.len());
  for target in Target::VARIANTS {
    manifests.push(target.manifest()?.json()?);
  }

  manifests.sort_unstable_by(sort);

  let path = docs_data_dir().join("metadata.json");
  fs::write(path, to_vec_pretty(&manifests)?)?;

  Ok(())
}

fn sort(a: &Json, b: &Json) -> Ordering {
  name(a).cmp(name(b))
}

fn name(target: &Json) -> &str {
  target
    .get("name")
    .and_then(|name| name.as_str())
    .expect("name is required")
}
