use crate::manifest::{Crate, Manifest};
use crate::path::crates_dir;
use anyhow::Result;
use std::fs::read_to_string;
use strum::{AsRefStr, EnumString, VariantArray};

const PLUGIN_PREFIX: &str = "tauri-plugin-";

#[derive(Clone, Copy, Debug, AsRefStr, EnumString, VariantArray)]
#[remain::sorted]
pub enum Target {
  #[strum(serialize = "tauri-plugin-pinia", ascii_case_insensitive)]
  PluginPinia,
  #[strum(serialize = "tauri-plugin-svelte", ascii_case_insensitive)]
  PluginSvelte,
  #[strum(serialize = "tauri-plugin-valtio", ascii_case_insensitive)]
  PluginValtio,
  #[strum(serialize = "tauri-store", ascii_case_insensitive)]
  Store,
}

impl Target {
  pub fn manifest(self) -> Result<Box<dyn Manifest>> {
    let path = crates_dir()
      .join(self.as_ref())
      .join("Cargo.toml");

    toml::from_str::<Crate>(&read_to_string(path)?)
      .map(Crate::boxed)
      .map_err(Into::into)
  }

  pub fn is_plugin(self) -> bool {
    self.as_ref().starts_with(PLUGIN_PREFIX)
  }

  pub fn plugin_name(&self) -> Option<&str> {
    self.as_ref().strip_prefix(PLUGIN_PREFIX)
  }
}
