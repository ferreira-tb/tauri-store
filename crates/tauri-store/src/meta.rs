use crate::store::StoreId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(debug_assertions)]
const FILENAME: &str = "meta.dev.tauristore";
#[cfg(not(debug_assertions))]
const FILENAME: &str = "meta.tauristore";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Meta {
  path: HashMap<StoreId, PathBuf>,
}
