use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as Json;
use std::collections::HashMap;
use std::result::Result as StdResult;

/// Internal state of a store.
#[derive(Clone, Debug, Default)]
pub struct StoreState(pub(super) HashMap<String, Json>);

impl StoreState {
  /// Consumes the store state and returns the inner map.
  pub fn into_inner(self) -> HashMap<String, Json> {
    self.0
  }
}

impl Serialize for StoreState {
  fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.0.serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for StoreState {
  fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    type Map = HashMap<String, Json>;
    Ok(Self(Map::deserialize(deserializer)?))
  }
}

impl From<HashMap<String, Json>> for StoreState {
  fn from(map: HashMap<String, Json>) -> Self {
    Self(map)
  }
}

impl<K, V> FromIterator<(K, V)> for StoreState
where
  K: Into<String>,
  V: Into<Json>,
{
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = (K, V)>,
  {
    let inner = iter
      .into_iter()
      .map(|(k, v)| (k.into(), v.into()))
      .collect();

    Self(inner)
  }
}

impl<K, V> From<(K, V)> for StoreState
where
  K: Into<String>,
  V: Into<Json>,
{
  fn from((key, value): (K, V)) -> Self {
    Self::from_iter([(key, value)])
  }
}

impl<K, V> From<Vec<(K, V)>> for StoreState
where
  K: Into<String>,
  V: Into<Json>,
{
  fn from(pairs: Vec<(K, V)>) -> Self {
    Self::from_iter(pairs)
  }
}

impl<const N: usize, K, V> From<[(K, V); N]> for StoreState
where
  K: Into<String>,
  V: Into<Json>,
{
  fn from(pairs: [(K, V); N]) -> Self {
    Self::from_iter(pairs)
  }
}
