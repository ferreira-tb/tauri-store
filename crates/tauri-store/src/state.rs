use crate::error::Result;
use crate::io_err;
use serde::de::DeserializeOwned;
use serde_json::{from_value, json, Value as Json};

#[cfg(feature = "ahash")]
use ahash::HashMap;
#[cfg(not(feature = "ahash"))]
use std::collections::HashMap;

pub type StoreState = HashMap<String, Json>;

pub trait StoreStateExt {
  fn parse<T: DeserializeOwned>(self) -> Result<T>;
  fn get_owned(&self, key: impl AsRef<str>) -> Option<Json>;
  fn try_get<T: DeserializeOwned>(&self, key: impl AsRef<str>) -> Result<T>;
}

impl StoreStateExt for StoreState {
  fn parse<T: DeserializeOwned>(self) -> Result<T> {
    from_value(json!(self)).map_err(Into::into)
  }

  fn get_owned(&self, key: impl AsRef<str>) -> Option<Json> {
    self.get(key.as_ref()).cloned()
  }

  fn try_get<T: DeserializeOwned>(&self, key: impl AsRef<str>) -> Result<T> {
    let Some(value) = self.get_owned(&key) else {
      return io_err!(NotFound, "key not found: {}", key.as_ref());
    };

    from_value(value).map_err(Into::into)
  }
}
