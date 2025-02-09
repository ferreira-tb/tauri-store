<script lang="ts">
  import { Ext } from '$components/link';
  import { currentMetadata } from '$stores/plugin';
  import { Container } from '$components/container';
  import { Breadcrumb } from '$components/breadcrumb';
  import { CodeBlock, CodeGroup } from '$components/code';
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
      beforeBackendSync: `${docs.javascript}/interfaces/StoreHooks.html#beforebackendsync`,
      beforeFrontendSync: `${docs.javascript}/interfaces/StoreHooks.html#beforefrontendsync`,
      hooks: `${docs.javascript}/interfaces/StoreFrontendOptions.html#hooks`,
      error: `${docs.javascript}/interfaces/StoreHooks.html#error`,

      // Rust
      on_load: `${docs.rust}/struct.Builder.html#method.on_load`,
    };
  });
</script>

<svelte:head>
  <title>Lifecycle hooks | tauri-store</title>
  <meta name="description" content="Lifecycle hooks" />
</svelte:head>

<Breadcrumb current="Lifecycle hooks" parents={['Guide']} />

{#snippet ext(key: keyof typeof url, label?: string, code = true)}
  <Ext href={url[key]} {code}>{label ?? key}</Ext>
{/snippet}

<Container title="Lifecycle hooks" level={1}></Container>

<Container title="JavaScript">
  <p>JavaScript hooks can be registered using the {@render ext('hooks')} option.</p>

  <CodeBlock lang="typescript" code={$jsHooks} />
</Container>

<Container title="beforeBackendSync" level={3}>
  {#snippet titleSnippet({ title })}
    <Ext href={url.beforeBackendSync}>{title}</Ext>
  {/snippet}

  <p>
    Registers a hook to be called before a store sends its state to Rust. This is useful for
    transforming the state before it is sent to the backend.
  </p>

  <CodeBlock lang="typescript" code={onBeforeBackendSync} />
</Container>

<Container title="beforeFrontendSync" level={3}>
  {#snippet titleSnippet({ title })}
    <Ext href={url.beforeFrontendSync}>{title}</Ext>
  {/snippet}

  <p>
    Registers a hook to be called before a store attempts to update itself with data from Rust. This
    can be used to modify the state before the changes are applied.
  </p>

  <CodeBlock lang="typescript" code={onBeforeFrontendSync} />
</Container>

<Container title="error" level={3}>
  {#snippet titleSnippet({ title })}
    <Ext href={url.error}>{title}</Ext>
  {/snippet}

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
