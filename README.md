# tauri-store

Persistent stores for Tauri.

## Features

- Save your stores to disk.
- Synchronize across multiple windows.
- Debounce or throttle store updates.
- Access the stores from both JavaScript and Rust.

## Documentation

Check the [documentation](https://tb.dev.br/tauri-store/) for more information on how to install and use the plugins.

## Plugins

Currently, the following plugins are available:

| Name                                                                                       | Version                                                                                                                 | Works with |
| ------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------- | ---------- |
| [tauri-store](https://tb.dev.br/tauri-store/guide/getting-started)                         | [![npm](https://img.shields.io/npm/v/tauri-store.svg)](https://www.npmjs.com/package/tauri-store)                       | Everything |
| [@tauri-store/pinia](https://tb.dev.br/tauri-store/plugin-pinia/guide/getting-started)     | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fpinia.svg)](https://www.npmjs.com/package/@tauri-store/pinia)     | Vue, Nuxt  |
| [@tauri-store/svelte](https://tb.dev.br/tauri-store/plugin-svelte/guide/getting-started)   | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fsvelte.svg)](https://www.npmjs.com/package/@tauri-store/svelte)   | Svelte     |
| [@tauri-store/valtio](https://tb.dev.br/tauri-store/plugin-valtio/guide/getting-started)   | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fvaltio.svg)](https://www.npmjs.com/package/@tauri-store/valtio)   | React      |
| [@tauri-store/zustand](https://tb.dev.br/tauri-store/plugin-zustand/guide/getting-started) | [![npm](https://img.shields.io/npm/v/%40tauri-store%2Fzustand.svg)](https://www.npmjs.com/package/@tauri-store/zustand) | React      |

## Examples

You can find example projects in the [examples directory](https://github.com/ferreira-tb/tauri-store/tree/main/examples). To check them out, run the following commands:

```sh
git clone https://github.com/ferreira-tb/tauri-store.git
cd tauri-store
pnpm install
pnpm run example <example-name>
```

To run the [`pinia`](https://github.com/ferreira-tb/tauri-store/tree/main/examples/pinia) example, you can use:

```sh
pnpm run example pinia
```

For a completely random example, run:

```sh
pnpm run example random
```

## Supported Tauri Version

The plugins require Tauri `2.0` or later.
