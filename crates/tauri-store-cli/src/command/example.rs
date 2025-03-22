use crate::path::examples_dir;
use crate::process::command;
use anyhow::Result;
use clap::Args;
use itertools::Itertools;
use rand::seq::IndexedRandom;
use std::fs::read_dir;

const EXCLUDE: &[&str] = &["assets", "playground"];

#[derive(Debug, Args)]
pub struct Example {
  example: Option<String>,
  #[arg(long, short, value_name = "FEATURE")]
  features: Option<Vec<String>>,
}

impl Example {
  pub fn run(mut self) -> Result<()> {
    let example = self.pick_example()?;
    println!("Starting example: {}", example.to_uppercase());

    command("pnpm")
      .args(["run", "build:shared"])
      .call()?;

    let mut args = vec!["tauri", "dev"]
      .into_iter()
      .map(String::from)
      .collect_vec();

    if let Some(features) = self.features.take() {
      for feature in features {
        args.push(String::from("-f"));
        args.push(feature);
      }
    }

    command("cargo")
      .args(args.iter().map(AsRef::as_ref))
      .dir(examples_dir().join(example))
      .call()
  }

  fn pick_example(&mut self) -> Result<String> {
    if let Some(example) = self.example.take() {
      Ok(example)
    } else {
      examples()?
        .choose(&mut rand::rng())
        .map(ToOwned::to_owned)
        .map(Ok)
        .unwrap()
    }
  }
}

fn examples() -> Result<Vec<String>> {
  let mut examples = Vec::new();
  for entry in read_dir(examples_dir())? {
    let entry = entry?;
    if entry.file_type()?.is_dir() {
      let name = entry
        .file_name()
        .into_string()
        .expect("invalid dirname");

      if !EXCLUDE.contains(&name.as_str()) {
        examples.push(name);
      }
    }
  }

  Ok(examples)
}
