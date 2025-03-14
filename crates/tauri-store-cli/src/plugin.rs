use crate::manifest::{Crate, Manifest};
use crate::path::crate_dir;
use anyhow::Result;
use convert_case::{Case, Casing};
use std::fs::read_to_string;
use strum::{AsRefStr, VariantArray};

const PLUGIN_PREFIX: &str = "tauri-plugin-";

#[derive(Clone, Copy, Debug, PartialEq, Eq, AsRefStr, VariantArray)]
pub enum Plugin {
  #[strum(serialize = "tauri-store")]
  Store,
  #[strum(serialize = "tauri-plugin-pinia")]
  Pinia,
  #[strum(serialize = "tauri-plugin-svelte")]
  Svelte,
  #[strum(serialize = "tauri-plugin-valtio")]
  Valtio,
}

impl Plugin {
  pub fn manifest(self) -> Result<Box<dyn Manifest>> {
    let path = crate_dir(self).join("Cargo.toml");
    let contents = read_to_string(path)?;
    toml::from_str::<Crate>(&contents)
      .map(Crate::boxed)
      .map_err(Into::into)
  }

  pub fn dir_name(self) -> String {
    let crate_name = self.crate_name();
    if let Self::Store = self {
      crate_name
    } else {
      crate_name
        .strip_prefix("tauri-")
        .unwrap()
        .to_owned()
    }
  }

  pub fn crate_name(self) -> String {
    self.as_ref().to_owned()
  }

  pub fn crate_name_as(self, case: Case) -> String {
    self.crate_name().to_case(case)
  }

  pub fn package_name(self) -> String {
    let crate_name = self.crate_name();
    if let Self::Store = self {
      crate_name
    } else {
      crate_name.replace(PLUGIN_PREFIX, "@tauri-store/")
    }
  }

  pub fn title(self) -> String {
    if let Self::Store = self {
      self.crate_name()
    } else {
      self
        .crate_name()
        .strip_prefix(PLUGIN_PREFIX)
        .unwrap()
        .to_owned()
    }
  }

  pub fn title_as(self, case: Case) -> String {
    self.title().to_case(case)
  }
}
