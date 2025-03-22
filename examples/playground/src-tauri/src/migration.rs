use tauri_plugin_pinia::Migration;

pub fn all() -> Vec<Migration> {
  vec![v1(), v2(), v3()]
}

fn v1() -> Migration {
  Migration::new("1.0.0", |state| {
    state.set("version", 1);
    state.set("foo", "foo_v1");
    Ok(())
  })
}

fn v2() -> Migration {
  Migration::new("2.0.0", |state| {
    state.set("version", 2);
    state.remove("foo");
    state.set("bar", "bar_v2");
    Ok(())
  })
}

fn v3() -> Migration {
  Migration::new("3.0.0", |state| {
    state.set("version", 3);
    state.remove("bar");
    Ok(())
  })
}
