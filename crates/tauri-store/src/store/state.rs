use crate::error::Result;
use crate::io_err;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::result::Result as StdResult;

/// Internal state of a store.
#[derive(Clone, Debug, Default)]
pub struct StoreState(HashMap<String, Value>);

impl StoreState {
  /// Creates an empty [`StoreState`].
  pub fn new() -> Self {
    Self::default()
  }

  /// Creates an empty [`StoreState`] with at least the specified capacity.
  pub fn with_capacity(capacity: usize) -> Self {
    Self(HashMap::with_capacity(capacity))
  }

  /// Consumes the [`StoreState`] and returns the inner [`HashMap`](std::collections::HashMap).
  #[inline]
  pub fn into_inner(self) -> HashMap<String, Value> {
    self.0
  }

  /// Gets a reference to the raw value corresponding to the key.
  pub fn get_raw(&self, key: impl AsRef<str>) -> Option<&Value> {
    self.0.get(key.as_ref())
  }

  /// Gets a mutable reference to the raw value corresponding to the key.
  pub fn get_raw_mut(&mut self, key: impl AsRef<str>) -> Option<&mut Value> {
    self.0.get_mut(key.as_ref())
  }

  /// Gets a value and tries to parse it as an instance of type `T`.
  pub fn get<T>(&self, key: impl AsRef<str>) -> Result<T>
  where
    T: DeserializeOwned,
  {
    let key = key.as_ref();
    let Some(value) = self.0.get(key).cloned() else {
      return io_err!(NotFound, "key not found: {key}");
    };

    Ok(serde_json::from_value(value)?)
  }

  /// Gets a value and tries to parse it as an instance of type `T`.
  ///
  /// If it does not exist, returns the provided default value.
  pub fn get_or<T>(&self, key: impl AsRef<str>, default: T) -> T
  where
    T: DeserializeOwned,
  {
    self.get(key).unwrap_or(default)
  }

  /// Gets a value and tries to parse it as an instance of type `T`.
  ///
  /// If it does not exist, returns the default value of `T`.
  pub fn get_or_default<T>(&self, key: impl AsRef<str>) -> T
  where
    T: DeserializeOwned + Default,
  {
    self.get(key).unwrap_or_default()
  }

  /// Gets a value and tries to parse it as an instance of type `T`.
  ///
  /// If it does not exist, returns the result of the provided closure.
  pub fn get_or_else<T>(&self, key: impl AsRef<str>, f: impl FnOnce() -> T) -> T
  where
    T: DeserializeOwned,
  {
    self.get(key).unwrap_or_else(|_| f())
  }

  /// Sets a key-value pair, returning the previous value, if any.
  pub fn set(&mut self, key: impl AsRef<str>, value: impl Into<Value>) -> Option<Value> {
    let key = key.as_ref().to_owned();
    self.0.insert(key, value.into())
  }

  /// Patches the state.
  pub fn patch(&mut self, state: impl Into<StoreState>) {
    self.0.extend(state.into().0);
  }

  /// Whether a key exists.
  pub fn has(&self, key: impl AsRef<str>) -> bool {
    self.0.contains_key(key.as_ref())
  }

  /// Creates an iterator over the keys.
  pub fn keys(&self) -> impl Iterator<Item = &String> {
    self.0.keys()
  }

  /// Creates an iterator over the values.
  pub fn values(&self) -> impl Iterator<Item = &Value> {
    self.0.values()
  }

  /// Creates an iterator over mutable references to the values.
  pub fn values_mut(&mut self) -> impl Iterator<Item = &mut Value> {
    self.0.values_mut()
  }

  /// Creates an iterator over the entries.
  pub fn entries(&self) -> impl Iterator<Item = (&String, &Value)> {
    self.0.iter()
  }

  /// Creates an iterator over mutable references to the entries.
  pub fn entries_mut(&mut self) -> impl Iterator<Item = (&String, &mut Value)> {
    self.0.iter_mut()
  }

  /// Removes a key, returning the previous value, if any.
  pub fn remove(&mut self, key: impl AsRef<str>) -> Option<Value> {
    self.0.remove(key.as_ref())
  }

  /// Retains only the values specified by the predicate.
  pub fn retain<F>(&mut self, f: F)
  where
    F: FnMut(&String, &mut Value) -> bool,
  {
    self.0.retain(f);
  }

  /// Clears the store state, removing all key-value pairs.
  #[inline]
  pub fn clear(&mut self) {
    self.0.clear();
  }

  /// Returns the amount of items.
  #[inline]
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Whether it is empty.
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
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
    type Map = HashMap<String, Value>;
    Ok(Self(Map::deserialize(deserializer)?))
  }
}

impl From<HashMap<String, Value>> for StoreState {
  fn from(map: HashMap<String, Value>) -> Self {
    Self(map)
  }
}

impl<K, V> FromIterator<(K, V)> for StoreState
where
  K: Into<String>,
  V: Into<Value>,
{
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = (K, V)>,
  {
    let state = iter
      .into_iter()
      .map(|(k, v)| (k.into(), v.into()))
      .collect();

    Self(state)
  }
}

impl<K, V> From<(K, V)> for StoreState
where
  K: Into<String>,
  V: Into<Value>,
{
  fn from((key, value): (K, V)) -> Self {
    Self::from_iter([(key, value)])
  }
}

impl<K, V> From<Vec<(K, V)>> for StoreState
where
  K: Into<String>,
  V: Into<Value>,
{
  fn from(pairs: Vec<(K, V)>) -> Self {
    Self::from_iter(pairs)
  }
}

impl<const N: usize, K, V> From<[(K, V); N]> for StoreState
where
  K: Into<String>,
  V: Into<Value>,
{
  fn from(pairs: [(K, V); N]) -> Self {
    Self::from_iter(pairs)
  }
}

impl From<StoreState> for Value {
  fn from(state: StoreState) -> Self {
    Value::from(Map::from_iter(state.0))
  }
}

impl From<&StoreState> for Value {
  fn from(state: &StoreState) -> Self {
    Value::from(state.clone())
  }
}
