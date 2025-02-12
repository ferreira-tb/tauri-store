use crate::manifest::{Crate, Manifest};
use crate::path::crates_dir;
use anyhow::Result;
use convert_case::{Case, Casing};
use std::fs::read_to_string;
use strum::{AsRefStr, EnumString, VariantArray};

const PLUGIN_PREFIX: &str = "tauri-plugin-";

#[derive(Clone, Copy, Debug, PartialEq, Eq, AsRefStr, EnumString, VariantArray)]
pub enum Plugin {
  #[strum(serialize = "tauri-store", ascii_case_insensitive)]
  Store,

  #[strum(serialize = "tauri-plugin-pinia", ascii_case_insensitive)]
  Pinia,
  #[strum(serialize = "tauri-plugin-svelte", ascii_case_insensitive)]
  Svelte,
  #[strum(serialize = "tauri-plugin-valtio", ascii_case_insensitive)]
  Valtio,
}

impl Plugin {
  pub fn manifest(self) -> Result<Box<dyn Manifest>> {
    let path = crates_dir()
      .join(self.as_ref())
      .join("Cargo.toml");

    toml::from_str::<Crate>(&read_to_string(path)?)
      .map(Crate::boxed)
      .map_err(Into::into)
  }

  pub fn name(&self) -> &str {
    if let Self::Store = self {
      self.as_ref()
    } else {
      self.as_ref().strip_prefix(PLUGIN_PREFIX).unwrap()
    }
  }

  pub fn pascal_name(self) -> String {
    self.name().to_case(Case::Pascal)
  }

  pub fn snake_name(self) -> String {
    self.name().to_case(Case::Snake)
  }
}
