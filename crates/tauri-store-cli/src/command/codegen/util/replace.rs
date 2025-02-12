use crate::plugin::Plugin;
use convert_case::{Case, Casing};

pub fn store_collection(target: Plugin, case: Case) -> String {
  let collection = if let Plugin::Store = target {
    "store-collection"
  } else {
    target.name()
  };

  collection.to_case(case)
}
