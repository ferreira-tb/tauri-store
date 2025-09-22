use crate::store::StoreState;

pub type MarshalingError = Box<dyn std::error::Error + Send + Sync>;

/// Describes how stores should be serialized and deserialized.
pub trait Marshaler: Send + Sync {
  fn serialize(&self, state: &StoreState) -> Result<Vec<u8>, MarshalingError>;
  fn deserialize(&self, bytes: &[u8]) -> Result<StoreState, MarshalingError>;

  fn extension(&self) -> &'static str {
    "store"
  }
}

pub struct JsonMarshaler;

impl Marshaler for JsonMarshaler {
  fn serialize(&self, state: &StoreState) -> Result<Vec<u8>, MarshalingError> {
    Ok(serde_json::to_vec(state)?)
  }

  fn deserialize(&self, bytes: &[u8]) -> Result<StoreState, MarshalingError> {
    Ok(serde_json::from_slice(bytes)?)
  }

  fn extension(&self) -> &'static str {
    "json"
  }
}

pub struct PrettyJsonMarshaler;

impl Marshaler for PrettyJsonMarshaler {
  fn serialize(&self, state: &StoreState) -> Result<Vec<u8>, MarshalingError> {
    Ok(serde_json::to_vec_pretty(state)?)
  }

  fn deserialize(&self, bytes: &[u8]) -> Result<StoreState, MarshalingError> {
    Ok(serde_json::from_slice(bytes)?)
  }

  fn extension(&self) -> &'static str {
    "json"
  }
}

#[cfg(feature = "marshaler-toml")]
pub struct TomlMarshaler;

#[cfg(feature = "marshaler-toml")]
impl Marshaler for TomlMarshaler {
  fn serialize(&self, state: &StoreState) -> Result<Vec<u8>, MarshalingError> {
    Ok(toml::to_string(state)?.into_bytes())
  }

  fn deserialize(&self, bytes: &[u8]) -> Result<StoreState, MarshalingError> {
    Ok(toml::from_slice(bytes)?)
  }

  fn extension(&self) -> &'static str {
    "toml"
  }
}

#[cfg(feature = "marshaler-toml")]
pub struct PrettyTomlMarshaler;

#[cfg(feature = "marshaler-toml")]
impl Marshaler for PrettyTomlMarshaler {
  fn serialize(&self, state: &StoreState) -> Result<Vec<u8>, MarshalingError> {
    Ok(toml::to_string_pretty(state)?.into_bytes())
  }

  fn deserialize(&self, bytes: &[u8]) -> Result<StoreState, MarshalingError> {
    Ok(toml::from_slice(bytes)?)
  }

  fn extension(&self) -> &'static str {
    "toml"
  }
}
