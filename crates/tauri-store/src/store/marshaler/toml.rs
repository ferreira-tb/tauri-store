use super::{Marshaler, MarshalingError};
use crate::store::StoreState;

/// Serializes and deserializes TOML stores.
pub struct TomlMarshaler;

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

/// Serializes and deserializes pretty TOML stores.
pub struct PrettyTomlMarshaler;

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
