use super::{Marshaler, MarshalingError};
use crate::store::StoreState;

/// Serializes and deserializes JSON stores.
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

/// Serializes and deserializes pretty JSON stores.
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
