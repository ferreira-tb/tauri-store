use super::{Marshaler, MarshalingError};
use crate::store::StoreState;
use ciborium::de::from_reader;
use ciborium::ser::into_writer;

/// Serializes and deserializes CBOR stores.
pub struct CborMarshaler;

impl Marshaler for CborMarshaler {
  fn serialize(&self, state: &StoreState) -> Result<Vec<u8>, MarshalingError> {
    let mut buf = Vec::new();
    into_writer(state, &mut buf)?;
    Ok(buf)
  }

  fn deserialize(&self, bytes: &[u8]) -> Result<StoreState, MarshalingError> {
    Ok(from_reader(bytes)?)
  }
}
