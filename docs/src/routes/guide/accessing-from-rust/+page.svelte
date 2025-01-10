<script lang="ts">
  import { pascalCase } from 'change-case';
  import { Ext } from '$lib/components/link';
  import { currentMetadata } from '$lib/stores/plugin';
  import { Container } from '$lib/components/container';
  import { Breadcrumb } from '$lib/components/breadcrumb';
  import { CodeBlock, CodeGroup } from '$lib/components/code';
  import { get, onLoad, tryGet, watchStore } from '$lib/content/guide/accessing-from-rust/snippets';

  const url = $derived.by(() => {
    const docs = $currentMetadata.docs;
    const title = $currentMetadata.title ?? '';
    const pascalTitle = pascalCase(title);
    return {
      // Rust
      ManagerExt: `${docs.rust}/trait.ManagerExt.html`,
      Store: `${docs.rust}/struct.Store.html`,
      on_load: `${docs.rust}/struct.Builder.html#method.on_load`,
      try_get: `${docs.rust}/struct.${pascalTitle}.html#method.try_get`,
      watch: `${docs.rust}/struct.${pascalTitle}.html#method.watch`,

      // Tauri
      AppHandle: 'https://docs.rs/tauri/latest/tauri/struct.AppHandle.html',
      Manager: 'https://docs.rs/tauri/latest/tauri/trait.Manager.html',
      WebviewWindow: 'https://docs.rs/tauri/latest/tauri/window/struct.Window.html',
      Window: 'https://docs.rs/tauri/latest/tauri/window/struct.Window.html',

      // Serde
      serde_json: 'https://docs.rs/serde_json/latest/serde_json/',
      'serde_json::Value': 'https://docs.rs/serde_json/latest/serde_json/enum.Value.html',
    };
  });
</script>

<svelte:head>
  <title>Accessing from Rust | tauri-store</title>
  <meta name="description" content="Accessing stores from Rust" />
</svelte:head>

<Breadcrumb current="Accessing from Rust" parents={['Guide']} />

{#snippet ext(key: keyof typeof url)}
  <Ext href={url[key]} code>{key}</Ext>
{/snippet}

<Container title="Accessing from Rust" level={1}>
  <p>
    When the {@render ext('ManagerExt')} trait is in scope, you can access your stores from any type
    that implements the {@render ext('Manager')} trait (e.g. {@render ext('AppHandle')},
    {@render ext('Window')}, {@render ext('WebviewWindow')}).
  </p>

  <p>
    Note that all values are stored as {@render ext('serde_json::Value')}, so you may need to
    convert them to the desired type when accessing from Rust. You can check the
    <Ext href={url.serde_json}>serde_json documentation</Ext> for more information.
  </p>

  <p>
    A list of all available methods for the stores can be found <Ext href={url.Store}>here</Ext>.
  </p>

  <CodeBlock lang="rust" code={$get} />

  <p>
    You can also use the {@render ext('try_get')} method to get the value directly as the desired type.
  </p>

  <CodeBlock lang="rust" code={$tryGet} />
</Container>

<Container title="Watching for changes">
  <p>
    The {@render ext('watch')} method can be used to set up a closure that will be called whenever the
    state of the store changes.
  </p>

  <CodeBlock lang="rust" code={$watchStore} />
</Container>

<Container title="Lifecycle hooks" />

<Container title="on_load" level={3}>
  {#snippet titleSnippet({ title })}
    <Ext href={url.on_load}>{title}</Ext>
  {/snippet}

  <p>Registers a closure to be called when a store is loaded.</p>

  <CodeGroup code={$onLoad} />
</Container>
