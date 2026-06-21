use super::save::SaveStrategy;
use super::Store;
use crate::CollectionMarker;
use serde::{Deserialize, Serialize};
use tauri::Runtime;

/// Keys to filter when saving the store.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StoreKeyFilter {
  /// A single key to filter.
  One(String),
  /// Multiple keys to filter.
  Many(Vec<String>),
}

impl StoreKeyFilter {
  /// Returns `true` if the given `key` matches this filter.
  pub fn matches(&self, key: &str) -> bool {
    match self {
      Self::One(k) => k.as_str() == key,
      Self::Many(keys) => keys.iter().any(|k| k.as_str() == key),
    }
  }
}

/// Strategy for filtering store keys.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StoreKeyFilterStrategy {
  /// All keys will be saved **except** the ones specified.
  #[default]
  Omit,
  /// Only the specified keys will be saved.
  Pick,
}

/// Options to configure the store behavior.
#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreOptions {
  pub save_on_exit: Option<bool>,
  pub save_on_change: Option<bool>,
  pub save_strategy: Option<SaveStrategy>,
  /// Keys to exclude from persistence to disk.
  /// These keys will still be synced across windows.
  pub save_filter_keys: Option<StoreKeyFilter>,
  /// Strategy to use when filtering keys during save.
  pub save_filter_keys_strategy: Option<StoreKeyFilterStrategy>,
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
      save_filter_keys: store.save_filter_keys.clone(),
      save_filter_keys_strategy: Some(store.save_filter_keys_strategy),
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

  if let Some(keys) = options.save_filter_keys {
    store.save_filter_keys = Some(keys);
  }

  if let Some(strategy) = options.save_filter_keys_strategy {
    store.save_filter_keys_strategy = strategy;
  }
}
