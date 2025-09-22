mod json;

#[cfg(feature = "marshaler-toml")]
mod toml;

use crate::store::StoreState;

pub use json::{JsonMarshaler, PrettyJsonMarshaler};

#[cfg(feature = "marshaler-toml")]
pub use toml::{PrettyTomlMarshaler, TomlMarshaler};

/// Generic marshaling error.
pub type MarshalingError = Box<dyn std::error::Error + Send + Sync>;

/// Describes how stores should be serialized and deserialized.
pub trait Marshaler: Send + Sync {
  fn serialize(&self, state: &StoreState) -> Result<Vec<u8>, MarshalingError>;
  fn deserialize(&self, bytes: &[u8]) -> Result<StoreState, MarshalingError>;

  /// Extension with which the store will be saved. Defaults to `store`.
  fn extension(&self) -> &'static str {
    "store"
  }
}
