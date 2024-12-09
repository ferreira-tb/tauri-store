use crate::package::Package;
use crate::path::{assets_dir, crate_commands_dir, package_commands_dir};
use anyhow::{Context, Result};
use clap::Args;
use convert_case::{Case, Casing};
use std::fs;

const PLUGIN_NAME: &str = "PLUGIN_NAME";
const PASCAL_PLUGIN_NAME: &str = "PASCAL_PLUGIN_NAME";

const AUTOGENERATED_WARNING: &str =
  "// This file was autogenerated and should not be edited manually.\n\n";

#[derive(Debug, Args)]
pub struct Codegen;

impl Codegen {
  #[expect(clippy::unused_self)]
  pub fn run(&self) -> Result<()> {
    generate_crate_commands()?;
    generate_package_commands()?;

    Ok(())
  }
}

fn generate_package_commands() -> Result<()> {
  let assets_dir = assets_dir();
  let commands_ts = assets_dir.join("commands.ts");
  let mut contents = fs::read_to_string(&commands_ts).with_context(|| {
    format!(
      "failed to read commands template: {}",
      commands_ts.display()
    )
  })?;

  contents
    .replace("// @ts-nocheck", "")
    .trim_start()
    .clone_into(&mut contents);

  contents = format!("{AUTOGENERATED_WARNING}{contents}");

  let packages = [Package::TauriPluginPinia];
  for package in packages {
    let plugin_name = package.plugin_name();
    let contents = contents
      .replace(PASCAL_PLUGIN_NAME, &plugin_name.to_case(Case::Pascal))
      .replace(PLUGIN_NAME, plugin_name);

    let dir = package_commands_dir(package);
    let autogenerated = dir.join("autogenerated.ts");
    fs::write(autogenerated, contents).with_context(|| {
      format!(
        "failed to write autogenerated commands for package: {}",
        package.as_ref()
      )
    })?;
  }

  Ok(())
}

fn generate_crate_commands() -> Result<()> {
  let assets_dir = assets_dir();
  let commands_rs = assets_dir.join("commands.rs");
  let mut contents = fs::read_to_string(&commands_rs).with_context(|| {
    format!(
      "failed to read commands template: {}",
      commands_rs.display()
    )
  })?;

  contents = format!("{AUTOGENERATED_WARNING}{contents}");

  let crates = [Package::TauriPluginPinia];
  for krate in crates {
    let plugin_name = krate.plugin_name();
    let contents = contents.replace(PLUGIN_NAME, plugin_name);

    let dir = crate_commands_dir(krate);
    let autogenerated = dir.join("autogenerated.rs");
    fs::write(autogenerated, contents).with_context(|| {
      format!(
        "failed to write autogenerated commands for crate: {}",
        krate.as_ref()
      )
    })?;
  }

  Ok(())
}
