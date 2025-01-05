<script lang="ts">
  import { Changelog, Container } from '$lib/components/container';
  import * as Svelte from '$lib/content/changelog/tauri-plugin-svelte';
</script>

<svelte:head>
  <title>Changelog | tauri-store</title>
  <meta name="description" content="Changelog for tauri-plugin-svelte" />
</svelte:head>

<Container title="Changelog" level={1} />

<Changelog.Next />

<Svelte.V0x />
