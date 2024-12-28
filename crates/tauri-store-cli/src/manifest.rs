use semver::Version;
use serde::{Deserialize, Serialize};

pub trait Manifest {
  fn name(&self) -> &str;
  fn version(&self) -> &Version;
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
  name: String,
  version: Version,
}

impl Manifest for Package {
  fn name(&self) -> &str {
    &self.name
  }

  fn version(&self) -> &Version {
    &self.version
  }
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
  fn name(&self) -> &str {
    self.package.name()
  }

  fn version(&self) -> &Version {
    self.package.version()
  }
}
