use crate::store::{Marshaler, StoreId};
use std::collections::HashMap;
use std::path::Path;

// Usually, we place store-specific information on the store themselves,
// but there are things for which that doesn't really work.
//
// For example, we need to know how to deserialize a store **before** loading it.

pub(crate) struct PathTable {
  pub(crate) default: Box<Path>,
  pub(crate) table: HashMap<StoreId, Box<Path>>,
}

impl PathTable {
  pub fn get(&self, store_id: &StoreId) -> &Path {
    self
      .table
      .get(store_id)
      .map(AsRef::as_ref)
      .unwrap_or_else(|| self.default.as_ref())
  }
}

pub(crate) struct MarshalerTable {
  pub(crate) default: Box<dyn Marshaler>,
  pub(crate) table: HashMap<StoreId, Box<dyn Marshaler>>,
}

impl MarshalerTable {
  pub fn get(&self, store_id: &StoreId) -> &dyn Marshaler {
    self
      .table
      .get(store_id)
      .map(AsRef::as_ref)
      .unwrap_or_else(|| self.default.as_ref())
  }
}
