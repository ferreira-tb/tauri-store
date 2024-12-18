use crate::package::Package;
use std::env;
use std::path::PathBuf;

const ASSETS_DIR: &str = "crates/tauri-store-cli/assets";
const CRATES_DIR: &str = "crates";
const PACKAGES_DIR: &str = "packages";

macro_rules! join_dir {
  ($func:ident, $dir:expr) => {
    pub fn $func() -> PathBuf {
      let path = env::current_dir()
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
join_dir!(packages_dir, PACKAGES_DIR);

pub fn crate_commands_dir(krate: Package) -> PathBuf {
  crates_dir()
    .join(krate.as_ref())
    .join("src/command")
}

pub fn package_commands_dir(package: Package) -> PathBuf {
  packages_dir()
    .join(package.as_ref())
    .join("src/commands")
}
