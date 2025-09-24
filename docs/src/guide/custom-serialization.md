---
layout: doc
title: Custom serialization
description: Custom serialization and deserialization
---

# Custom serialization

By default, all stores are serialized and deserialized as JSON using the [`serde_json`](https://docs.rs/serde_json/latest/serde_json/index.html) crate. You can customize this behavior by changing the [`Marshaler`](https://docs.rs/tauri-store/latest/tauri_store/trait.Marshaler.html) that the plugin uses.

::: code-group

```rust [src-tauri/src/lib.rs]
use tauri_store::{PrettyJsonMarshaler, TomlMarshaler};

tauri_store::Builder::new()
  // Sets the default marshaler.
  .marshaler(Box::new(PrettyJsonMarshaler))
  // Sets the marshaler for a specific store.
  .marshaler_of("my-store", Box::new(TomlMarshaler))
  .build_plugin();
```

:::

Currently, the following marshalers are available:

- [`CborMarshaler`](https://docs.rs/tauri-store/latest/tauri_store/struct.CborMarshaler.html)
- [`JsonMarshaler`](https://docs.rs/tauri-store/latest/tauri_store/struct.JsonMarshaler.html)
- [`PrettyJsonMarshaler`](https://docs.rs/tauri-store/latest/tauri_store/struct.PrettyJsonMarshaler.html)
- [`PrettyRonMarshaler`](https://docs.rs/tauri-store/latest/tauri_store/struct.PrettyRonMarshaler.html)
- [`PrettyTomlMarshaler`](https://docs.rs/tauri-store/latest/tauri_store/struct.PrettyTomlMarshaler.html)
- [`RonMarshaler`](https://docs.rs/tauri-store/latest/tauri_store/struct.RonMarshaler.html)
- [`TomlMarshaler`](https://docs.rs/tauri-store/latest/tauri_store/struct.TomlMarshaler.html)

## Custom marshaler

You can also implement your own marshaler to serialize and deserialize it in any way you prefer. This is particularly useful if, for instance, you want to encrypt the stored data.

```rust
use tauri_store::{Marshaler, MarshalingError, StoreState};

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
