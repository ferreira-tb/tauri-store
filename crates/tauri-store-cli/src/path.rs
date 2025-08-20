use crate::plugin::Plugin;
use std::path::PathBuf;

macro_rules! join_dir {
  ($func:ident, $dir:expr) => {
    pub fn $func() -> PathBuf {
      let path = std::env::current_dir()
        .expect("failed to get current dir")
        .join($dir);

      if !path.try_exists().unwrap_or(false) {
        panic!("path not found: {}", path.display());
      }

      path
    }
  };
}

join_dir!(assets_dir, "crates/tauri-store-cli/assets");
join_dir!(crates_dir, "crates");
join_dir!(examples_dir, "examples");
join_dir!(packages_dir, "packages");

pub fn crate_dir(plugin: Plugin) -> PathBuf {
  crates_dir().join(plugin.dir_name())
}

pub fn crate_src_dir(plugin: Plugin) -> PathBuf {
  crate_dir(plugin).join("src")
}

pub fn package_src_dir(plugin: Plugin) -> PathBuf {
  packages_dir().join(plugin.dir_name()).join("src")
}

pub fn crate_ios_dir(plugin: Plugin) -> PathBuf {
  crate_dir(plugin).join("ios")
}

pub fn crate_android_dir(plugin: Plugin) -> PathBuf {
  crate_dir(plugin).join("android")
}
