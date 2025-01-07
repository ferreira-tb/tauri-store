use crate::target::Target;
use std::path::PathBuf;

const ASSETS_DIR: &str = "crates/tauri-store-cli/assets";
const CRATES_DIR: &str = "crates";
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
join_dir!(packages_dir, PACKAGES_DIR);

pub fn crate_src_dir(krate: Target) -> PathBuf {
  crates_dir().join(krate.as_ref()).join("src")
}

pub fn crate_commands_dir(krate: Target) -> PathBuf {
  crate_src_dir(krate).join("command")
}

pub fn package_src_dir(package: Target) -> PathBuf {
  packages_dir().join(package.as_ref()).join("src")
}

pub fn package_commands_dir(package: Target) -> PathBuf {
  package_src_dir(package).join("commands")
}
