<script lang="ts">
  import { Breadcrumb } from '$components/breadcrumb';
  import { Changelog, Container } from '$components/container';
  import * as Svelte from '$content/changelog/tauri-plugin-svelte';
</script>

<svelte:head>
  <title>Changelog | tauri-store</title>
  <meta name="description" content="Changelog for tauri-plugin-svelte" />
</svelte:head>

<Breadcrumb current="tauri-plugin-svelte" parents={['Changelog']} />

<Container title="Changelog" level={1} />

<Changelog.Next>
  <ul>
    <li>Add <code>TauriPluginSvelteStoreOptions.saveOnExit</code>.</li>
  </ul>
</Changelog.Next>

<Svelte.V0x />
