<script lang="ts">
  import { Ext } from '$lib/components/link';
  import * as Alert from '$lib/components/alert';
  import { CodeGroup } from '$lib/components/code';
  import { currentMetadata } from '$lib/stores/plugin';
  import { Container } from '$lib/components/container';
  import { Breadcrumb } from '$lib/components/breadcrumb';
  import { Extra } from '$lib/content/guide/getting-started/extra';
  import { UsageSteps } from '$lib/content/guide/getting-started/usage-steps';
  import {
    capabilities,
    installCrate,
    installPackage,
    pluginRegistration,
  } from '$lib/content/guide/getting-started/snippets';

  const url = $derived.by(() => {
    return {
      capabilities: 'https://v2.tauri.app/security/capabilities/',
      javascriptDocs: $currentMetadata.docs.javascript!,
      rustDocs: $currentMetadata.docs.rust,
    };
  });
</script>

<svelte:head>
  <title>Getting started | tauri-store</title>
  <meta name="description" content="Getting started" />
</svelte:head>

<Breadcrumb current="Getting started" parents={['Guide']} />

<div>
  <Container title="Getting started" level={1}>
    <Alert.Root>
      <Alert.Title>Supported Tauri Version</Alert.Title>
      <Alert.Description>This plugin requires Tauri 2.0 or later.</Alert.Description>
    </Alert.Root>
  </Container>

  <Container title="Install">
    <p>
      Install the <Ext href={url.rustDocs}>Rust crate</Ext> by adding the following to your
      <code>Cargo.toml</code> file:
    </p>
    <CodeGroup code={$installCrate} />

    <p>
      Install the <Ext href={url.javascriptDocs}>JavaScript package</Ext> with your preferred package
      manager:
    </p>
    <CodeGroup code={$installPackage} />
  </Container>

  <Container title="Usage">
    <ol>
      <li>
        Enable the required permissions in your
        <Ext href={url.capabilities}>capabilities</Ext> file:

        <CodeGroup code={$capabilities} />
      </li>

      <li>
        <span>Register the plugin with Tauri:</span>

        <CodeGroup code={$pluginRegistration} />
      </li>

      <UsageSteps />
    </ol>
  </Container>

  <Extra />
</div>
