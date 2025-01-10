use crate::target::Target;
use std::path::PathBuf;

const ASSETS_DIR: &str = "crates/tauri-store-cli/assets";
const CRATES_DIR: &str = "crates";
const EXAMPLES_SHARED_RUST_DIR: &str = "examples/shared/rust";
const DOCS_DATA_DIR: &str = "docs/src/lib/data";
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
join_dir!(docs_data_dir, DOCS_DATA_DIR);
join_dir!(examples_shared_rust_dir, EXAMPLES_SHARED_RUST_DIR);
join_dir!(packages_dir, PACKAGES_DIR);

pub fn assets_examples_dir() -> PathBuf {
  assets_dir().join("examples")
}

pub fn assets_plugin_rs_dir() -> PathBuf {
  assets_dir().join("plugin-rs")
}

pub fn assets_plugin_ts_dir() -> PathBuf {
  assets_dir().join("plugin-ts")
}

pub fn crate_src_dir(krate: Target) -> PathBuf {
  crates_dir().join(krate.as_ref()).join("src")
}

pub fn crate_commands_dir(krate: Target) -> PathBuf {
  crate_src_dir(krate).join("command")
}

pub fn examples_shared_rust_impl_dir() -> PathBuf {
  examples_shared_rust_dir().join("src/impl")
}

pub fn package_src_dir(package: Target) -> PathBuf {
  packages_dir().join(package.as_ref()).join("src")
}

pub fn package_commands_dir(package: Target) -> PathBuf {
  package_src_dir(package).join("commands")
}
