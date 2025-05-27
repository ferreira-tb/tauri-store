use crate::CollectionMarker;

use super::save::SaveStrategy;
use super::Store;
use serde::{Deserialize, Serialize};
use tauri::Runtime;

/// Options to configure the store behavior.
#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreOptions {
  pub save_on_exit: Option<bool>,
  pub save_on_change: Option<bool>,
  pub save_strategy: Option<SaveStrategy>,
}

impl<R, C> From<&Store<R, C>> for StoreOptions
where
  R: Runtime,
  C: CollectionMarker,
{
  fn from(store: &Store<R, C>) -> Self {
    Self {
      save_on_exit: Some(store.save_on_exit),
      save_on_change: Some(store.save_on_change),
      save_strategy: store.save_strategy,
    }
  }
}

#[allow(clippy::needless_pass_by_value)]
pub(super) fn set_options<R, C>(store: &mut Store<R, C>, options: StoreOptions)
where
  R: Runtime,
  C: CollectionMarker,
{
  if let Some(enabled) = options.save_on_exit {
    store.save_on_exit = enabled;
  }

  if let Some(enabled) = options.save_on_change {
    store.save_on_change = enabled;
  }

  if let Some(strategy) = options.save_strategy {
    store.set_save_strategy(strategy);
  }
}
