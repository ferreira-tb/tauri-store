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
- Svelte: <PluginLink path="./guide/getting-started" plugin="tauri-plugin-svelte">`tauri-plugin-svelte`</PluginLink>

## Optional features

- `file-sync-all`: Calls [`File::sync_all`](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all) after writing to the store file to ensure that all in-memory data reaches the filesystem. Enabling this can significantly degrade performance.

## Versioning

This crate follows [Cargo guidelines for SemVer compatibility](https://doc.rust-lang.org/cargo/reference/semver.html).

::: info Experimental features
[Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) prefixed with `unstable-` are experimental and may introduce breaking changes between patch versions or even be completely removed.
:::

## Any questions?

Feel free to start a discussion on the [GitHub repository](https://github.com/ferreira-tb/tauri-store/discussions) or ask in our [Discord server](https://discord.gg/ARd7McmVNv).
