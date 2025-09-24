use super::{Marshaler, MarshalingError};
use crate::store::StoreState;
use ron::de::from_bytes;
use ron::ser::{to_string, to_string_pretty, PrettyConfig};

/// Serializes and deserializes RON stores.
pub struct RonMarshaler;

impl Marshaler for RonMarshaler {
  fn serialize(&self, state: &StoreState) -> Result<Vec<u8>, MarshalingError> {
    Ok(to_string(state)?.into_bytes())
  }

  fn deserialize(&self, bytes: &[u8]) -> Result<StoreState, MarshalingError> {
    Ok(from_bytes(bytes)?)
  }

  fn extension(&self) -> &'static str {
    "ron"
  }
}

/// Serializes and deserializes pretty RON stores.
pub struct PrettyRonMarshaler;

impl Marshaler for PrettyRonMarshaler {
  fn serialize(&self, state: &StoreState) -> Result<Vec<u8>, MarshalingError> {
    let config = PrettyConfig::default();
    Ok(to_string_pretty(state, config)?.into_bytes())
  }

  fn deserialize(&self, bytes: &[u8]) -> Result<StoreState, MarshalingError> {
    Ok(from_bytes(bytes)?)
  }

  fn extension(&self) -> &'static str {
    "ron"
  }
}
