<script lang="ts">
  import { Ext } from '$components/link';
  import { Container } from '$components/container';
  import { Breadcrumb } from '$components/breadcrumb';
  import { CodeBlock, CodeGroup } from '$components/code';
  import { currentMetadata, currentPlugin } from '$stores/plugin';
  import {
    jsHooks,
    onBeforeBackendSync,
    onBeforeFrontendSync,
    onError,
    onLoad,
  } from '$content/guide/lifecycle-hooks/snippets';

  const url = $derived.by(() => {
    const docs = $currentMetadata.docs;
    return {
      // JavaScript
      StoreHooks: `${docs.javascript}/types/StoreHooks.html`,

      // Rust
      on_load: `${docs.rust}/struct.Builder.html#method.on_load`,
    };
  });

  const staticUrl = {
    structuredClone: 'https://developer.mozilla.org/en-US/docs/Web/API/Window/structuredClone',
    valtioDeepClone: 'https://valtio.dev/docs/how-tos/how-to-reset-state',
    valtioSnapshot: 'https://valtio.dev/docs/api/advanced/snapshot',
  };
</script>

<svelte:head>
  <title>Lifecycle hooks | tauri-store</title>
  <meta name="description" content="Lifecycle hooks" />
</svelte:head>

<Breadcrumb current="Lifecycle hooks" parents={['Guide']} />

{#snippet ext(key: keyof typeof url, label?: string, code = true)}
  <Ext href={url[key]} {code}>{label ?? key}</Ext>
{/snippet}

{#snippet extStatic(key: keyof typeof staticUrl, label?: string, code = true)}
  <Ext href={staticUrl[key]} {code}>{label ?? key}</Ext>
{/snippet}

<Container title="Lifecycle hooks" level={1}></Container>

<Container title="JavaScript">
  <p>JavaScript hooks can be registered using the {@render ext('StoreHooks', 'hooks')} option.</p>

  <CodeBlock lang="typescript" code={$jsHooks} />
</Container>

<Container title="beforeBackendSync" level={3}>
  <p>
    Registers a hook to be called before a store sends its state to Rust. This can be used to modify
    the state before it is sent to the backend.
  </p>

  <CodeBlock lang="typescript" code={onBeforeBackendSync} />

  {#if $currentPlugin === 'tauri-plugin-valtio'}
    <p>
      <code>state</code> is a <Ext href={staticUrl.valtioSnapshot}>snapshot</Ext>, so it's deeply
      frozen. If you need to mutate it, you can use
      {@render extStatic('valtioDeepClone', 'deepClone')} from <code>valtio/utils</code>
      or {@render extStatic('structuredClone')} to create a new object.
    </p>
  {/if}
</Container>

<Container title="beforeFrontendSync" level={3}>
  <p>
    Registers a hook to be called before a store attempts to update itself with data from Rust. This
    can be used to modify the state before the changes are applied.
  </p>

  <CodeBlock lang="typescript" code={onBeforeFrontendSync} />
</Container>

<Container title="error" level={3}>
  <p>Registers a hook to be called when an error is thrown by a store.</p>

  <CodeBlock lang="typescript" code={onError} />
</Container>

<Container title="Rust" />

<Container title="on_load" level={3}>
  {#snippet titleSnippet({ title })}
    <Ext href={url.on_load}>{title}</Ext>
  {/snippet}

  <p>Registers a hook to be called when a store is loaded.</p>

  <CodeGroup code={$onLoad} />
</Container>
