use anyhow::Result;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value as Json};

pub trait Manifest {
  fn json(&self) -> Result<Json>;
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
  name: String,
  version: Version,
}

impl Package {
  pub fn boxed(self) -> Box<dyn Manifest> {
    Box::new(self)
  }
}

impl Manifest for Package {
  fn json(&self) -> Result<Json> {
    to_value(self).map_err(Into::into)
  }
}
