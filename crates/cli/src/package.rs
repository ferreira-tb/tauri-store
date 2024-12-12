use strum::AsRefStr;

#[derive(Clone, Copy, Debug, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum Package {
  TauriPluginPinia,
}

impl Package {
  pub fn plugin_name(&self) -> &str {
    match self {
      Self::TauriPluginPinia => "pinia",
    }
  }
}
