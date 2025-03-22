use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use std::sync::Arc;

/// Unique identifier for a store.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StoreId(Arc<str>);

impl StoreId {
  pub fn new(id: &str) -> Self {
    Self::from(id)
  }
}

impl AsRef<str> for StoreId {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl Clone for StoreId {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}

impl From<&StoreId> for StoreId {
  fn from(value: &StoreId) -> Self {
    value.clone()
  }
}

impl From<&str> for StoreId {
  fn from(id: &str) -> Self {
    Self(Arc::from(id))
  }
}

impl From<String> for StoreId {
  fn from(id: String) -> Self {
    Self(Arc::from(id))
  }
}

impl From<&String> for StoreId {
  fn from(id: &String) -> Self {
    Self(Arc::from(id.as_str()))
  }
}

impl From<Arc<str>> for StoreId {
  fn from(id: Arc<str>) -> Self {
    Self::from(id.as_ref())
  }
}

impl From<Box<str>> for StoreId {
  fn from(id: Box<str>) -> Self {
    Self::from(id.as_ref())
  }
}

impl From<Cow<'_, str>> for StoreId {
  fn from(id: Cow<'_, str>) -> Self {
    Self::from(id.as_ref())
  }
}

impl fmt::Display for StoreId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
