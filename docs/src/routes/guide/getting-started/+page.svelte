<script lang="ts">
  import { findMetadata } from '$lib/data';
  import { Ext } from '$lib/components/link';
  import * as Alert from '$lib/components/alert';
  import { Container } from '$lib/components/container';
  import { CodeBlock, CodeGroup } from '$lib/components/code';
  import { currentPlugin, DEFAULT_PLUGIN } from '$lib/stores/plugin';
  import {
    capabilities,
    installCrate,
    installPackage,
  } from '$lib/content/guide/getting-started/snippets';

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
      Install the <Ext href={url.rustDocs}>Rust crate</Ext> by adding the following to your
      <code>Cargo.toml</code> file:
    </p>
    <CodeBlock lang="toml" code={$installCrate} />

    <p>
      Install the <Ext href={url.javascriptDocs}>JavaScript package</Ext> with your preferred package
      manager:
    </p>
    <CodeGroup code={$installPackage} />
  </Container>

  <Container title="Usage" id="usage" level={2}>
    <p>
      1. Enable the required permissions in your
      <Ext href={url.capabilities}>capabilities</Ext> file:
    </p>
    <CodeGroup code={$capabilities} />
  </Container>
</div>
