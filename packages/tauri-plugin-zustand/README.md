# tauri-plugin-zustand

> **IMPORTANT**: This is work in progress and **SHOULD NOT** be used before it reaches version 0.1.0. Expect breaking changes on every release before that. Tracking issue: [ferreira-tb/tauri-store#106](https://github.com/ferreira-tb/tauri-store/issues/106).

Persistent [Zustand](https://zustand.docs.pmnd.rs/getting-started/introduction) stores for Tauri and React.

## Features

- Save your stores to disk.
- Synchronize across multiple windows.
- Debounce or throttle store updates.
- Access the stores from both JavaScript and Rust.

## Documentation

Check the [documentation](https://tb.dev.br/tauri-store/guide/getting-started?plugin=tauri-plugin-zustand) for more information on how to install and use the plugin.

## Plugins

Currently, the following plugins are available:

| Name                                                                                                  | Version                                                                                                           | Works with |
| ----------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ---------- |
| [tauri-store](https://tb.dev.br/tauri-store/guide/getting-started?plugin=tauri-store)                 | [![npm](https://img.shields.io/npm/v/tauri-store.svg)](https://www.npmjs.com/package/tauri-store)                 | Everything |
| [tauri-plugin-pinia](https://tb.dev.br/tauri-store/guide/getting-started?plugin=tauri-plugin-pinia)   | [![npm](https://img.shields.io/npm/v/tauri-plugin-pinia.svg)](https://www.npmjs.com/package/tauri-plugin-pinia)   | Vue, Nuxt  |
| [tauri-plugin-svelte](https://tb.dev.br/tauri-store/guide/getting-started?plugin=tauri-plugin-svelte) | [![npm](https://img.shields.io/npm/v/tauri-plugin-svelte.svg)](https://www.npmjs.com/package/tauri-plugin-svelte) | Svelte     |
| [tauri-plugin-valtio](https://tb.dev.br/tauri-store/guide/getting-started?plugin=tauri-plugin-valtio) | [![npm](https://img.shields.io/npm/v/tauri-plugin-valtio.svg)](https://www.npmjs.com/package/tauri-plugin-valtio) | React      |

## Supported Tauri Version

The plugins require Tauri `2.0` or later.
