<script setup lang="ts">
import { useSelectedPlugin } from '@/composables/plugin';

const plugin = useSelectedPlugin();
</script>

# Getting started

::: info Supported Tauri Version
This plugin requires Tauri `2.0` or later.
:::

## Install

Install the Rust crate by adding the following to your `Cargo.toml` file:

```toml-vue
[dependencies]
{{ `${plugin.name} = ${plugin.shortVersion}` }}
```

Install the JavaScript package with your preferred package manager:

::: code-group

```sh-vue [npm]
npm install {{ plugin.name }}
```

```sh-vue [pnpm]
pnpm add {{ plugin.name }}
```

```sh-vue [yarn]
yarn add {{ plugin.name }}
```

```sh-vue [bun]
bun add {{ plugin.name }}
```

:::

## Usage

1. Enable the required permissions in your capabilities file:

::: code-group

```json-vue{4} [src-tauri/capabilities/{{ plugin.shortName }}.json]
{
  "identifier": "{{ plugin.shortName }}",
  "windows": ["*"],
  "permissions": ["{{ plugin.shortName }}:default", "core:event:default"]
}
```

:::

2. Register the plugin with Tauri:

::: code-group

```rust-vue{2} [src-tauri/src/main.rs]
tauri::Builder::default()
  .plugin({{ plugin.snakeName }}::init())
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
```

:::

<div class="tauri-plugin-pinia">

<!--@include: ../examples/getting-started/pinia.md-->

</div>
