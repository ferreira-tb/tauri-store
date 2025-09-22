use super::{Context, Generator};
use crate::{
  path::{assets_dir, crate_android_dir, crate_ios_dir},
  plugin::Plugin,
};
use anyhow::Result;
use colored::Colorize;
use convert_case::Case;
use std::{
  path::{Path, PathBuf},
  time::Instant,
};

pub(super) fn generate() -> Result<()> {
  let start = Instant::now();
  println!("{}", "generating mobile code".cyan());

  let ios_assets = assets_plugin_ios_dir();
  generate_ios(&ios_assets)?;

  let android_assets = assets_plugin_android_dir();
  generate_android(&android_assets)?;

  let duration = start.elapsed();
  println!("{}", format!("done in {duration:?}").green());

  Ok(())
}

fn generate_ios(assets: &Path) -> Result<()> {
  generate_ios_plugin(assets)?;
  generate_ios_package(assets)?;
  generate_gitignore(assets)?;
  Ok(())
}

fn generate_ios_plugin(assets: &Path) -> Result<()> {
  let input = assets.join("Sources").join("StorePlugin.swift");
  let output = |ctx: Context<'_>| {
    let dir = crate_ios_dir(ctx.plugin);
    dir
      .join("Sources")
      .join(format!("{}Plugin.swift", ctx.plugin.title_as(Case::Pascal)))
      .into()
  };

  Generator::builder(&input, &output)
    .skip(&[Plugin::Core])
    .replace(&[
      ("__PASCAL_PLUGIN_TITLE__", &|it| it.title_as(Case::Pascal)),
      ("__SNAKE_PLUGIN_TITLE__", &|it| it.title_as(Case::Snake)),
      ("__REGISTERED_PLUGIN_NAME__", &|it| it.title_as(Case::Snake)),
    ])
    .generate()
}

fn generate_ios_package(assets: &Path) -> Result<()> {
  let input = assets.join("Package.swift");
  let output = |ctx: Context<'_>| {
    let dir = crate_ios_dir(ctx.plugin);
    dir.join("Package.swift").into()
  };

  Generator::builder(&input, &output)
    .skip(&[Plugin::Core])
    .replace(&[
      ("__PASCAL_PLUGIN_TITLE__", &|it| it.title_as(Case::Pascal)),
      ("__SNAKE_PLUGIN_TITLE__", &|it| it.title_as(Case::Snake)),
      ("__REGISTERED_PLUGIN_NAME__", &|it| it.title_as(Case::Snake)),
    ])
    .generate()
}

fn generate_android(assets: &Path) -> Result<()> {
  generate_android_build(assets)?;
  generate_android_plugin(assets)?;
  generate_android_manifest(assets)?;
  generate_android_settings_gradle(assets)?;
  generate_proguard_rules(assets)?;
  generate_gitignore(assets)?;
  Ok(())
}

fn generate_android_manifest(assets: &Path) -> Result<()> {
  let input = assets
    .join("src")
    .join("main")
    .join("AndroidManifest.xml");
  let output = |ctx: Context<'_>| {
    let dir = crate_android_dir(ctx.plugin);
    dir
      .join("src")
      .join("main")
      .join("AndroidManifest.xml")
      .into()
  };

  Generator::builder(&input, &output)
    .skip(&[Plugin::Core])
    .replace(&[
      ("__PASCAL_PLUGIN_TITLE__", &|it| it.title_as(Case::Pascal)),
      ("__SNAKE_PLUGIN_TITLE__", &|it| it.title_as(Case::Snake)),
      ("__REGISTERED_PLUGIN_NAME__", &|it| it.title_as(Case::Snake)),
    ])
    .generate()
}

fn generate_android_settings_gradle(assets: &Path) -> Result<()> {
  let input = assets.join("settings.gradle");
  let output = |ctx: Context<'_>| {
    let dir = crate_android_dir(ctx.plugin);
    dir.join("settings.gradle").into()
  };

  Generator::builder(&input, &output)
    .skip(&[Plugin::Core])
    .replace(&[
      ("__PASCAL_PLUGIN_TITLE__", &|it| it.title_as(Case::Pascal)),
      ("__SNAKE_PLUGIN_TITLE__", &|it| it.title_as(Case::Snake)),
      ("__REGISTERED_PLUGIN_NAME__", &|it| it.title_as(Case::Snake)),
    ])
    .generate()
}

fn generate_gitignore(assets: &Path) -> Result<()> {
  let input = assets.join(".gitignore");
  let output = |ctx: Context<'_>| {
    let dir = crate_android_dir(ctx.plugin);
    dir.join(".gitignore").into()
  };

  Generator::builder(&input, &output)
    .skip(&[Plugin::Core])
    .replace(&[
      ("__PASCAL_PLUGIN_TITLE__", &|it| it.title_as(Case::Pascal)),
      ("__SNAKE_PLUGIN_TITLE__", &|it| it.title_as(Case::Snake)),
      ("__REGISTERED_PLUGIN_NAME__", &|it| it.title_as(Case::Snake)),
    ])
    .generate()
}

fn generate_proguard_rules(assets: &Path) -> Result<()> {
  let input = assets.join("proguard-rules.pro");
  let output = |ctx: Context<'_>| {
    let dir = crate_android_dir(ctx.plugin);
    dir.join("proguard-rules.pro").into()
  };

  Generator::builder(&input, &output)
    .skip(&[Plugin::Core])
    .replace(&[
      ("__PASCAL_PLUGIN_TITLE__", &|it| it.title_as(Case::Pascal)),
      ("__SNAKE_PLUGIN_TITLE__", &|it| it.title_as(Case::Snake)),
      ("__REGISTERED_PLUGIN_NAME__", &|it| it.title_as(Case::Snake)),
    ])
    .generate()
}

fn generate_android_plugin(assets: &Path) -> Result<()> {
  let input = assets
    .join("src")
    .join("main")
    .join("java")
    .join("Plugin.kt");
  let output = |ctx: Context<'_>| {
    let dir = crate_android_dir(ctx.plugin);
    dir
      .join("src")
      .join("main")
      .join("java")
      .join(format!("{}Plugin.kt", ctx.plugin.title_as(Case::Pascal)))
      .into()
  };

  Generator::builder(&input, &output)
    .skip(&[Plugin::Core])
    .replace(&[
      ("__PASCAL_PLUGIN_TITLE__", &|it| it.title_as(Case::Pascal)),
      ("__SNAKE_PLUGIN_TITLE__", &|it| it.title_as(Case::Snake)),
      ("__REGISTERED_PLUGIN_NAME__", &|it| it.title_as(Case::Snake)),
    ])
    .generate()
}

fn generate_android_build(assets: &Path) -> Result<()> {
  let input = assets.join("build.gradle.kts");
  let output = |ctx: Context<'_>| {
    let dir = crate_android_dir(ctx.plugin);
    dir.join("build.gradle.kts").into()
  };

  Generator::builder(&input, &output)
    .skip(&[Plugin::Core])
    .replace(&[
      ("__PASCAL_PLUGIN_TITLE__", &|it| it.title_as(Case::Pascal)),
      ("__SNAKE_PLUGIN_TITLE__", &|it| it.title_as(Case::Snake)),
      ("__REGISTERED_PLUGIN_NAME__", &|it| it.title_as(Case::Snake)),
    ])
    .generate()
}

fn assets_plugin_ios_dir() -> PathBuf {
  assets_dir().join("ios")
}

fn assets_plugin_android_dir() -> PathBuf {
  assets_dir().join("android")
}
