---
layout: doc
title: Migration
description: Defining store migrations
---

# Migration

You can define migrations to change the structure of your stores between versions. This is the recommended way to remove key-value pairs from a store.

::: code-group

```rust [src-tauri/src/lib.rs]
use tauri_plugin_svelte::Migration;

tauri_plugin_svelte::Builder::new()
  .migrations([v1(), v2(), v3()])
  .build();

fn v1() -> Migration {
  Migration::new("1.0.0", |state| {
    state.set("foo", "hello");
    Ok(())
  })
}

fn v2() -> Migration {
  Migration::new("2.0.0", |state| {
    state.remove("foo");
    state.set("bar", "你好");
    Ok(())
  })
}

fn v3() -> Migration {
  Migration::new("3.0.0", |state| {
    state.clear();
    state.set("baz", "olá");
    Ok(())
  })
}
```

:::

::: tip
For a working example, check the [`examples`](https://github.com/ferreira-tb/tauri-store/tree/main/examples/migration) directory in our repository.
:::
