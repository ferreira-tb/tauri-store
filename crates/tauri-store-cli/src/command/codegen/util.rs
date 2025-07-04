pub mod replace {
  use crate::plugin::Plugin;
  use convert_case::{Case, Casing};

  pub fn store_collection(target: Plugin, case: Case) -> String {
    if let Plugin::Store = target {
      "store-collection".to_case(case)
    } else {
      target.title_as(case)
    }
  }
}

pub mod transform {
  const AUTOGENERATED_WARNING: &str = "
// THIS FILE WAS AUTOGENERATED AND SHOULD NOT BE EDITED MANUALLY.
//
// Check the `codegen` command in the `tauri-store-cli` crate.
// https://github.com/ferreira-tb/tauri-store/tree/main/crates/tauri-store-cli\n
";

  pub fn prepend_autogenerated(contents: &mut String) {
    let warning = AUTOGENERATED_WARNING.trim_start();
    contents.insert_str(0, warning);
  }

  pub fn remove_nocheck(contents: &mut String) {
    contents
      .replace("// @ts-nocheck", "")
      .trim_start()
      .clone_into(contents);
  }
}
