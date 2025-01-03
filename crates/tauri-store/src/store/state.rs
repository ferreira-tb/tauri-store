use crate::error::Result;
use crate::io_err;
use serde::de::DeserializeOwned;
use serde_json::{from_value, json, Value as Json};
use std::collections::HashMap;

pub type StoreState = HashMap<String, Json>;

pub trait StoreStateExt {
  /// Tries to parse the store state as an instance of type `T`.
  fn parse<T: DeserializeOwned>(&self) -> Result<T>;
  /// Gets a value from the store and tries to parse it as an instance of type `T`.
  fn try_get<T: DeserializeOwned>(&self, key: impl AsRef<str>) -> Result<T>;
}

impl StoreStateExt for StoreState {
  fn parse<T: DeserializeOwned>(&self) -> Result<T> {
    Ok(from_value(json!(self))?)
  }

  fn try_get<T: DeserializeOwned>(&self, key: impl AsRef<str>) -> Result<T> {
    let key = key.as_ref();
    let Some(value) = self.get(key).cloned() else {
      return io_err!(NotFound, "key not found: {key}");
    };

    Ok(from_value(value)?)
  }
}
