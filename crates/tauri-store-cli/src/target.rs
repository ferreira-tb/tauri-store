use crate::manifest::{Crate, Manifest};
use crate::path::crates_dir;
use anyhow::Result;
use std::fs::read_to_string;
use strum::{AsRefStr, VariantArray};

#[derive(Clone, Copy, Debug, AsRefStr, VariantArray)]
#[strum(serialize_all = "kebab-case")]
#[remain::sorted]
pub enum Target {
  TauriPluginPinia,
  TauriPluginSvelte,
  TauriStore,
}

impl Target {
  pub fn manifest(&self) -> Result<Box<dyn Manifest>> {
    let path = crates_dir()
      .join(self.as_ref())
      .join("Cargo.toml");

    toml::from_str::<Crate>(&read_to_string(path)?)
      .map(Crate::boxed)
      .map_err(Into::into)
  }

  pub fn plugin_name(&self) -> Option<&str> {
    let name = match self {
      Self::TauriPluginPinia => "pinia",
      Self::TauriPluginSvelte => "svelte",
      Self::TauriStore => return None,
    };

    Some(name)
  }
}
