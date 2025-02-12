use crate::plugin::Plugin;
use std::path::PathBuf;

const ASSETS_DIR: &str = "crates/tauri-store-cli/assets";
const CRATES_DIR: &str = "crates";
const DOCS_CHANGELOG_DIR: &str = "docs/src/content/changelog";
const DOCS_DATA_DIR: &str = "docs/src/lib/data";
const EXAMPLES_DIR: &str = "examples";
const PACKAGES_DIR: &str = "packages";

macro_rules! join_dir {
  ($func:ident, $dir:expr) => {
    pub fn $func() -> PathBuf {
      let path = std::env::current_dir()
        .expect("failed to get current dir")
        .join($dir);

      if !matches!(path.try_exists(), Ok(true)) {
        panic!("path not found: {}", path.display());
      }

      path
    }
  };
}

join_dir!(assets_dir, ASSETS_DIR);
join_dir!(crates_dir, CRATES_DIR);
join_dir!(docs_changelog_dir, DOCS_CHANGELOG_DIR);
join_dir!(docs_data_dir, DOCS_DATA_DIR);
join_dir!(examples_dir, EXAMPLES_DIR);
join_dir!(packages_dir, PACKAGES_DIR);

pub fn crate_dir(plugin: Plugin) -> PathBuf {
  crates_dir().join(plugin.as_ref())
}

pub fn crate_src_dir(plugin: Plugin) -> PathBuf {
  crate_dir(plugin).join("src")
}

pub fn package_src_dir(plugin: Plugin) -> PathBuf {
  packages_dir().join(plugin.as_ref()).join("src")
}
