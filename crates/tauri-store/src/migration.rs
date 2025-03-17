use crate::store::StoreId;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct MigrationHistory(HashMap<StoreId, Version>);
