---
layout: doc
title: Lifecycle hooks
description: Lifecycle hooks
---

# Lifecycle hooks

## JavaScript

JavaScript hooks can be registered using the [StoreHooks](https://tb.dev.br/tauri-store/js-docs/tauri-store/interfaces/StoreHooks.html) option.

```typescript
import { store } from 'tauri-store';

const value = { counter: 0 };
const counterStore = store('counter', value, {
  hooks: {
    error: (err) => console.error(err),
    beforeBackendSync: (state) => {
      console.log(state);
      return state;
    },
  },
});
```

### [`beforeBackendSync`](https://tb.dev.br/tauri-store/js-docs/tauri-store/interfaces/StoreHooks.html#beforebackendsync)

Registers a hook to be called before a store sends its state to Rust. This can be used to modify the state before it is sent to the backend.

```typescript
const hooks: StoreHooks = {
  beforeBackendSync: (state) => {
    state.foo = null;
    state.nested.bar.baz = 'qux';
    return state;
  },
};
```

### [`beforeFrontendSync`](https://tb.dev.br/tauri-store/js-docs/tauri-store/interfaces/StoreHooks.html#beforefrontendsync)

Registers a hook to be called before a store attempts to update itself with data from Rust. This can be used to modify the state before the changes are applied.

```typescript
const hooks: StoreHooks = {
  beforeFrontendSync: (state) => {
    state.foo = new Set(state.foo);
    state.bar = state.bar.map((value) => value * 2);
    return state;
  },
};
```

### [`error`](https://tb.dev.br/tauri-store/js-docs/tauri-store/interfaces/StoreHooks.html#error)

Registers a hook to be called when an error is thrown by a store.

```typescript
const hooks: StoreHooks = {
  error: (err) => console.error(err),
};
```

## Rust

### [`on_load`](https://docs.rs/tauri-store/latest/tauri_store/struct.Builder.html#method.on_load)

Registers a hook to be called when a store is loaded.

```rust
tauri_store::Builder::new()
  .on_load(|store| {
    println!("store loaded: {}", store.id());
    Ok(())
  })
  .build_plugin()
```
