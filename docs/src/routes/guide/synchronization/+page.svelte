<script lang="ts">
  import * as Alert from '$lib/components/alert';
  import { Ext, Link } from '$lib/components/link';
  import { currentMetadata } from '$lib/stores/plugin';
  import { Container } from '$lib/components/container';
  import { Breadcrumb } from '$lib/components/breadcrumb';
  import { CodeBlock, CodeGroup } from '$lib/components/code';
  import { syncDenylist, syncOptions } from '$lib/content/guide/synchronization/snippets';

  const url = $derived.by(() => {
    const docs = $currentMetadata.docs;
    return {
      // Rust
      sync_denylist: `${docs.rust}/struct.Builder.html#method.sync_denylist`,
    };
  });
</script>

<svelte:head>
  <title>Synchronization | tauri-store</title>
  <meta name="description" content="Synchronization" />
</svelte:head>

<Breadcrumb current="Synchronization" parents={['Guide']} />

{#snippet ext(key: keyof typeof url, label?: string, code = true)}
  <Ext href={url[key]} {code}>{label ?? key}</Ext>
{/snippet}

<Container title="Synchronization" level={1}>
  <p>
    Whenever the state changes, the store notifies Rust to keep the frontend and backend in sync.
    However, since data is serialized with each notification, frequent updates can be
    resource-intensive. One way to address this issue is by applying debouncing or throttling,
    making the synchronization process more efficient.
  </p>

  <CodeBlock lang="typescript" code={$syncOptions} />

  <Alert.Root>
    <Alert.Title>Debounce or throttle?</Alert.Title>
    <Alert.Description>
      For a detailed explanation of the differences between debouncing and throttling, take a look
      at <Ext href="https://kettanaito.com/blog/debounce-vs-throttle">this article</Ext>.
    </Alert.Description>
  </Alert.Root>

  <p>
    While this process isn’t directly related to
    <Link href="/tauri-store/guide/persisting-state">store persistence</Link>, it can still affect
    what gets saved. When a store is saved, the data written to disk comes from Rust’s cache at that
    moment. If the synchronization hasn’t finished yet, Rust might still be working with outdated
    values.
  </p>
</Container>

<Container title="Denylist">
  <p>
    If a store should be <Link href="/tauri-store/guide/persisting-state">saved to disk</Link>, but
    not synchronized across windows, you can add it to the
    {@render ext('sync_denylist', 'denylist', false)}.
  </p>

  <CodeGroup code={$syncDenylist} />
</Container>
