---
layout: doc
title: tauri-store
description: Persistent stores for Tauri
---

# tauri-store

Persistent stores for Tauri.

## Features

- Save your stores to disk.
- Synchronize across multiple windows.
- Debounce or throttle store updates.
- Access the stores from both JavaScript and Rust.

## Framework support

The [`tauri-store`](https://crates.io/crates/tauri-store) crate is a framework-agnostic backend for store plugins. Currently, the following plugins are available:

| Name                                                              | Version                                                                                                                 | Works with |
| ----------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- | ---------- |
| [tauri-store](./guide/getting-started.md)                         | [![npm](https://img.shields.io/npm/v/tauri-store.svg)](https://www.npmjs.com/package/tauri-store)                       | Everything |
| [@tauri-store/pinia](./plugin-pinia/guide/getting-started.md)     | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fpinia.svg)](https://www.npmjs.com/package/@tauri-store/pinia)     | Vue, Nuxt  |
| [@tauri-store/svelte](./plugin-svelte/guide/getting-started.md)   | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fsvelte.svg)](https://www.npmjs.com/package/@tauri-store/svelte)   | Svelte     |
| [@tauri-store/valtio](./plugin-valtio/guide/getting-started.md)   | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fvaltio.svg)](https://www.npmjs.com/package/@tauri-store/valtio)   | React      |
| [@tauri-store/vue](./plugin-vue/guide/getting-started.md)         | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fvue.svg)](https://www.npmjs.com/package/@tauri-store/vue)         | Vue, Nuxt  |
| [@tauri-store/zustand](./plugin-zustand/guide/getting-started.md) | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fzustand.svg)](https://www.npmjs.com/package/@tauri-store/zustand) | React      |

## Examples

You can find example projects in the [examples directory](https://github.com/ferreira-tb/tauri-store/tree/main/examples) of the GitHub repository. To check them out, run the following commands:

```sh
git clone https://github.com/ferreira-tb/tauri-store.git
cd tauri-store
pnpm install
pnpm run example [EXAMPLE_NAME]
```

For example, to run the [`pinia`](https://github.com/ferreira-tb/tauri-store/tree/main/examples/pinia) project, you can use:

```sh
pnpm run example pinia
```

For a completely random example, run:

```sh
pnpm run example random
```

## Cargo features

You can enable some [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) to customize the plugin's behavior.

- `file-sync-all`: call [`File::sync_all`](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all) after writing to the store file to ensure that all in-memory data reaches the filesystem. Enabling this can significantly degrade performance.

## Versioning

This crate follows [Cargo guidelines](https://doc.rust-lang.org/cargo/reference/semver.html) for [SemVer](https://semver.org/) compatibility. However, features prefixed with `unstable` are experimental and may introduce breaking changes between minor versions or even be completely removed.

## Any questions?

Feel free to open an issue on the [GitHub repository](https://github.com/ferreira-tb/tauri-store/issues) or ask in our [Discord server](https://discord.gg/ARd7McmVNv).
