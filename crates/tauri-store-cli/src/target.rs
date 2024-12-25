use crate::manifest::{Manifest, Package};
use crate::path::packages_dir;
use anyhow::Result;
use serde_json::from_slice;
use std::fs::read;
use strum::{AsRefStr, VariantArray};

#[derive(Clone, Copy, Debug, AsRefStr, VariantArray)]
#[strum(serialize_all = "kebab-case")]
#[remain::sorted]
pub enum Target {
  TauriPluginPinia,
  TauriPluginSvelte,
}

impl Target {
  pub fn manifest(&self) -> Result<Box<dyn Manifest>> {
    let path = packages_dir()
      .join(self.as_ref())
      .join("package.json");

    from_slice::<Package>(&read(path)?)
      .map(Package::boxed)
      .map_err(Into::into)
  }

  pub fn plugin_name(&self) -> &str {
    match self {
      Self::TauriPluginPinia => "pinia",
      Self::TauriPluginSvelte => "svelte",
    }
  }
}
