use crate::manifest::{Crate, Manifest};
use crate::path::crates_dir;
use anyhow::Result;
use std::fs::read_to_string;
use strum::{AsRefStr, VariantArray};

const PLUGIN_PREFIX: &str = "tauri-plugin-";

#[derive(Clone, Copy, Debug, AsRefStr, VariantArray)]
#[remain::sorted]
pub enum Target {
  #[strum(serialize = "tauri-plugin-pinia")]
  PluginPinia,
  #[strum(serialize = "tauri-plugin-svelte")]
  PluginSvelte,
  #[strum(serialize = "tauri-plugin-valtio")]
  PluginValtio,
  #[strum(serialize = "tauri-store")]
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

  pub fn is_plugin(&self) -> bool {
    self.as_ref().starts_with(PLUGIN_PREFIX)
  }

  pub fn plugin_name(&self) -> Option<&str> {
    self.as_ref().strip_prefix(PLUGIN_PREFIX)
  }
}
