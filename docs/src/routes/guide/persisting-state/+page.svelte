<script lang="ts">
  import { Ext, Link } from '$components/link';
  import { Container } from '$components/container';
  import { Breadcrumb } from '$components/breadcrumb';
  import { CodeBlock, CodeGroup } from '$components/code';
  import { currentMetadata, currentPlugin } from '$stores/plugin';
  import {
    autosave,
    customDirectory,
    saveDenylist,
    saveOnChange,
    saveStores,
    setCollectionPath,
  } from '$content/guide/persisting-state/snippets';

  // prettier-ignore
  const url = $derived.by(() => {
    const docs = $currentMetadata.docs;
    return {
      // JavaScript
      saveOnChange: `${docs.javascript}/interfaces/StoreBackendOptions.html#saveonchange`,

      // Rust
      autosave: `${docs.rust}/struct.Builder.html#method.autosave`,
      path: `${docs.rust}/struct.Builder.html#method.path`,
      save_denylist: `${docs.rust}/struct.Builder.html#method.save_denylist`,

      // Tauri
      appDataDir: 'https://docs.rs/tauri/latest/tauri/path/struct.PathResolver.html#method.app_data_dir',
    };
  });
</script>

<svelte:head>
  <title>Persisting state | tauri-store</title>
  <meta name="description" content="Persisting store state" />
</svelte:head>

<Breadcrumb current="Persisting state" parents={['Guide']} />

{#snippet ext(key: keyof typeof url, label?: string, code = true)}
  <Ext href={url[key]} {code}>{label ?? key}</Ext>
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

  <CodeGroup code={$autosave} />
</Container>

<Container title="Custom directory">
  <p>
    By default, the stores are saved in a directory called <code>{$currentPlugin}</code> within your
    app's {@render ext('appDataDir', 'data directory')}. You can change this by setting the
    {@render ext('path')} option during the plugin's initialization.
  </p>

  <CodeGroup code={$customDirectory} />

  <p>
    The path can also be modified at runtime. In this case, all
    <em>currently active</em> stores will be moved to the new directory.
  </p>

  <CodeGroup code={$setCollectionPath} />
</Container>

<Container title="Denylist">
  <p>
    If a store should be <Link href="/tauri-store/guide/synchronization">synchronized</Link>, but
    not saved to disk, you can add it to the {@render ext('save_denylist', 'denylist', false)}.
  </p>

  <CodeGroup code={$saveDenylist} />
</Container>
