use semver::Version;
use serde::{Deserialize, Serialize};

pub trait Manifest {
  fn version(&self) -> Version;
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Crate {
  package: Package,
}

impl Crate {
  pub fn boxed(self) -> Box<dyn Manifest> {
    Box::new(self)
  }
}

impl Manifest for Crate {
  fn version(&self) -> Version {
    self.package.version()
  }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
  version: Version,
}

impl Manifest for Package {
  fn version(&self) -> Version {
    self.version.clone()
  }
}
