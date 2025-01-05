<script lang="ts">
  import { Ext } from '$lib/components/link';
  import { currentMetadata } from '$lib/stores/plugin';
  import { Container } from '$lib/components/container';
  import { CodeBlock, CodeGroup } from '$lib/components/code';
  import {
    autosave,
    customDirectory,
    saveOnChange,
    saveStores,
  } from '$lib/content/guide/persisting-state/snippets';

  const url = $derived.by(() => {
    const docs = $currentMetadata.docs;
    return {
      autosave: `${docs.rust}/struct.Builder.html#method.autosave`,
      path: `${docs.rust}/struct.Builder.html#method.path`,
      saveOnChange: `${docs.javascript}/interfaces/StoreBackendOptions.html#saveonchange`,

      appDataDir:
        'https://docs.rs/tauri/latest/tauri/path/struct.PathResolver.html#method.app_data_dir',
    };
  });
</script>

<svelte:head>
  <title>Persisting state | tauri-store</title>
  <meta name="description" content="Persisting store state" />
</svelte:head>

{#snippet ext(key: keyof typeof url, label?: string)}
  <Ext href={url[key]} code>{label ?? key}</Ext>
{/snippet}

<Container title="Persisting state" level={1}>
  <p>
    All your stores are automatically persisted to disk upon a graceful exit, but you can also
    manually save them if needed:
  </p>

  <CodeGroup code={$saveStores} />
</Container>

<Container title="Save on change">
  <p>
    If there's a need to save a store whenever its state changes, you can enable the
    {@render ext('saveOnChange')} option when defining the store.
  </p>

  <CodeBlock lang="typescript" code={$saveOnChange} />
</Container>

<Container title="Autosave">
  <p>You can also enable {@render ext('autosave')} to periodically write the stores to disk.</p>

  <CodeBlock lang="rust" code={$autosave} />
</Container>

<Container title="Custom directory">
  <p>
    By default, the stores are saved in the {@render ext('appDataDir', "app's data directory")}. You
    can change this by setting the {@render ext('path')} option during the plugin's initialization.
  </p>

  <CodeBlock lang="rust" code={$customDirectory} />
</Container>
