# tauri-store

Persistent stores for Tauri.

## Features

- Save your stores to disk.
- Synchronize across multiple windows.
- Debounce or throttle store updates.
- Access the stores from both JavaScript and Rust.

## Framework support

The [`tauri-store`](https://docs.rs/tauri-store/latest/tauri_store/) crate is a framework-agnostic backend for store plugins. Currently, the following plugins are available:

- Pinia (Vue or Nuxt): <PluginLink path="./guide/getting-started" plugin="tauri-plugin-pinia">`tauri-plugin-pinia`</PluginLink>

## Optional features

- `file-sync-all`: Calls [`File::sync_all`](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all) after writing to the store file. Enabling this can severely degrade performance.
- `unstable-async`: Enables async support for the plugin.

## Versioning

This crate follows [Cargo guidelines for SemVer compatibility](https://doc.rust-lang.org/cargo/reference/semver.html).

::: warning Experimental features
[Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) prefixed with `unstable-` (e.g. `unstable-async`) are experimental and may introduce breaking changes between patch versions or even be completely removed.
:::

## Any questions?

Feel free to start a discussion on the [GitHub repository](https://github.com/ferreira-tb/tauri-store/discussions) or ask in our [Discord server](https://discord.gg/ARd7McmVNv).
