# @tauri-store/valtio

Persistent [Valtio](https://valtio.dev/) stores for Tauri and React.

> [!NOTE]
> This is not an official plugin.

## Features

- Save your stores to disk.
- Synchronize across multiple windows.
- Debounce or throttle store updates.
- Access the stores from both JavaScript and Rust.

## Documentation

Check the [documentation](https://tb.dev.br/tauri-store/plugin-valtio/guide/getting-started) for more information on how to install and use the plugin.

## Plugins

Currently, the following plugins are available:

| Name                                                                                       | Version                                                                                                                 | Works with |
| ------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------- | ---------- |
| [tauri-store](https://tb.dev.br/tauri-store/guide/getting-started)                         | [![npm](https://img.shields.io/npm/v/tauri-store.svg)](https://www.npmjs.com/package/tauri-store)                       | Everything |
| [@tauri-store/pinia](https://tb.dev.br/tauri-store/plugin-pinia/guide/getting-started)     | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fpinia.svg)](https://www.npmjs.com/package/@tauri-store/pinia)     | Vue, Nuxt  |
| [@tauri-store/svelte](https://tb.dev.br/tauri-store/plugin-svelte/guide/getting-started)   | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fsvelte.svg)](https://www.npmjs.com/package/@tauri-store/svelte)   | Svelte     |
| [@tauri-store/valtio](https://tb.dev.br/tauri-store/plugin-valtio/guide/getting-started)   | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fvaltio.svg)](https://www.npmjs.com/package/@tauri-store/valtio)   | React      |
| [@tauri-store/zustand](https://tb.dev.br/tauri-store/plugin-zustand/guide/getting-started) | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fzustand.svg)](https://www.npmjs.com/package/@tauri-store/zustand) | React      |

## Supported Tauri Version

The plugins require Tauri `2.0` or later.
