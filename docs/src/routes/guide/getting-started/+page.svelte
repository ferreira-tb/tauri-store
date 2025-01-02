<script lang="ts">
  import { findMetadata } from '$lib/data';
  import * as Alert from '$lib/components/alert';
  import Link from '$lib/components/link.svelte';
  import Container from '$lib/components/container.svelte';
  import { currentPlugin, DEFAULT_PLUGIN } from '$lib/stores/plugin';

  const url = $derived.by(() => {
    const current = $currentPlugin ?? DEFAULT_PLUGIN;
    return {
      capabilities: 'https://v2.tauri.app/security/capabilities/',
      javascriptDocs: findMetadata(current).docs.javascript!,
      rustDocs: findMetadata(current).docs.rust,
    };
  });
</script>

<div>
  <Container title="Getting started" id="getting-started" level={1}>
    <Alert.Root>
      <Alert.Title>Supported Tauri Version</Alert.Title>
      <Alert.Description>This plugin requires Tauri 2.0 or later.</Alert.Description>
    </Alert.Root>
  </Container>

  <Container title="Install" id="install" level={2}>
    <p>
      Install the <Link href={url.rustDocs} external>Rust crate</Link> by adding the following to your
      <code>Cargo.toml</code> file:
    </p>
    <p>
      Install the <Link href={url.javascriptDocs} external>JavaScript package</Link> with your preferred
      package manager:
    </p>
  </Container>

  <Container title="Usage" id="usage" level={2}>
    <p>
      1. Enable the required permissions in your
      <Link href={url.capabilities} external>capabilities</Link> file:
    </p>
  </Container>
</div>
