---
layout: doc
title: Custom serialization
description: Custom serialization and deserialization
---

# Custom serialization

By default, all stores are serialized and deserialized as JSON using the [`serde_json`](https://docs.rs/serde_json/latest/serde_json/index.html) crate. You can customize this behavior by changing the [`Marshaler`](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/trait.Marshaler.html) that the plugin uses.

::: code-group

```rust [src-tauri/src/lib.rs]
use tauri_plugin_pinia::{PrettyJsonMarshaler, TomlMarshaler};

tauri_plugin_pinia::Builder::new()
  // Sets the default marshaler.
  .marshaler(Box::new(PrettyJsonMarshaler))
  // Sets the marshaler for a specific store.
  .marshaler_of("my-store", Box::new(TomlMarshaler))
  .build();
```

:::

Currently, the following marshalers are available:

- [`JsonMarshaler`](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/struct.JsonMarshaler.html)
- [`PrettyJsonMarshaler`](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/struct.PrettyJsonMarshaler.html)
- [`PrettyTomlMarshaler`](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/struct.PrettyTomlMarshaler.html)
- [`TomlMarshaler`](https://docs.rs/tauri-plugin-pinia/latest/tauri_plugin_pinia/struct.TomlMarshaler.html)

## Custom marshaler

You can also implement your own marshaler to serialize and deserialize it in any way you prefer. This is particularly useful if, for instance, you want to encrypt the stored data.

```rust
use tauri_plugin_pinia::{Marshaler, MarshalingError, StoreState};

struct SecureMarshaler;

impl Marshaler for SecureMarshaler {
  fn serialize(&self, state: &StoreState) -> Result<Vec<u8>, MarshalingError> {
    Ok(secure_serializer_fn(state)?)
  }

  fn deserialize(&self, bytes: &[u8]) -> Result<StoreState, MarshalingError> {
    Ok(secure_deserializer_fn(bytes)?)
  }
}
```
